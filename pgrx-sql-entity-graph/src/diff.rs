//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Schema snapshot comparison and diff infrastructure.
//!
//! This module provides functionality for comparing two schema snapshots
//! and detecting additions, removals, modifications, and renames.

use crate::SqlGraphIdentifier;
use crate::snapshot::{EntityId, SchemaSnapshot, SerializableEntity};
use crate::snapshot_types::SqlGraphEntitySnapshot;
use std::collections::{HashMap, HashSet};

/// The result of comparing two schema snapshots.
#[derive(Debug, Clone)]
pub struct SchemaDiff {
    /// The old (baseline) snapshot version
    pub from_version: String,
    /// The new (target) snapshot version
    pub to_version: String,
    /// Entities that exist in the new snapshot but not the old
    pub added: Vec<EntityChange>,
    /// Entities that exist in the old snapshot but not the new
    pub removed: Vec<EntityChange>,
    /// Entities that exist in both but have different signatures
    pub modified: Vec<EntityModification>,
    /// Entities that appear to have been renamed (based on signature matching)
    pub renamed: Vec<EntityRename>,
}

/// A single entity that was added or removed.
#[derive(Debug, Clone)]
pub struct EntityChange {
    /// The entity ID
    pub id: EntityId,
    /// The entity data (snapshot version without TypeId)
    pub entity: SqlGraphEntitySnapshot,
    /// Whether this entity depends on other changes
    pub dependencies: Vec<String>, // rust_identifiers of dependencies
}

/// An entity that was modified between versions.
#[derive(Debug, Clone)]
pub struct EntityModification {
    /// The entity's identifier
    pub id: EntityId,
    /// The old version of the entity
    pub old_entity: SqlGraphEntitySnapshot,
    /// The new version of the entity
    pub new_entity: SqlGraphEntitySnapshot,
    /// Description of what changed
    pub changes: Vec<String>,
}

/// An entity that appears to have been renamed.
#[derive(Debug, Clone)]
pub struct EntityRename {
    /// The old entity identifier
    pub old_id: EntityId,
    /// The new entity identifier
    pub new_id: EntityId,
    /// The old entity data
    pub old_entity: SqlGraphEntitySnapshot,
    /// The new entity data
    pub new_entity: SqlGraphEntitySnapshot,
    /// Confidence score (0.0-1.0) that this is actually a rename
    pub confidence: f64,
}

impl SchemaDiff {
    /// Compare two schema snapshots and produce a diff.
    pub fn compare(old: &SchemaSnapshot, new: &SchemaSnapshot) -> Self {
        let mut diff = SchemaDiff {
            from_version: old.version.clone(),
            to_version: new.version.clone(),
            added: Vec::new(),
            removed: Vec::new(),
            modified: Vec::new(),
            renamed: Vec::new(),
        };

        // Build indices for fast lookup
        let new_index = new.build_entity_index();

        // Track which entities we've already processed
        let mut processed_old: HashSet<String> = HashSet::new();
        let mut processed_new: HashSet<String> = HashSet::new();

        // First pass: detect exact matches and modifications
        for old_entity in &old.entities {
            let old_identifier = &old_entity.id.rust_identifier;

            if let Some(new_entity) = new_index.get(old_identifier) {
                // Entity exists in both snapshots
                if old_entity.id.signature_hash != new_entity.id.signature_hash {
                    // Entity was modified
                    let changes = Self::describe_changes(&old_entity.entity, &new_entity.entity);
                    diff.modified.push(EntityModification {
                        id: new_entity.id.clone(),
                        old_entity: old_entity.entity.clone(),
                        new_entity: new_entity.entity.clone(),
                        changes,
                    });
                }
                processed_old.insert(old_identifier.clone());
                processed_new.insert(old_identifier.clone());
            }
        }

        // Second pass: detect potential renames using signature matching
        let unmatched_old: Vec<&SerializableEntity> = old
            .entities
            .iter()
            .filter(|e| !processed_old.contains(&e.id.rust_identifier))
            .collect();

        let unmatched_new: Vec<&SerializableEntity> = new
            .entities
            .iter()
            .filter(|e| !processed_new.contains(&e.id.rust_identifier))
            .collect();

        // Try to match by signature hash (fuzzy matching for renames)
        for old_entity in &unmatched_old {
            for new_entity in &unmatched_new {
                // Can only rename within the same entity type
                if old_entity.id.entity_type != new_entity.id.entity_type {
                    continue;
                }

                // Skip if already processed
                if processed_new.contains(&new_entity.id.rust_identifier) {
                    continue;
                }

                // Calculate similarity
                let confidence = Self::calculate_rename_confidence(old_entity, new_entity);

                if confidence > 0.7 {
                    // Likely a rename
                    diff.renamed.push(EntityRename {
                        old_id: old_entity.id.clone(),
                        new_id: new_entity.id.clone(),
                        old_entity: old_entity.entity.clone(),
                        new_entity: new_entity.entity.clone(),
                        confidence,
                    });

                    processed_old.insert(old_entity.id.rust_identifier.clone());
                    processed_new.insert(new_entity.id.rust_identifier.clone());
                    break; // Found a match, move to next old_entity
                }
            }
        }

        // Third pass: remaining entities are pure additions/removals
        for old_entity in &old.entities {
            if !processed_old.contains(&old_entity.id.rust_identifier) {
                diff.removed.push(EntityChange {
                    id: old_entity.id.clone(),
                    entity: old_entity.entity.clone(),
                    dependencies: Self::extract_dependencies(&old_entity.entity, old),
                });
            }
        }

        for new_entity in &new.entities {
            if !processed_new.contains(&new_entity.id.rust_identifier) {
                diff.added.push(EntityChange {
                    id: new_entity.id.clone(),
                    entity: new_entity.entity.clone(),
                    dependencies: Self::extract_dependencies(&new_entity.entity, new),
                });
            }
        }

        diff
    }

