//LICENSE Portions Copyright 2026-2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use eyre::{Context, eyre};
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

static DEPENDS_ON_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)(?:--|/\*)\s*depends-on:\s*([^\n*]+?)(?:\*/|\n|$)").unwrap());

static PR_PREFIX_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(\d+)\.").unwrap());

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FragmentId {
    Numeric(u64),
    Slug(String),
}

impl fmt::Display for FragmentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Numeric(n) => write!(f, "{n}"),
            Self::Slug(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FragmentDependency {
    Id(FragmentId),
    Filename(String),
}

impl fmt::Display for FragmentDependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(id) => write!(f, "{id}"),
            Self::Filename(name) => write!(f, "{name}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MigrationFragment {
    pub path: PathBuf,
    pub filename: String,
    pub id: FragmentId,
    pub dependencies: Vec<FragmentDependency>,
    pub sql_content: String,
}

pub fn parse_fragment_id(filename: &str) -> FragmentId {
    if let Some(caps) = PR_PREFIX_RE.captures(filename)
        && let Ok(num) = caps[1].parse::<u64>()
    {
        return FragmentId::Numeric(num);
    }

    // If there is a dot, use the prefix before the first dot as the slug identifier.
    // Otherwise, use the whole filename.
    let slug = filename.split('.').next().unwrap_or(filename);
    FragmentId::Slug(slug.to_string())
}

pub fn parse_depends_on(content: &str) -> Vec<FragmentDependency> {
    let mut deps = Vec::new();
    let mut seen = BTreeSet::new();

    for caps in DEPENDS_ON_RE.captures_iter(content) {
        let raw_list = &caps[1];
        for item in raw_list.split(',') {
            let item = item.trim();
            if item.is_empty() {
                continue;
            }

            let dep = if item.ends_with(".sql") {
                FragmentDependency::Filename(item.to_string())
            } else {
                let token = item.trim_start_matches('#').trim();
                if let Ok(num) = token.parse::<u64>() {
                    FragmentDependency::Id(FragmentId::Numeric(num))
                } else if !token.is_empty() {
                    FragmentDependency::Id(FragmentId::Slug(token.to_string()))
                } else {
                    continue;
                }
            };

            let key = dep.to_string();
            if seen.insert(key) {
                deps.push(dep);
            }
        }
    }

    deps
}

pub fn load_fragment(path: &Path) -> eyre::Result<MigrationFragment> {
    let filename = path
        .file_name()
        .and_then(|f| f.to_str())
        .ok_or_else(|| eyre!("invalid filename for fragment at `{}`", path.display()))?
        .to_string();

    let sql_content = fs::read_to_string(path)
        .wrap_err_with(|| format!("failed to read SQL fragment `{}`", path.display()))?;

    let id = parse_fragment_id(&filename);
    let dependencies = parse_depends_on(&sql_content);

    Ok(MigrationFragment { path: path.to_path_buf(), filename, id, dependencies, sql_content })
}

pub fn collect_fragments_from_dir(unreleased_dir: &Path) -> eyre::Result<Vec<MigrationFragment>> {
    if !unreleased_dir.exists() {
        return Ok(Vec::new());
    }

    let mut fragments = Vec::new();
    for entry in fs::read_dir(unreleased_dir)
        .wrap_err_with(|| format!("failed to read directory `{}`", unreleased_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };

        if name.starts_with('.') || !name.ends_with(".sql") {
            continue;
        }

        fragments.push(load_fragment(&path)?);
    }

    topological_sort_fragments(fragments)
}

pub fn topological_sort_fragments(
    fragments: Vec<MigrationFragment>,
) -> eyre::Result<Vec<MigrationFragment>> {
    if fragments.is_empty() {
        return Ok(Vec::new());
    }

    let n = fragments.len();
    let mut id_map: HashMap<FragmentId, Vec<usize>> = HashMap::new();
    let mut filename_map: HashMap<String, usize> = HashMap::new();

    for (idx, fragment) in fragments.iter().enumerate() {
        id_map.entry(fragment.id.clone()).or_default().push(idx);
        filename_map.insert(fragment.filename.clone(), idx);
    }

    // in_degree: number of unresolved prerequisites for each fragment index
    let mut in_degree = vec![0usize; n];
    // dependents: adjacency list: prerequisite index -> list of dependent fragment indices
    let mut dependents: Vec<Vec<usize>> = vec![Vec::new(); n];

    for (idx, fragment) in fragments.iter().enumerate() {
        let mut prerequisite_indices = BTreeSet::new();

        for dep in &fragment.dependencies {
            let mut found = false;
            match dep {
                FragmentDependency::Id(id) => {
                    if let Some(target_indices) = id_map.get(id) {
                        for &target_idx in target_indices {
                            if target_idx != idx {
                                prerequisite_indices.insert(target_idx);
                            }
                        }
                        found = true;
                    }
                }
                FragmentDependency::Filename(name) => {
                    if let Some(&target_idx) = filename_map.get(name) {
                        if target_idx != idx {
                            prerequisite_indices.insert(target_idx);
                        }
                        found = true;
                    }
                }
            }

            if !found {
                tracing::debug!(
                    "fragment `{}` declares prerequisite `{}`, which was not found in unreleased fragments (assuming already applied in previous release)",
                    fragment.filename,
                    dep
                );
            }
        }

        in_degree[idx] = prerequisite_indices.len();
        for prereq_idx in prerequisite_indices {
            dependents[prereq_idx].push(idx);
        }
    }

    // Tiebreaker: (fragment.id, fragment.filename)
    let tiebreaker =
        |idx: usize| -> (&FragmentId, &str) { (&fragments[idx].id, &fragments[idx].filename) };

    // Ready elements sorted by tiebreaker
    let mut ready = BTreeSet::new();
    for (idx, &degree) in in_degree.iter().enumerate() {
        if degree == 0 {
            ready.insert((tiebreaker(idx), idx));
        }
    }

    let mut sorted_indices = Vec::with_capacity(n);

    while let Some((_, current_idx)) = ready.pop_first() {
        sorted_indices.push(current_idx);

        for &dep_idx in &dependents[current_idx] {
            in_degree[dep_idx] -= 1;
            if in_degree[dep_idx] == 0 {
                ready.insert((tiebreaker(dep_idx), dep_idx));
            }
        }
    }

    if sorted_indices.len() != n {
        let unresolved: Vec<String> = (0..n)
            .filter(|&idx| in_degree[idx] > 0)
            .map(|idx| fragments[idx].filename.clone())
            .collect();
        return Err(eyre!(
            "circular or unresolved dependency among migration fragments: {}",
            unresolved.join(", ")
        ));
    }

    let mut indexed_fragments: BTreeMap<usize, MigrationFragment> =
        fragments.into_iter().enumerate().collect();

    let result =
        sorted_indices.into_iter().map(|idx| indexed_fragments.remove(&idx).unwrap()).collect();

    Ok(result)
}

pub fn format_echo_header(extname: &str, target_version: &str) -> String {
    format!(
        "\\echo Use \"ALTER EXTENSION {extname} UPDATE TO '{target_version}'\" to load this file. \\quit\n"
    )
}

pub fn format_fragment_banner(filename: &str) -> String {
    let sep = "-- ============================================================================";
    format!("\n{sep}\n-- Fragment: {filename}\n{sep}\n")
}

pub fn assemble_sql_content(
    extname: &str,
    target_version: &str,
    fragments: &[MigrationFragment],
) -> String {
    let mut content = format_echo_header(extname, target_version);
    for fragment in fragments {
        content.push_str(&format_fragment_banner(&fragment.filename));
        content.push_str(fragment.sql_content.trim());
        content.push('\n');
    }
    content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_numeric_fragment_id() {
        assert_eq!(
            parse_fragment_id("6099.rename_solve_mvcc_to_visibility.sql"),
            FragmentId::Numeric(6099)
        );
        assert_eq!(parse_fragment_id("123.sql"), FragmentId::Numeric(123));
    }

    #[test]
    fn parses_slug_fragment_id() {
        assert_eq!(
            parse_fragment_id("add_vector_index.sql"),
            FragmentId::Slug("add_vector_index".to_string())
        );
    }

    #[test]
    fn parses_depends_on_single_and_block_comments() {
        let content = "\
-- depends-on: 6099, 6120
/* depends-on: #6221, other.sql */
SELECT 1;
";
        let deps = parse_depends_on(content);
        assert_eq!(
            deps,
            vec![
                FragmentDependency::Id(FragmentId::Numeric(6099)),
                FragmentDependency::Id(FragmentId::Numeric(6120)),
                FragmentDependency::Id(FragmentId::Numeric(6221)),
                FragmentDependency::Filename("other.sql".to_string()),
            ]
        );
    }

    #[test]
    fn topological_sort_respects_dependencies_and_tiebreaker() {
        let f1 = MigrationFragment {
            path: PathBuf::from("6099.init.sql"),
            filename: "6099.init.sql".to_string(),
            id: FragmentId::Numeric(6099),
            dependencies: vec![],
            sql_content: "-- 6099".to_string(),
        };
        let f2 = MigrationFragment {
            path: PathBuf::from("6221.dependent.sql"),
            filename: "6221.dependent.sql".to_string(),
            id: FragmentId::Numeric(6221),
            dependencies: vec![FragmentDependency::Id(FragmentId::Numeric(6099))],
            sql_content: "-- 6221".to_string(),
        };
        let f3 = MigrationFragment {
            path: PathBuf::from("6100.independent.sql"),
            filename: "6100.independent.sql".to_string(),
            id: FragmentId::Numeric(6100),
            dependencies: vec![],
            sql_content: "-- 6100".to_string(),
        };

        let sorted = topological_sort_fragments(vec![f2, f3, f1]).unwrap();
        let filenames: Vec<_> = sorted.into_iter().map(|f| f.filename).collect();
        // 6099 and 6100 are ready first; 6099 < 6100.
        // 6221 depends on 6099 so it can run once 6099 is done.
        // Queue ready has 6099, 6100. 6099 pops, adds 6221. Ready has 6100, 6221.
        // 6100 pops next because 6100 < 6221. Then 6221 pops.
        assert_eq!(filenames, vec!["6099.init.sql", "6100.independent.sql", "6221.dependent.sql"]);
    }

    #[test]
    fn missing_dependency_assumed_already_applied() {
        let f1 = MigrationFragment {
            path: PathBuf::from("6221.dependent.sql"),
            filename: "6221.dependent.sql".to_string(),
            id: FragmentId::Numeric(6221),
            dependencies: vec![FragmentDependency::Id(FragmentId::Numeric(9999))],
            sql_content: "-- 6221".to_string(),
        };
        let sorted = topological_sort_fragments(vec![f1]).unwrap();
        assert_eq!(sorted.len(), 1);
        assert_eq!(sorted[0].filename, "6221.dependent.sql");
    }

    #[test]
    fn circular_dependency_errors() {
        let f1 = MigrationFragment {
            path: PathBuf::from("100.a.sql"),
            filename: "100.a.sql".to_string(),
            id: FragmentId::Numeric(100),
            dependencies: vec![FragmentDependency::Id(FragmentId::Numeric(200))],
            sql_content: "-- 100".to_string(),
        };
        let f2 = MigrationFragment {
            path: PathBuf::from("200.b.sql"),
            filename: "200.b.sql".to_string(),
            id: FragmentId::Numeric(200),
            dependencies: vec![FragmentDependency::Id(FragmentId::Numeric(100))],
            sql_content: "-- 200".to_string(),
        };
        let err = topological_sort_fragments(vec![f1, f2]).unwrap_err();
        assert!(err.to_string().contains("circular or unresolved dependency"));
    }
}
