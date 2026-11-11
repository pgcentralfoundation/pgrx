//LICENSE Portions Copyright 2026-2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use super::version::UpgradeScript;
use eyre::{Context, eyre};
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

const FRAGMENT_MARKER_PREFIX: &str = "-- Fragment: ";

static DEPENDS_ON_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)(?:--|/\*)\s*depends-on:\s*([^\n*]+?)(?:\*/|\n|$)").unwrap());

static PR_PREFIX_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(\d+)\.").unwrap());

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum FragmentId {
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
pub(crate) enum FragmentDependency {
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
pub(crate) struct MigrationFragment {
    pub(crate) path: PathBuf,
    pub(crate) filename: String,
    pub(crate) id: FragmentId,
    pub(crate) dependencies: Vec<FragmentDependency>,
    pub(crate) sql_content: String,
}

/// Fragments already assembled into upgrade scripts in `sql/`.
#[derive(Debug, Default)]
pub(crate) struct ShippedFragments {
    filenames: BTreeMap<String, PathBuf>,
    ids: BTreeSet<FragmentId>,
}

impl ShippedFragments {
    pub(crate) fn from_scripts<'a>(
        scripts: impl IntoIterator<Item = &'a UpgradeScript>,
    ) -> eyre::Result<Self> {
        let mut shipped = Self::default();
        for script in scripts {
            let sql = fs::read_to_string(&script.path)
                .wrap_err_with(|| format!("failed to read `{}`", script.path.display()))?;
            shipped.record_sql(&sql, &script.path);
        }
        Ok(shipped)
    }

    pub(crate) fn record_sql(&mut self, sql: &str, script_path: &Path) {
        for line in sql.lines() {
            if let Some(filename) =
                line.strip_prefix(FRAGMENT_MARKER_PREFIX).map(str::trim).filter(|s| !s.is_empty())
            {
                self.ids.insert(parse_fragment_id(filename));
                self.filenames
                    .entry(filename.to_string())
                    .or_insert_with(|| script_path.to_path_buf());
            }
        }
    }

    pub(crate) fn shipped_in(&self, filename: &str) -> Option<&Path> {
        self.filenames.get(filename).map(PathBuf::as_path)
    }

    pub(crate) fn satisfies(&self, dep: &FragmentDependency) -> bool {
        match dep {
            FragmentDependency::Id(id) => self.ids.contains(id),
            FragmentDependency::Filename(name) => self.filenames.contains_key(name),
        }
    }
}

pub(crate) fn parse_fragment_id(filename: &str) -> FragmentId {
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

pub(crate) fn parse_depends_on(content: &str) -> Vec<FragmentDependency> {
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

fn load_fragment(path: &Path) -> eyre::Result<MigrationFragment> {
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

/// Reads every fragment in `unreleased_dir`, in no particular order.
pub(crate) fn load_fragments(unreleased_dir: &Path) -> eyre::Result<Vec<MigrationFragment>> {
    if !unreleased_dir.exists() {
        return Ok(Vec::new());
    }

    let mut fragments = Vec::new();
    for entry in fs::read_dir(unreleased_dir)
        .wrap_err_with(|| format!("failed to read directory `{}`", unreleased_dir.display()))?
    {
        let path = entry?.path();
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

    Ok(fragments)
}

/// Returns every `(fragment, dependency)` pair that matches neither another fragment in
/// `fragments` nor an entry in `shipped`, ordered deterministically by filename.
pub(crate) fn unresolved_dependencies<'a>(
    fragments: &'a [MigrationFragment],
    shipped: &ShippedFragments,
) -> Vec<(&'a MigrationFragment, &'a FragmentDependency)> {
    let mut id_map: HashMap<&FragmentId, Vec<usize>> = HashMap::new();
    let mut filename_map: HashMap<&str, usize> = HashMap::new();

    for (idx, fragment) in fragments.iter().enumerate() {
        id_map.entry(&fragment.id).or_default().push(idx);
        filename_map.insert(&fragment.filename, idx);
    }

    let mut order: Vec<usize> = (0..fragments.len()).collect();
    order.sort_by(|&a, &b| fragments[a].filename.cmp(&fragments[b].filename));

    let mut unresolved = Vec::new();
    for idx in order {
        let fragment = &fragments[idx];
        for dep in &fragment.dependencies {
            let targets = match dep {
                FragmentDependency::Id(id) => id_map.get(id).map(Vec::as_slice),
                FragmentDependency::Filename(name) => {
                    filename_map.get(name.as_str()).map(std::slice::from_ref)
                }
            };
            let in_unreleased = targets.is_some_and(|t| t.iter().any(|&target| target != idx));
            if !in_unreleased && !shipped.satisfies(dep) {
                unresolved.push((fragment, dep));
            }
        }
    }
    unresolved
}

/// Validates that every `depends-on` resolves against `fragments` or `shipped`, then orders
/// `fragments` topologically.
pub(crate) fn resolve_and_sort_fragments(
    fragments: Vec<MigrationFragment>,
    shipped: &ShippedFragments,
    unreleased_dir: &Path,
) -> eyre::Result<Vec<MigrationFragment>> {
    let unresolved = unresolved_dependencies(&fragments, shipped);
    if !unresolved.is_empty() {
        let details = unresolved
            .into_iter()
            .map(|(fragment, dep)| format!("`{}` depends on `{dep}`", fragment.filename))
            .collect::<Vec<_>>()
            .join(", ");
        return Err(eyre!(
            "unresolved migration fragment dependency (not found in `{}` or assembled scripts in `sql/`): {details}",
            unreleased_dir.display()
        ));
    }
    topological_sort_fragments(fragments)
}

/// For each fragment, the indices of the fragments in `fragments` it declares as prerequisites.
pub(crate) fn prerequisites(fragments: &[MigrationFragment]) -> Vec<BTreeSet<usize>> {
    let mut id_map: HashMap<&FragmentId, Vec<usize>> = HashMap::new();
    let mut filename_map: HashMap<&str, usize> = HashMap::new();

    for (idx, fragment) in fragments.iter().enumerate() {
        id_map.entry(&fragment.id).or_default().push(idx);
        filename_map.insert(&fragment.filename, idx);
    }

    fragments
        .iter()
        .enumerate()
        .map(|(idx, fragment)| {
            let mut prerequisite_indices = BTreeSet::new();
            for dep in &fragment.dependencies {
                let targets = match dep {
                    FragmentDependency::Id(id) => id_map.get(id).map(Vec::as_slice),
                    FragmentDependency::Filename(name) => {
                        filename_map.get(name.as_str()).map(std::slice::from_ref)
                    }
                };

                if let Some(targets) = targets {
                    prerequisite_indices.extend(targets.iter().filter(|&&target| target != idx));
                }
            }
            prerequisite_indices
        })
        .collect()
}

pub(crate) fn topological_sort_fragments(
    fragments: Vec<MigrationFragment>,
) -> eyre::Result<Vec<MigrationFragment>> {
    if fragments.is_empty() {
        return Ok(Vec::new());
    }

    let n = fragments.len();

    // in_degree: number of unresolved prerequisites for each fragment index
    let mut in_degree = vec![0usize; n];
    // dependents: adjacency list: prerequisite index -> list of dependent fragment indices
    let mut dependents: Vec<Vec<usize>> = vec![Vec::new(); n];

    for (idx, prerequisite_indices) in prerequisites(&fragments).into_iter().enumerate() {
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

fn format_echo_header(extname: &str, target_version: &str) -> String {
    format!(
        "\\echo Use \"ALTER EXTENSION {extname} UPDATE TO '{target_version}'\" to load this file. \\quit\n"
    )
}

fn fragment_marker(filename: &str) -> String {
    format!("{FRAGMENT_MARKER_PREFIX}{filename}")
}

fn format_fragment_banner(filename: &str) -> String {
    let sep = "-- ============================================================================";
    let marker = fragment_marker(filename);
    format!("\n{sep}\n{marker}\n{sep}\n")
}

pub(crate) fn assemble_sql_content(
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
    fn resolve_and_sort_validates_against_shipped_fragments() {
        let mut shipped = ShippedFragments::default();
        shipped.record_sql(
            "-- Fragment: 6099.init.sql\r\nSELECT 1;\n-- Fragment: vector_ops.sql\n",
            Path::new("sql/my_ext--0.25.0--0.26.0.sql"),
        );

        let make = |filename: &str, deps: Vec<FragmentDependency>| MigrationFragment {
            path: PathBuf::from(filename),
            filename: filename.to_string(),
            id: parse_fragment_id(filename),
            dependencies: deps,
            sql_content: String::new(),
        };

        // Resolved via shipped ID, shipped slug, and shipped filename.
        let resolved = vec![make(
            "6221.dependent.sql",
            vec![
                FragmentDependency::Id(FragmentId::Numeric(6099)),
                FragmentDependency::Id(FragmentId::Slug("vector_ops".to_string())),
                FragmentDependency::Filename("6099.init.sql".to_string()),
            ],
        )];
        let sorted = resolve_and_sort_fragments(resolved, &shipped, Path::new("sql/unreleased"))
            .expect("shipped dependencies should resolve");
        assert_eq!(sorted[0].filename, "6221.dependent.sql");

        // Unresolved dependency and self-only dependency both fail.
        for bad_dep in [
            FragmentDependency::Id(FragmentId::Numeric(9999)),
            FragmentDependency::Filename("missing.sql".to_string()),
            FragmentDependency::Id(FragmentId::Numeric(6221)),
        ] {
            let err = resolve_and_sort_fragments(
                vec![make("6221.dependent.sql", vec![bad_dep.clone()])],
                &shipped,
                Path::new("sql/unreleased"),
            )
            .unwrap_err();
            assert!(
                err.to_string().contains(&format!("`6221.dependent.sql` depends on `{bad_dep}`")),
                "{err}"
            );
        }
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