    /// Calculate confidence (0.0-1.0) that two entities represent a rename.
    fn calculate_rename_confidence(old: &SerializableEntity, new: &SerializableEntity) -> f64 {
        let mut score = 0.0;
        let mut max_score = 0.0;

        // Same type is required (already filtered)
        max_score += 1.0;
        score += 1.0;

        // Same signature hash is strong evidence
        max_score += 3.0;
        if old.id.signature_hash == new.id.signature_hash {
            score += 3.0;
        }

        // Similar file location
        max_score += 1.0;
        if let (Some(old_file), Some(new_file)) = (old.entity.file(), new.entity.file()) {
            if old_file == new_file {
                score += 1.0;
            } else if old_file.contains(new_file) || new_file.contains(old_file) {
                score += 0.5;
            }
        }

        // Similar module path
        max_score += 1.0;
        let old_parts: Vec<&str> = old.id.rust_identifier.split("::").collect();
        let new_parts: Vec<&str> = new.id.rust_identifier.split("::").collect();

        if old_parts.len() > 1 && new_parts.len() > 1 {
            // Compare module path (all but last segment)
            let old_module = &old_parts[..old_parts.len() - 1];
            let new_module = &new_parts[..new_parts.len() - 1];

            let common =
                old_module.iter().zip(new_module.iter()).take_while(|(a, b)| a == b).count();
            let max_len = old_module.len().max(new_module.len()) as f64;

            if max_len > 0.0 {
                score += (common as f64) / max_len;
            }
        }

        score / max_score
    }

    /// Describe what changed between two versions of an entity.
    fn describe_changes(old: &SqlGraphEntitySnapshot, new: &SqlGraphEntitySnapshot) -> Vec<String> {
        let mut changes = Vec::new();

        // This is a simplified version - in reality, we'd do deep comparisons
        // For now, just note that the signature changed
        let old_hash = {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            old.hash(&mut hasher);
            format!("{:x}", hasher.finish())
        };

        let new_hash = {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            new.hash(&mut hasher);
            format!("{:x}", hasher.finish())
        };

        if old_hash != new_hash {
            changes.push("Signature changed".to_string());
        }

        // Add more specific change detection based on entity type
        match (old, new) {
            (
                SqlGraphEntitySnapshot::Function(old_fn),
                SqlGraphEntitySnapshot::Function(new_fn),
            ) => {
                if old_fn.name != new_fn.name {
                    changes.push(format!("Name changed: {} -> {}", old_fn.name, new_fn.name));
                }
            }
            (SqlGraphEntitySnapshot::Type(old_type), SqlGraphEntitySnapshot::Type(new_type)) => {
                if old_type.name != new_type.name {
                    changes
                        .push(format!("Type name changed: {} -> {}", old_type.name, new_type.name));
                }
            }
            (SqlGraphEntitySnapshot::Enum(old_enum), SqlGraphEntitySnapshot::Enum(new_enum)) => {
                if old_enum.name != new_enum.name {
                    changes
                        .push(format!("Enum name changed: {} -> {}", old_enum.name, new_enum.name));
                }
                if old_enum.variants != new_enum.variants {
                    changes.push("Enum variants changed".to_string());
                }
            }
            _ => {}
        }

        if changes.is_empty() {
            changes.push("Unknown change".to_string());
        }

        changes
    }

    /// Extract dependencies for an entity from the snapshot.
    fn extract_dependencies(
        entity: &SqlGraphEntitySnapshot,
        snapshot: &SchemaSnapshot,
    ) -> Vec<String> {
        let mut deps = Vec::new();

        // Find edges in the dependency graph where this entity is the source
        let entity_identifier = entity.rust_identifier();

        for edge in &snapshot.dependency_edges {
            if edge.from == entity_identifier {
                deps.push(edge.to.clone());
            }
        }

        deps
    }

    /// Check if the diff contains any breaking changes.
    pub fn has_breaking_changes(&self) -> bool {
        // Removals are always breaking
        if !self.removed.is_empty() {
            return true;
        }

        // Modifications to functions or types are potentially breaking
        for modification in &self.modified {
            match &modification.new_entity {
                SqlGraphEntitySnapshot::Function(_)
                | SqlGraphEntitySnapshot::Type(_)
                | SqlGraphEntitySnapshot::Enum(_) => {
                    return true;
                }
                _ => {}
            }
        }

        false
    }

