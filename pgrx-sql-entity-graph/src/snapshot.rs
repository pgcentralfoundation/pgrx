//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Schema snapshot infrastructure for automatic upgrade script generation.
//!
//! This module provides data structures and functionality for:
//! - Capturing the current state of a pgrx extension schema
//! - Serializing/deserializing schema snapshots to/from JSON
//! - Comparing snapshots to detect changes between versions

use crate::pgrx_sql::SqlGraphRequires;
use crate::snapshot_types::SqlGraphEntitySnapshot;
use crate::{PgrxSql, SqlGraphEntity};
use petgraph::visit::{EdgeRef, IntoEdgeReferences};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

/// A snapshot of an extension's schema at a specific version.
///
/// This captures all SQL entities and their dependencies,
/// allowing for comparison between different versions to
/// generate upgrade scripts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct SchemaSnapshot {
    /// The extension version this snapshot represents
    pub version: String,

    /// When this snapshot was created
    #[serde(with = "systemtime_serde")]
    pub timestamp: SystemTime,

    /// The extension name
    pub extension_name: String,

    /// All entities in the schema
    pub entities: Vec<SerializableEntity>,

    /// Dependency edges between entities
    pub dependency_edges: Vec<DependencyEdge>,

    /// Whether the extension uses versioned shared objects
    pub versioned_so: bool,
}

/// A single entity with its identifier and SQL graph data.
///
/// Note: This uses [`SqlGraphEntitySnapshot`] which excludes TypeId fields,
/// as TypeId values are not stable across compilations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct SerializableEntity {
    /// Unique identifier for this entity
    pub id: EntityId,

    /// The entity data (snapshot version without TypeId)
    pub entity: SqlGraphEntitySnapshot,

    /// Node index in the original graph (for debugging)
    #[serde(skip)]
    pub node_index: Option<usize>,
}

/// Unique identifier for an entity
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityId {
    /// The type of entity (Function, Type, Enum, etc.)
    pub entity_type: EntityType,

    /// The Rust identifier (module::path::name)
    pub rust_identifier: String,

    /// Hash of the entity's signature (for fuzzy matching during renames)
    pub signature_hash: String,
}

/// Entity type discriminator
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    ExtensionRoot,
    Schema,
    CustomSql,
    Function,
    Type,
    BuiltinType,
    Enum,
    Ord,
    Hash,
    Aggregate,
    Trigger,
}

/// A dependency edge between two entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyEdge {
    /// The depending entity (source)
    pub from: String, // rust_identifier

    /// The dependency target
    pub to: String, // rust_identifier

    /// The type of dependency
    pub requires: SqlGraphRequires,
}

/// Deserialize a SchemaSnapshot from a JSON string.
///
/// This is a helper function to avoid exposing serde_json directly in generated code.
/// Note: The JSON string must have a 'static lifetime because SchemaSnapshot contains
/// &'static str fields that will be leaked from the deserialized strings.
pub fn deserialize_snapshot(json: &'static str) -> Result<SchemaSnapshot, serde_json::Error> {
    serde_json::from_str(json)
}

impl SchemaSnapshot {
    /// Create a snapshot from a PgrxSql graph
    pub fn from_pgrx_sql(pgrx_sql: &PgrxSql) -> Self {
        let mut entities = Vec::new();
        let mut dependency_edges = Vec::new();

        // Collect all entities from the graph, converting to snapshot versions
        for node_idx in pgrx_sql.graph.node_indices() {
            let entity = &pgrx_sql.graph[node_idx];
            let id = EntityId::from_entity(entity);

            entities.push(SerializableEntity {
                id,
                entity: SqlGraphEntitySnapshot::from(entity),
                node_index: Some(node_idx.index()),
            });
        }

        // Collect all dependency edges
        for edge in pgrx_sql.graph.edge_references() {
            let from_entity = &pgrx_sql.graph[edge.source()];
            let to_entity = &pgrx_sql.graph[edge.target()];
            let from_id = EntityId::from_entity(from_entity);
            let to_id = EntityId::from_entity(to_entity);

            dependency_edges.push(DependencyEdge {
                from: from_id.rust_identifier.clone(),
                to: to_id.rust_identifier.clone(),
                requires: *edge.weight(),
            });
        }

        SchemaSnapshot {
            version: pgrx_sql.control.default_version.clone(),
            timestamp: SystemTime::now(),
            extension_name: pgrx_sql.extension_name.clone(),
            entities,
            dependency_edges,
            versioned_so: pgrx_sql.versioned_so,
        }
    }

    /// Save the snapshot to a JSON file
    pub fn save(&self, path: &Path) -> eyre::Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Load a snapshot from a JSON file
    ///
    /// Note: This currently loads as JSON Value for comparison purposes.
    /// Full deserialization with proper lifetimes will be implemented in Phase 3.
    pub fn load_json(path: &Path) -> eyre::Result<serde_json::Value> {
        let json = fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;
        Ok(value)
    }

    /// Get an entity by its identifier
    pub fn get_entity(&self, id: &EntityId) -> Option<&SerializableEntity> {
        self.entities.iter().find(|e| &e.id == id)
    }

    /// Build an index of entities by rust_identifier for fast lookup
    pub fn build_entity_index(&self) -> HashMap<String, &SerializableEntity> {
        self.entities.iter().map(|e| (e.id.rust_identifier.clone(), e)).collect()
    }
}

impl EntityId {
    /// Create an EntityId from a SqlGraphEntity
    pub fn from_entity(entity: &SqlGraphEntity) -> Self {
        use crate::SqlGraphIdentifier;

        let entity_type = EntityType::from_entity(entity);
        let rust_identifier = entity.rust_identifier();
        let signature_hash = Self::compute_signature_hash(entity);

        EntityId { entity_type, rust_identifier, signature_hash }
    }

    /// Compute a hash of the entity's signature for fuzzy matching
    fn compute_signature_hash(entity: &SqlGraphEntity) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();

        // Hash the entity itself (it implements Hash)
        entity.hash(&mut hasher);

        format!("{:x}", hasher.finish())
    }
}

impl EntityType {
    /// Get the entity type from a SqlGraphEntity
    pub fn from_entity(entity: &SqlGraphEntity) -> Self {
        match entity {
            SqlGraphEntity::ExtensionRoot(_) => EntityType::ExtensionRoot,
            SqlGraphEntity::Schema(_) => EntityType::Schema,
            SqlGraphEntity::CustomSql(_) => EntityType::CustomSql,
            SqlGraphEntity::Function(_) => EntityType::Function,
            SqlGraphEntity::Type(_) => EntityType::Type,
            SqlGraphEntity::BuiltinType(_) => EntityType::BuiltinType,
            SqlGraphEntity::Enum(_) => EntityType::Enum,
            SqlGraphEntity::Ord(_) => EntityType::Ord,
            SqlGraphEntity::Hash(_) => EntityType::Hash,
            SqlGraphEntity::Aggregate(_) => EntityType::Aggregate,
            SqlGraphEntity::Trigger(_) => EntityType::Trigger,
        }
    }
}

// Custom serialization for SystemTime
mod systemtime_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = time.duration_since(UNIX_EPOCH).map_err(serde::ser::Error::custom)?;
        duration.as_secs().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(UNIX_EPOCH + std::time::Duration::from_secs(secs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_id_equality() {
        use crate::control_file::ControlFile;

        let control = ControlFile {
            comment: "Test extension".to_string(),
            default_version: "0.1.0".to_string(),
            module_pathname: None,
            relocatable: false,
            superuser: false,
            schema: None,
            trusted: false,
        };

        let entity1 = SqlGraphEntity::ExtensionRoot(control.clone());
        let entity2 = SqlGraphEntity::ExtensionRoot(control);

        let id1 = EntityId::from_entity(&entity1);
        let id2 = EntityId::from_entity(&entity2);

        assert_eq!(id1, id2);
    }
}