    /// Get a summary of the diff.
    pub fn summary(&self) -> String {
        format!(
            "Schema changes from {} to {}: {} added, {} removed, {} modified, {} renamed",
            self.from_version,
            self.to_version,
            self.added.len(),
            self.removed.len(),
            self.modified.len(),
            self.renamed.len()
        )
    }

    /// Get entities in dependency order for additions.
    /// This ensures that dependencies are created before the entities that depend on them.
    pub fn additions_in_dependency_order(&self) -> Vec<&EntityChange> {
        let mut result = Vec::new();
        let mut added_identifiers = HashSet::new();

        // Build a map of identifiers to EntityChange
        let mut entity_map: HashMap<String, &EntityChange> = HashMap::new();
        for change in &self.added {
            entity_map.insert(change.id.rust_identifier.clone(), change);
        }

        // Topological sort
        fn visit<'a>(
            identifier: &str,
            entity_map: &HashMap<String, &'a EntityChange>,
            added_identifiers: &mut HashSet<String>,
            result: &mut Vec<&'a EntityChange>,
        ) {
            if added_identifiers.contains(identifier) {
                return;
            }

            if let Some(change) = entity_map.get(identifier) {
                // Visit dependencies first
                for dep in &change.dependencies {
                    visit(dep, entity_map, added_identifiers, result);
                }

                added_identifiers.insert(identifier.to_string());
                result.push(change);
            }
        }

        // Visit all entities
        for identifier in entity_map.keys() {
            visit(identifier, &entity_map, &mut added_identifiers, &mut result);
        }

        result
    }

    /// Get entities in reverse dependency order for removals.
    /// This ensures that entities are dropped before their dependencies.
    pub fn removals_in_dependency_order(&self) -> Vec<&EntityChange> {
        let mut result = Vec::new();
        let mut removed_identifiers = HashSet::new();

        // Build a map of identifiers to EntityChange
        let mut entity_map: HashMap<String, &EntityChange> = HashMap::new();
        for change in &self.removed {
            entity_map.insert(change.id.rust_identifier.clone(), change);
        }

        // Reverse topological sort (process dependents before dependencies)
        fn visit<'a>(
            identifier: &str,
            entity_map: &HashMap<String, &'a EntityChange>,
            all_changes: &[EntityChange],
            removed_identifiers: &mut HashSet<String>,
            result: &mut Vec<&'a EntityChange>,
        ) {
            if removed_identifiers.contains(identifier) {
                return;
            }

            // Find entities that depend on this one
            for change in all_changes {
                if change.dependencies.contains(&identifier.to_string()) {
                    visit(
                        &change.id.rust_identifier,
                        entity_map,
                        all_changes,
                        removed_identifiers,
                        result,
                    );
                }
            }

            if let Some(change) = entity_map.get(identifier) {
                removed_identifiers.insert(identifier.to_string());
                result.push(change);
            }
        }

        // Visit all entities
        for identifier in entity_map.keys() {
            visit(identifier, &entity_map, &self.removed, &mut removed_identifiers, &mut result);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SqlGraphEntity;
    use crate::schema::entity::SchemaEntity;
    use crate::snapshot_types::SqlGraphEntitySnapshot;
    use std::time::SystemTime;

    #[test]
    fn test_empty_diff() {
        let snapshot1 = SchemaSnapshot {
            version: "1.0.0".to_string(),
            timestamp: SystemTime::now(),
            extension_name: "test".to_string(),
            entities: vec![],
            dependency_edges: vec![],
            versioned_so: false,
        };

        let snapshot2 = snapshot1.clone();

        let diff = SchemaDiff::compare(&snapshot1, &snapshot2);

        assert_eq!(diff.added.len(), 0);
        assert_eq!(diff.removed.len(), 0);
        assert_eq!(diff.modified.len(), 0);
        assert_eq!(diff.renamed.len(), 0);
    }

    #[test]
    fn test_addition() {
        let snapshot1 = SchemaSnapshot {
            version: "1.0.0".to_string(),
            timestamp: SystemTime::now(),
            extension_name: "test".to_string(),
            entities: vec![],
            dependency_edges: vec![],
            versioned_so: false,
        };

        let schema_entity = SchemaEntity {
            module_path: "test::schema",
            name: "test_schema",
            file: "src/lib.rs",
            line: 10,
        };

        let mut snapshot2 = snapshot1.clone();
        snapshot2.version = "1.1.0".to_string();
        snapshot2.entities.push(SerializableEntity {
            id: EntityId::from_entity(&SqlGraphEntity::Schema(schema_entity.clone())),
            entity: SqlGraphEntitySnapshot::Schema(schema_entity),
            node_index: Some(0),
        });

        let diff = SchemaDiff::compare(&snapshot1, &snapshot2);

        assert_eq!(diff.added.len(), 1);
        assert_eq!(diff.removed.len(), 0);
        assert_eq!(diff.modified.len(), 0);
    }
}
