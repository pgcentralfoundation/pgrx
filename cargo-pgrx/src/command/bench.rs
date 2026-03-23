//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use crate::CommandExecute;
use crate::cargo::CargoProfile;
use crate::command::get::get_property;
use crate::command::run::run;
use crate::command::start::collect_postgresql_conf_settings;
use crate::manifest::{get_package_manifest, pg_config_and_version};
use eyre::{Context, eyre};
use owo_colors::OwoColorize;
use pgrx_pg_config::{PgConfig, Pgrx, createdb, dropdb};
use postgres::{Client, NoTls};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;
use uuid::Uuid;

const BENCH_WRAPPER_SCHEMA: &str = "benches";

#[derive(clap::Args, Debug, Clone)]
#[clap(author)]
pub(crate) struct Bench {
    /// Positional arguments: [pgXX] [benchname]
    #[clap(env = "PG_VERSION")]
    args: Vec<String>,

    /// If specified, use this database name instead of `$extname_benches`
    #[clap(long)]
    dbname: Option<String>,
    /// Unique name for this benchmark group
    #[clap(long)]
    group_name: Option<String>,
    /// Named benchmark group to compare against
    #[clap(long)]
    compare_group: Option<String>,
    /// Recreate the benchmark database before running
    #[clap(long)]
    resetdb: bool,
    /// Use CASCADE when dropping the extension during refresh
    #[clap(long)]
    cascade: bool,
    /// List discovered benchmark wrappers and exit
    #[clap(long)]
    list: bool,
    /// Emit the final summary as JSON
    #[clap(long)]
    json: bool,
    /// Package to build (see `cargo help pkgid`)
    #[clap(long, short)]
    package: Option<String>,
    /// Path to Cargo.toml
    #[clap(long, value_parser)]
    manifest_path: Option<PathBuf>,
    /// Compile for debug mode instead of the default release mode
    #[clap(long)]
    debug: bool,
    /// Specific profile to use (conflicts with `--debug`)
    #[clap(long)]
    profile: Option<String>,
    #[clap(flatten)]
    features: clap_cargo::Features,
    #[clap(long)]
    target: Option<String>,
    #[clap(from_global, action = clap::ArgAction::Count)]
    verbose: u8,
    /// Custom `postgresql.conf` settings in the form of `key=value`
    #[clap(long)]
    postgresql_conf: Vec<String>,
}

impl CommandExecute for Bench {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        let (resolved_pg_version, bench_filter) = self.resolve_args()?;
        let postgresql_conf = collect_postgresql_conf_settings(&self.postgresql_conf)?;
        let pgrx = Pgrx::from_config()?;

        let (package_manifest, package_manifest_path) = get_package_manifest(
            &self.features,
            self.package.as_deref(),
            self.manifest_path.as_deref(),
        )?;
        let mut features = self.features.clone();
        ensure_feature(&mut features, "pg_bench");
        let (pg_config, _) = pg_config_and_version(
            &pgrx,
            &package_manifest,
            resolved_pg_version,
            Some(&mut features),
            true,
        )?;

        let extname = get_property(&package_manifest_path, "extname")?
            .ok_or(eyre!("could not determine extension name"))?;
        let extversion = crate::command::install::get_version(&package_manifest_path)?;
        let dbname = self.dbname.clone().unwrap_or_else(|| format!("{extname}_benches"));
        let profile = CargoProfile::from_flags(
            self.profile.as_deref(),
            if self.debug { CargoProfile::Dev } else { CargoProfile::Release },
        )?;

        run(
            &pg_config,
            self.manifest_path.as_deref(),
            self.package.as_deref(),
            &package_manifest_path,
            &dbname,
            true,
            &profile,
            &features,
            false,
            false,
            self.target.as_deref(),
            &postgresql_conf,
        )?;

        if self.resetdb {
            dropdb(&pg_config, &dbname, false, None)?;
            createdb(&pg_config, &dbname, false, true, None)?;
        }

        let mut client = connect_client(&pg_config, &dbname)?;
        ensure_persistent_schema(&mut client)?;

        refresh_extension(&mut client, &extname, self.cascade)?;

        let benchmarks = discover_benchmarks(&mut client, bench_filter.as_deref())?;
        if self.list {
            for benchmark in benchmarks {
                println!(
                    "{} [{}]",
                    benchmark.descriptor.bench_name,
                    format_benchmark_settings(&benchmark.descriptor)
                );
            }
            return Ok(());
        }

        if benchmarks.is_empty() {
            eyre::bail!(
                "no benchmarks discovered in schema `{BENCH_WRAPPER_SCHEMA}` from `mod benches`"
            );
        }

        let git_metadata = collect_git_metadata(package_manifest_path.parent().unwrap())?;
        let resolved_group_name = match self.group_name.clone() {
            Some(name) => name,
            None => default_group_name(&mut client, git_metadata.git_commit.as_deref())?,
        };
        let compare_group = resolve_compare_group(
            &mut client,
            self.compare_group.as_deref(),
            &profile,
            &resolved_group_name,
        )?;

        let run_group_id = insert_run_group(
            &mut client,
            &resolved_group_name,
            compare_group.as_ref(),
            &extname,
            &extversion,
            &pg_config,
            &profile,
            &features,
            &git_metadata,
        )?;
        snapshot_pg_settings(&mut client, run_group_id)?;

        let mut summary_benchmarks = Vec::new();
        let mut failures = 0usize;

        for benchmark in &benchmarks {
            print_running_benchmark(benchmark);
            let baseline = compare_group
                .as_ref()
                .map(|group| {
                    load_benchmark_result_for_group(
                        &mut client,
                        group.id,
                        &benchmark.descriptor.bench_name,
                    )
                })
                .transpose()?
                .flatten();
            // Persisted Criterion artifacts are replayed back into the backend so the benchmark's
            // `change` analysis comes from Criterion's own baseline processing instead of a host-
            // side approximation over normalized SQL tables.
            let baseline_artifacts =
                if baseline.as_ref().is_some_and(|baseline| baseline.status == BenchStatus::Ok) {
                    compare_group
                        .as_ref()
                        .map(|group| {
                            load_criterion_artifacts_for_group(
                                &mut client,
                                group.id,
                                &benchmark.descriptor.bench_name,
                            )
                        })
                        .transpose()?
                        .flatten()
                } else {
                    None
                };
            let started_at = SystemTime::now();
            let payload = execute_benchmark_query(
                &mut client,
                &benchmark.run_wrapper_name,
                baseline_artifacts.as_deref(),
            )?;
            let finished_at = SystemTime::now();

            if payload.status == BenchStatus::Failed {
                failures += 1;
            }

            let benchmark_run_id = persist_benchmark_result(
                &mut client,
                run_group_id,
                &payload,
                started_at,
                finished_at,
            )?;
            summary_benchmarks.push(build_benchmark_summary(
                benchmark_run_id,
                &payload,
                baseline.as_ref(),
                compare_group.as_ref().map(|group| group.group_name.as_str()),
            ));
            if let Some(completed_benchmark) = summary_benchmarks.last() {
                print_completed_benchmark(completed_benchmark);
            }
        }

        let status = if failures == 0 {
            "completed"
        } else if failures == benchmarks.len() {
            "failed"
        } else {
            "partial"
        };
        mark_run_group_complete(&mut client, run_group_id, status)?;

        let missing_from_current = if let Some(compare_group) = &compare_group {
            load_missing_benchmarks(&mut client, run_group_id, compare_group.id)?
        } else {
            Vec::new()
        };
        let summary = BenchSummary {
            group_name: resolved_group_name,
            compare_group_name: compare_group.map(|group| group.group_name),
            benchmarks: summary_benchmarks,
            missing_from_current,
        };

        if self.json {
            println!("{}", serde_json::to_string_pretty(&summary)?);
        } else {
            print_summary(&summary);
        }

        Ok(())
    }
}

impl Bench {
    fn resolve_args(&self) -> eyre::Result<(Option<String>, Option<String>)> {
        match self.args.as_slice() {
            [] => Ok((None, None)),
            [only] if only.starts_with("pg") => Ok((Some(only.clone()), None)),
            [only] => Ok((None, Some(only.clone()))),
            [pg_version, benchname] if pg_version.starts_with("pg") => {
                Ok((Some(pg_version.clone()), Some(benchname.clone())))
            }
            _ => Err(eyre!(
                "expected positional arguments `[pgXX] [benchname]`, got `{}`",
                self.args.join(" ")
            )),
        }
    }
}

fn ensure_feature(features: &mut clap_cargo::Features, feature: &str) {
    if !features.features.iter().any(|value| value == feature) {
        features.features.push(feature.to_string());
    }
}

fn connect_client(pg_config: &PgConfig, dbname: &str) -> eyre::Result<Client> {
    let user = current_user()?;
    postgres::Config::new()
        .host(pg_config.host())
        .port(pg_config.port()?)
        .user(&user)
        .dbname(dbname)
        .connect(NoTls)
        .wrap_err("failed to connect to Postgres benchmark database")
}

fn current_user() -> eyre::Result<String> {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .map_err(|_| eyre!("could not determine current operating-system user from environment"))
}

fn ensure_persistent_schema(client: &mut Client) -> eyre::Result<()> {
    client.batch_execute(PERSISTENT_SCHEMA_SQL)?;
    Ok(())
}

fn refresh_extension(client: &mut Client, extname: &str, cascade: bool) -> eyre::Result<()> {
    let quoted_extname = quote_ident(extname);
    let drop_sql = if cascade {
        format!("DROP EXTENSION IF EXISTS {quoted_extname} CASCADE")
    } else {
        format!("DROP EXTENSION IF EXISTS {quoted_extname}")
    };

    if let Err(error) = client.batch_execute(&drop_sql) {
        if !cascade {
            return Err(eyre!(
                "failed to drop extension `{extname}` before bench refresh: {error}\nrerun with `cargo pgrx bench --cascade` if you want dependency cleanup"
            ));
        }
        return Err(error.into());
    }

    client.batch_execute(&format!("CREATE EXTENSION {quoted_extname}"))?;
    Ok(())
}

fn discover_benchmarks(
    client: &mut Client,
    filter: Option<&str>,
) -> eyre::Result<Vec<DiscoveredBenchmark>> {
    let rows = client.query(
        "SELECT proname
         FROM pg_proc
         JOIN pg_namespace ON pg_namespace.oid = pg_proc.pronamespace
         WHERE pg_namespace.nspname = $1
           AND proname LIKE '__pgrx_bench_run_%'
         ORDER BY proname",
        &[&BENCH_WRAPPER_SCHEMA],
    )?;

    let mut benchmarks = Vec::new();
    for row in rows {
        let run_wrapper_name: String = row.get(0);
        let descriptor = load_benchmark_descriptor(client, &run_wrapper_name)?;
        if filter.is_none_or(|filter| {
            run_wrapper_name.contains(filter) || descriptor.bench_name.contains(filter)
        }) {
            benchmarks.push(DiscoveredBenchmark { run_wrapper_name, descriptor });
        }
    }

    Ok(benchmarks)
}

fn execute_benchmark_query(
    client: &mut Client,
    benchmark_wrapper: &str,
    baseline_artifacts: Option<&[BenchArtifact]>,
) -> eyre::Result<BenchResult> {
    let query = format!(
        "SELECT {}.{}($1)",
        quote_ident(BENCH_WRAPPER_SCHEMA),
        quote_ident(benchmark_wrapper)
    );
    let mut tx = client.transaction()?;
    let baseline_payload = baseline_artifacts.map(serde_json::to_value).transpose()?;
    let row = tx.query_one(&query, &[&baseline_payload])?;
    let payload: Value = row.get(0);
    tx.rollback()?;
    serde_json::from_value(payload).wrap_err("failed to decode benchmark result payload")
}

fn load_benchmark_descriptor(
    client: &mut Client,
    run_wrapper_name: &str,
) -> eyre::Result<BenchDescriptor> {
    let describe_wrapper_name =
        run_wrapper_name.replacen("__pgrx_bench_run_", "__pgrx_bench_describe_", 1);
    let query = format!(
        "SELECT {}.{}()",
        quote_ident(BENCH_WRAPPER_SCHEMA),
        quote_ident(&describe_wrapper_name)
    );
    let row = client.query_one(&query, &[])?;
    let payload: Value = row.get(0);
    serde_json::from_value(payload).wrap_err("failed to decode benchmark descriptor payload")
}

fn insert_run_group(
    client: &mut Client,
    group_name: &str,
    compare_group: Option<&ResolvedGroup>,
    extname: &str,
    extversion: &str,
    pg_config: &PgConfig,
    profile: &CargoProfile,
    features: &clap_cargo::Features,
    git_metadata: &GitMetadata,
) -> eyre::Result<Uuid> {
    let id = Uuid::new_v4();
    let cargo_features = features.features.clone();
    let command_line = std::env::args().collect::<Vec<_>>().join(" ");
    let hostname = host_name().ok();
    let rustc_version = command_output("rustc", ["--version"]).ok();
    let cargo_version = command_output("cargo", ["--version"]).ok();
    let os = std::env::consts::OS.to_string();
    let arch = std::env::consts::ARCH.to_string();
    let compare_group_id = compare_group.map(|group| group.id);

    client.execute(
        "INSERT INTO pgrx_bench.run_group (
            id,
            group_name,
            status,
            compare_group_id,
            extname,
            extversion,
            pg_version_major,
            profile_name,
            cargo_features,
            command_line,
            hostname,
            os,
            arch,
            rustc_version,
            cargo_version,
            pgrx_version,
            cargo_pgrx_version,
            git_commit,
            git_branch,
            git_dirty,
            git_describe
        ) VALUES (
            $1, $2, 'running', $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20
        )",
        &[
            &id,
            &group_name,
            &compare_group_id,
            &extname,
            &extversion,
            &i32::try_from(pg_config.major_version()?)?,
            &profile.name(),
            &cargo_features,
            &command_line,
            &hostname,
            &os,
            &arch,
            &rustc_version,
            &cargo_version,
            &env!("CARGO_PKG_VERSION"),
            &env!("CARGO_PKG_VERSION"),
            &git_metadata.git_commit,
            &git_metadata.git_branch,
            &git_metadata.git_dirty,
            &git_metadata.git_describe,
        ],
    )?;

    Ok(id)
}

fn snapshot_pg_settings(client: &mut Client, group_id: Uuid) -> eyre::Result<()> {
    client.execute(
        "INSERT INTO pgrx_bench.run_group_pg_setting (
            group_id, name, setting, unit, source, sourcefile, sourceline, boot_val, reset_val, pending_restart
        )
        SELECT
            $1, name, setting, unit, source, sourcefile, sourceline, boot_val, reset_val, pending_restart
        FROM pg_settings",
        &[&group_id],
    )?;
    Ok(())
}

fn persist_benchmark_result(
    client: &mut Client,
    group_id: Uuid,
    payload: &BenchResult,
    started_at: SystemTime,
    finished_at: SystemTime,
) -> eyre::Result<i64> {
    let mut tx = client.transaction()?;

    let case_id: i64 = tx
        .query_one(
            "INSERT INTO pgrx_bench.benchmark_case (
                schema_name,
                bench_name,
                function_name,
                setup_function,
                transaction_mode,
                source_file,
                source_line
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (schema_name, bench_name) DO UPDATE
            SET function_name = EXCLUDED.function_name,
                setup_function = EXCLUDED.setup_function,
                transaction_mode = EXCLUDED.transaction_mode,
                source_file = EXCLUDED.source_file,
                source_line = EXCLUDED.source_line
            RETURNING id",
            &[
                &payload.schema_name,
                &payload.bench_name,
                &payload.function_name,
                &payload.setup_function,
                &payload.transaction_mode.as_str(),
                &payload.source_file,
                &i32::try_from(payload.source_line)?,
            ],
        )?
        .get(0);

    // The exact Criterion files live in `pgrx_bench.artifact`; `raw_result` keeps the normalized
    // benchmark payload small enough for summary queries and historical comparisons.
    let mut raw_payload = payload.clone();
    raw_payload.artifacts.clear();
    let raw_result = serde_json::to_value(&raw_payload)?;
    let benchmark_run_id: i64 = tx
        .query_one(
            "INSERT INTO pgrx_bench.benchmark_run (
                group_id,
                case_id,
                status,
                error_text,
                started_at,
                finished_at,
                criterion_config,
                raw_result
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id",
            &[
                &group_id,
                &case_id,
                &bench_status_as_str(&payload.status),
                &payload.error_text,
                &started_at,
                &finished_at,
                &serde_json::to_value(&payload.criterion_config)?,
                &raw_result,
            ],
        )?
        .get(0);

    for estimate in &payload.estimates {
        tx.execute(
            "INSERT INTO pgrx_bench.benchmark_estimate (
                benchmark_run_id,
                estimate_kind,
                point_estimate_ns,
                standard_error_ns,
                confidence_level,
                ci_lower_bound_ns,
                ci_upper_bound_ns
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            &[
                &benchmark_run_id,
                &estimate.estimate_kind,
                &estimate.point_estimate_ns,
                &estimate.standard_error_ns,
                &estimate.confidence_level,
                &estimate.ci_lower_bound_ns,
                &estimate.ci_upper_bound_ns,
            ],
        )?;
    }

    for sample in &payload.samples {
        tx.execute(
            "INSERT INTO pgrx_bench.benchmark_sample (
                benchmark_run_id,
                sample_index,
                iteration_count,
                elapsed_ns
            ) VALUES ($1, $2, $3, $4)",
            &[
                &benchmark_run_id,
                &i32::try_from(sample.sample_index)?,
                &i64::try_from(sample.iteration_count)?,
                &sample.elapsed_ns,
            ],
        )?;
    }

    if let Some(throughput) = &payload.throughput {
        tx.execute(
            "INSERT INTO pgrx_bench.benchmark_throughput (
                benchmark_run_id,
                kind,
                value
            ) VALUES ($1, $2, $3)",
            &[&benchmark_run_id, &throughput.kind, &throughput.value],
        )?;
    }

    for artifact in &payload.artifacts {
        tx.execute(
            "INSERT INTO pgrx_bench.artifact (
                benchmark_run_id,
                artifact_kind,
                media_type,
                payload_json
            ) VALUES ($1, $2, $3, $4)",
            &[
                &benchmark_run_id,
                &artifact.artifact_kind,
                &artifact.media_type,
                &artifact.payload_json,
            ],
        )?;
    }

    tx.commit()?;
    Ok(benchmark_run_id)
}

fn mark_run_group_complete(client: &mut Client, group_id: Uuid, status: &str) -> eyre::Result<()> {
    client.execute(
        "UPDATE pgrx_bench.run_group
         SET status = $2,
             completed_at = clock_timestamp()
         WHERE id = $1",
        &[&group_id, &status],
    )?;
    Ok(())
}

fn load_benchmark_result_for_group(
    client: &mut Client,
    group_id: Uuid,
    bench_name: &str,
) -> eyre::Result<Option<BenchResult>> {
    let row = client.query_opt(
        "SELECT benchmark_run.raw_result
         FROM pgrx_bench.benchmark_run
         JOIN pgrx_bench.benchmark_case ON benchmark_case.id = benchmark_run.case_id
         WHERE benchmark_run.group_id = $1
           AND benchmark_case.bench_name = $2",
        &[&group_id, &bench_name],
    )?;

    row.map(|row| {
        let payload: Value = row.get(0);
        serde_json::from_value(payload).wrap_err("failed to decode persisted benchmark result")
    })
    .transpose()
}

fn load_criterion_artifacts_for_group(
    client: &mut Client,
    group_id: Uuid,
    bench_name: &str,
) -> eyre::Result<Option<Vec<BenchArtifact>>> {
    let rows = client.query(
        "SELECT artifact_kind, media_type, payload_json
         FROM pgrx_bench.artifact
         JOIN pgrx_bench.benchmark_run ON benchmark_run.id = artifact.benchmark_run_id
         JOIN pgrx_bench.benchmark_case ON benchmark_case.id = benchmark_run.case_id
         WHERE benchmark_run.group_id = $1
           AND benchmark_case.bench_name = $2
         ORDER BY artifact_kind",
        &[&group_id, &bench_name],
    )?;

    if rows.is_empty() {
        return Ok(None);
    }

    Ok(Some(
        rows.into_iter()
            .map(|row| BenchArtifact {
                artifact_kind: row.get(0),
                media_type: row.get(1),
                payload_json: row.get(2),
            })
            .collect(),
    ))
}

fn load_missing_benchmarks(
    client: &mut Client,
    current_group_id: Uuid,
    baseline_group_id: Uuid,
) -> eyre::Result<Vec<String>> {
    let rows = client.query(
        "SELECT baseline_case.bench_name
         FROM pgrx_bench.benchmark_run AS baseline_run
         JOIN pgrx_bench.benchmark_case AS baseline_case
           ON baseline_case.id = baseline_run.case_id
         LEFT JOIN pgrx_bench.benchmark_run AS current_run
           ON current_run.group_id = $1
          AND current_run.case_id = baseline_run.case_id
         WHERE baseline_run.group_id = $2
           AND current_run.id IS NULL
         ORDER BY baseline_case.bench_name",
        &[&current_group_id, &baseline_group_id],
    )?;

    Ok(rows.into_iter().map(|row| row.get(0)).collect())
}

fn resolve_compare_group(
    client: &mut Client,
    compare_group_name: Option<&str>,
    profile: &CargoProfile,
    current_group_name: &str,
) -> eyre::Result<Option<ResolvedGroup>> {
    let row = if let Some(compare_group_name) = compare_group_name {
        client
            .query_opt(
                "SELECT id, group_name
                 FROM pgrx_bench.run_group
                 WHERE group_name = $1",
                &[&compare_group_name],
            )?
            .ok_or_else(|| eyre!("comparison group `{compare_group_name}` was not found"))?
    } else {
        match client.query_opt(
            "SELECT id, group_name
             FROM pgrx_bench.run_group
             WHERE status IN ('completed', 'partial')
               AND profile_name = $1
               AND group_name <> $2
             ORDER BY created_at DESC
             LIMIT 1",
            &[&profile.name(), &current_group_name],
        )? {
            Some(row) => row,
            None => return Ok(None),
        }
    };

    Ok(Some(ResolvedGroup { id: row.get(0), group_name: row.get(1) }))
}

fn default_group_name(client: &mut Client, git_commit: Option<&str>) -> eyre::Result<String> {
    let timestamp: String =
        client.query_one("SELECT to_char(clock_timestamp(), 'YYYYMMDD_HH24MISS')", &[])?.get(0);
    let short_hash = git_commit
        .map(|commit| commit.chars().take(7).collect::<String>())
        .unwrap_or_else(|| "nogit".to_string());
    Ok(format!("{timestamp}_{short_hash}"))
}

fn build_benchmark_summary(
    benchmark_run_id: i64,
    current: &BenchResult,
    baseline: Option<&BenchResult>,
    compare_group_name: Option<&str>,
) -> BenchmarkSummaryRow {
    let primary_estimate = primary_estimate_display(current);
    let comparison = compare_group_name
        .map(|compare_group_name| build_change_summary(current, baseline, compare_group_name));

    BenchmarkSummaryRow {
        benchmark_run_id,
        bench_name: current.bench_name.clone(),
        status: current.status.clone(),
        error_text: current.error_text.clone(),
        primary_estimate,
        throughput: current.throughput.clone(),
        slope: estimate_display(current, "slope"),
        mean: estimate_display(current, "mean"),
        std_dev: estimate_display(current, "std_dev"),
        median: estimate_display(current, "median"),
        median_abs_dev: estimate_display(current, "median_abs_dev"),
        comparison,
    }
}

fn primary_estimate_display(payload: &BenchResult) -> Option<EstimateDisplay> {
    estimate_display(payload, "slope").or_else(|| estimate_display(payload, "mean"))
}

fn estimate_display(payload: &BenchResult, estimate_kind: &str) -> Option<EstimateDisplay> {
    payload.estimates.iter().find(|estimate| estimate.estimate_kind == estimate_kind).map(
        |estimate| EstimateDisplay {
            estimate_kind: estimate.estimate_kind.clone(),
            point_estimate_ns: estimate.point_estimate_ns,
            ci_lower_bound_ns: estimate.ci_lower_bound_ns,
            ci_upper_bound_ns: estimate.ci_upper_bound_ns,
            confidence_level: estimate.confidence_level,
            standard_error_ns: estimate.standard_error_ns,
        },
    )
}

fn build_change_summary(
    current: &BenchResult,
    baseline: Option<&BenchResult>,
    compare_group_name: &str,
) -> ChangeSummary {
    if let Some(comparison) = &current.comparison {
        return ChangeSummary {
            baseline_group_name: compare_group_name.to_string(),
            lower_pct: Some(comparison.mean.ci_lower_bound * 100.0),
            point_pct: Some(comparison.mean.point_estimate * 100.0),
            upper_pct: Some(comparison.mean.ci_upper_bound * 100.0),
            p_value: Some(comparison.p_value),
            significance_level: comparison.significance_level,
            noise_threshold: comparison.noise_threshold,
            summary: comparison.summary.clone(),
        };
    }

    let baseline = match baseline {
        Some(baseline) => baseline,
        None => {
            return ChangeSummary {
                baseline_group_name: compare_group_name.to_string(),
                lower_pct: None,
                point_pct: None,
                upper_pct: None,
                p_value: None,
                significance_level: current.criterion_config.significance_level,
                noise_threshold: current.criterion_config.noise_threshold,
                summary: "New benchmark; no baseline comparison available.".to_string(),
            };
        }
    };

    if current.status != BenchStatus::Ok {
        return ChangeSummary {
            baseline_group_name: compare_group_name.to_string(),
            lower_pct: None,
            point_pct: None,
            upper_pct: None,
            p_value: None,
            significance_level: current.criterion_config.significance_level,
            noise_threshold: current.criterion_config.noise_threshold,
            summary: "Comparison unavailable because the current benchmark failed.".to_string(),
        };
    }

    if baseline.status != BenchStatus::Ok {
        return ChangeSummary {
            baseline_group_name: compare_group_name.to_string(),
            lower_pct: None,
            point_pct: None,
            upper_pct: None,
            p_value: None,
            significance_level: current.criterion_config.significance_level,
            noise_threshold: current.criterion_config.noise_threshold,
            summary: "Comparison unavailable because the baseline benchmark did not complete successfully.".to_string(),
        };
    }

    // If we had a persisted baseline and both runs succeeded, the backend should normally have
    // returned Criterion comparison data. Hitting this branch means the raw samples were kept but
    // Criterion did not emit `change/estimates.json`, which is useful to surface explicitly.
    ChangeSummary {
        baseline_group_name: compare_group_name.to_string(),
        lower_pct: None,
        point_pct: None,
        upper_pct: None,
        p_value: None,
        significance_level: current.criterion_config.significance_level,
        noise_threshold: current.criterion_config.noise_threshold,
        summary: "Comparison unavailable because Criterion did not emit comparison output."
            .to_string(),
    }
}

fn bench_status_as_str(status: &BenchStatus) -> &'static str {
    match status {
        BenchStatus::Ok => "ok",
        BenchStatus::Failed => "failed",
    }
}

fn quote_ident(value: &str) -> String {
    format!("\"{}\"", value.replace('"', "\"\""))
}

fn command_output<'a>(
    program: &str,
    args: impl IntoIterator<Item = &'a str>,
) -> eyre::Result<String> {
    let args = args.into_iter().collect::<Vec<_>>();
    let output = Command::new(program)
        .args(&args)
        .output()
        .wrap_err_with(|| format!("failed to run `{}`", format_program_and_args(program, &args)))?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(eyre!(
            "command `{}` failed: {}",
            format_program_and_args(program, &args),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

fn command_output_in_dir<'a>(
    program: &str,
    args: impl IntoIterator<Item = &'a str>,
    current_dir: &Path,
) -> eyre::Result<String> {
    let args = args.into_iter().collect::<Vec<_>>();
    let output = Command::new(program)
        .args(&args)
        .current_dir(current_dir)
        .output()
        .wrap_err_with(|| format!("failed to run `{}`", format_program_and_args(program, &args)))?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(eyre!(
            "command `{}` failed: {}",
            format_program_and_args(program, &args),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

fn format_program_and_args(program: &str, args: &[&str]) -> String {
    let mut parts = Vec::with_capacity(args.len() + 1);
    parts.push(program);
    parts.extend_from_slice(args);
    parts.join(" ")
}

fn host_name() -> eyre::Result<String> {
    if let Ok(hostname) = std::env::var("HOSTNAME") {
        return Ok(hostname);
    }

    if let Ok(hostname) = std::env::var("COMPUTERNAME") {
        return Ok(hostname);
    }

    command_output("hostname", std::iter::empty())
}

fn collect_git_metadata(root: &Path) -> eyre::Result<GitMetadata> {
    let git_commit = command_output_in_dir("git", ["rev-parse", "HEAD"], root).ok();
    let git_branch =
        command_output_in_dir("git", ["rev-parse", "--abbrev-ref", "HEAD"], root).ok();
    let git_describe =
        command_output_in_dir("git", ["describe", "--always", "--dirty", "--tags"], root).ok();
    let git_dirty = command_output_in_dir(
        "git",
        ["status", "--porcelain", "--untracked-files=no"],
        root,
    )
    .map(|status| !status.is_empty())
    .unwrap_or(false);

    Ok(GitMetadata { git_commit, git_branch, git_describe, git_dirty })
}

fn print_summary(summary: &BenchSummary) {
    let total = summary.benchmarks.len();
    let failed = summary
        .benchmarks
        .iter()
        .filter(|benchmark| benchmark.status == BenchStatus::Failed)
        .count();
    let succeeded = total.saturating_sub(failed);

    println!("{} {}", "Bench group".bold().green(), summary.group_name.bold().white());
    if let Some(compare_group_name) = &summary.compare_group_name {
        println!("{} {}", "   Compared".bold().cyan(), compare_group_name.bold().white());
    }
    println!(
        "{} {} total, {} ok, {} failed",
        "    Result".bold().cyan(),
        total,
        succeeded.to_string().green(),
        failed.to_string().red()
    );

    if !summary.missing_from_current.is_empty() {
        println!("{}", "Missing From Current Run".bold().cyan());
        for bench_name in &summary.missing_from_current {
            println!("  {}", bench_name);
        }
    }
}

fn print_completed_benchmark(benchmark: &BenchmarkSummaryRow) {
    println!("{}", benchmark.bench_name.bold());

    if benchmark.status == BenchStatus::Failed {
        let message =
            benchmark.error_text.as_deref().unwrap_or("benchmark failed without an error message");
        println!("{}{}", summary_indent(), format!("error:  {message}").red());
        println!();
        return;
    }

    if let Some(primary_estimate) = &benchmark.primary_estimate {
        println!("{}time:   {}", summary_indent(), format_estimate_interval(primary_estimate));
    }

    if let Some(throughput) = &benchmark.throughput {
        println!("{}thrpt:  {}", summary_indent(), format_throughput(throughput));
    }

    if let Some(change) = &benchmark.comparison {
        if let (Some(lower), Some(point), Some(upper)) =
            (change.lower_pct, change.point_pct, change.upper_pct)
        {
            let p_value = change
                .p_value
                .map(|p_value| {
                    let comparator = if p_value < change.significance_level { "<" } else { ">" };
                    format!(" (p = {:.2} {} {:.2})", p_value, comparator, change.significance_level)
                })
                .unwrap_or_default();
            println!(
                "{}change: [{} {} {}]{}",
                summary_indent(),
                format_percent(lower),
                format_percent(point),
                format_percent(upper),
                p_value
            );
        }
        println!("{}{}", summary_indent(), change.summary);
    }

    if let Some(slope) = &benchmark.slope {
        println!("{}slope:  {}", summary_indent(), format_estimate_interval(slope));
    }

    if let Some(mean) = &benchmark.mean {
        if let Some(std_dev) = &benchmark.std_dev {
            println!(
                "{}mean:   {} std. dev. {}",
                summary_indent(),
                format_estimate_interval(mean),
                format_estimate_interval(std_dev)
            );
        } else {
            println!("{}mean:   {}", summary_indent(), format_estimate_interval(mean));
        }
    }

    if let Some(median) = &benchmark.median {
        if let Some(median_abs_dev) = &benchmark.median_abs_dev {
            println!(
                "{}median: {} med. abs. dev. {}",
                summary_indent(),
                format_estimate_interval(median),
                format_estimate_interval(median_abs_dev)
            );
        } else {
            println!("{}median: {}", summary_indent(), format_estimate_interval(median));
        }
    }

    println!();
}

fn print_running_benchmark(benchmark: &DiscoveredBenchmark) {
    println!(
        "{} {} [{}]",
        "     Running".bold().green(),
        benchmark.descriptor.bench_name.bold().white(),
        format_benchmark_settings(&benchmark.descriptor).cyan()
    );
}

fn format_benchmark_settings(descriptor: &BenchDescriptor) -> String {
    format!(
        "transaction={}, setup={}, sample_size={}, warm_up={}ms, measurement={}ms, nresamples={}, noise_threshold={}, significance_level={}",
        descriptor.transaction_mode.as_str(),
        descriptor.setup_function.as_deref().unwrap_or("none"),
        descriptor.criterion_config.sample_size,
        descriptor.criterion_config.warm_up_time_ms,
        descriptor.criterion_config.measurement_time_ms,
        descriptor.criterion_config.nresamples,
        descriptor.criterion_config.noise_threshold,
        descriptor.criterion_config.significance_level,
    )
}

fn summary_indent() -> String {
    format!("{:>28}", "")
}

fn format_estimate_interval(estimate: &EstimateDisplay) -> String {
    match (estimate.ci_lower_bound_ns, estimate.ci_upper_bound_ns) {
        (Some(lower), Some(upper)) => format!(
            "[{} {} {}]",
            format_duration_ns(lower),
            format_duration_ns(estimate.point_estimate_ns),
            format_duration_ns(upper)
        ),
        _ => format_duration_ns(estimate.point_estimate_ns),
    }
}

fn format_duration_ns(value_ns: f64) -> String {
    if value_ns.abs() >= 1_000_000_000.0 {
        format_measurement(value_ns / 1_000_000_000.0, "s")
    } else if value_ns.abs() >= 1_000_000.0 {
        format_measurement(value_ns / 1_000_000.0, "ms")
    } else if value_ns.abs() >= 1_000.0 {
        format_measurement(value_ns / 1_000.0, "us")
    } else if value_ns.abs() >= 1.0 {
        format_measurement(value_ns, "ns")
    } else {
        format_measurement(value_ns * 1_000.0, "ps")
    }
}

fn format_measurement(value: f64, unit: &str) -> String {
    let formatted = if value.abs() >= 100.0 {
        format!("{value:.2}")
    } else if value.abs() >= 10.0 {
        format!("{value:.3}")
    } else {
        format!("{value:.4}")
    };
    format!("{} {}", trim_trailing_zeroes(formatted), unit)
}

fn trim_trailing_zeroes(mut value: String) -> String {
    if value.contains('.') {
        while value.ends_with('0') {
            value.pop();
        }
        if value.ends_with('.') {
            value.push('0');
        }
    }
    value
}

fn format_percent(value: f64) -> String {
    let formatted = if value.abs() >= 100.0 {
        format!("{value:+.2}")
    } else if value.abs() >= 10.0 {
        format!("{value:+.3}")
    } else {
        format!("{value:+.4}")
    };
    format!("{}%", trim_trailing_zeroes(formatted))
}

fn format_throughput(throughput: &BenchThroughput) -> String {
    match throughput.kind.as_str() {
        "bytes" => format!("{:.2} bytes/s", throughput.value),
        "elements" => format!("{:.2} elem/s", throughput.value),
        kind => format!("{:.2} {kind}/s", throughput.value),
    }
}

#[derive(Debug)]
struct ResolvedGroup {
    id: Uuid,
    group_name: String,
}

#[derive(Debug, Default)]
struct GitMetadata {
    git_commit: Option<String>,
    git_branch: Option<String>,
    git_describe: Option<String>,
    git_dirty: bool,
}

#[derive(Debug, Serialize)]
struct BenchSummary {
    group_name: String,
    compare_group_name: Option<String>,
    benchmarks: Vec<BenchmarkSummaryRow>,
    missing_from_current: Vec<String>,
}

#[derive(Debug, Serialize)]
struct BenchmarkSummaryRow {
    benchmark_run_id: i64,
    bench_name: String,
    status: BenchStatus,
    error_text: Option<String>,
    primary_estimate: Option<EstimateDisplay>,
    throughput: Option<BenchThroughput>,
    slope: Option<EstimateDisplay>,
    mean: Option<EstimateDisplay>,
    std_dev: Option<EstimateDisplay>,
    median: Option<EstimateDisplay>,
    median_abs_dev: Option<EstimateDisplay>,
    comparison: Option<ChangeSummary>,
}

#[derive(Debug, Serialize)]
struct EstimateDisplay {
    estimate_kind: String,
    point_estimate_ns: f64,
    ci_lower_bound_ns: Option<f64>,
    ci_upper_bound_ns: Option<f64>,
    confidence_level: Option<f64>,
    standard_error_ns: Option<f64>,
}

#[derive(Debug, Serialize)]
struct ChangeSummary {
    baseline_group_name: String,
    lower_pct: Option<f64>,
    point_pct: Option<f64>,
    upper_pct: Option<f64>,
    p_value: Option<f64>,
    significance_level: f64,
    noise_threshold: f64,
    summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiscoveredBenchmark {
    run_wrapper_name: String,
    descriptor: BenchDescriptor,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
enum BenchStatus {
    Ok,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BenchResult {
    schema_name: String,
    bench_name: String,
    function_name: String,
    setup_function: Option<String>,
    transaction_mode: BenchTransactionMode,
    source_file: String,
    source_line: u32,
    criterion_config: BenchConfig,
    status: BenchStatus,
    error_text: Option<String>,
    estimates: Vec<BenchEstimate>,
    samples: Vec<BenchSample>,
    throughput: Option<BenchThroughput>,
    #[serde(default)]
    comparison: Option<BenchComparison>,
    #[serde(default)]
    artifacts: Vec<BenchArtifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BenchDescriptor {
    schema_name: String,
    bench_name: String,
    function_name: String,
    setup_function: Option<String>,
    transaction_mode: BenchTransactionMode,
    source_file: String,
    source_line: u32,
    criterion_config: BenchConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BenchConfig {
    sample_size: usize,
    measurement_time_ms: u64,
    warm_up_time_ms: u64,
    nresamples: usize,
    noise_threshold: f64,
    significance_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BenchEstimate {
    estimate_kind: String,
    point_estimate_ns: f64,
    standard_error_ns: Option<f64>,
    confidence_level: Option<f64>,
    ci_lower_bound_ns: Option<f64>,
    ci_upper_bound_ns: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BenchSample {
    sample_index: usize,
    iteration_count: u64,
    elapsed_ns: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BenchThroughput {
    kind: String,
    value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BenchComparison {
    mean: BenchComparisonEstimate,
    median: BenchComparisonEstimate,
    p_value: f64,
    significance_level: f64,
    noise_threshold: f64,
    summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BenchComparisonEstimate {
    estimate_kind: String,
    point_estimate: f64,
    standard_error: f64,
    confidence_level: f64,
    ci_lower_bound: f64,
    ci_upper_bound: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BenchArtifact {
    artifact_kind: String,
    media_type: String,
    payload_json: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum BenchTransactionMode {
    Shared,
    SubtransactionPerBatch,
    SubtransactionPerIteration,
}

impl BenchTransactionMode {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Shared => "shared",
            Self::SubtransactionPerBatch => "subtransaction_per_batch",
            Self::SubtransactionPerIteration => "subtransaction_per_iteration",
        }
    }
}

const PERSISTENT_SCHEMA_SQL: &str = r#"
CREATE SCHEMA IF NOT EXISTS pgrx_bench;

CREATE TABLE IF NOT EXISTS pgrx_bench.run_group (
    id uuid PRIMARY KEY,
    group_name text NOT NULL UNIQUE,
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    completed_at timestamptz,
    status text NOT NULL,
    compare_group_id uuid REFERENCES pgrx_bench.run_group(id),
    extname text NOT NULL,
    extversion text,
    pg_version_major integer NOT NULL,
    profile_name text NOT NULL,
    cargo_features text[] NOT NULL DEFAULT ARRAY[]::text[],
    command_line text NOT NULL,
    hostname text,
    os text,
    arch text,
    rustc_version text,
    cargo_version text,
    pgrx_version text,
    cargo_pgrx_version text,
    git_commit text,
    git_branch text,
    git_dirty boolean NOT NULL DEFAULT false,
    git_describe text,
    extra_metadata jsonb NOT NULL DEFAULT '{}'::jsonb
);

CREATE TABLE IF NOT EXISTS pgrx_bench.run_group_pg_setting (
    group_id uuid NOT NULL REFERENCES pgrx_bench.run_group(id) ON DELETE CASCADE,
    name text NOT NULL,
    setting text,
    unit text,
    source text,
    sourcefile text,
    sourceline integer,
    boot_val text,
    reset_val text,
    pending_restart boolean,
    PRIMARY KEY (group_id, name)
);

CREATE TABLE IF NOT EXISTS pgrx_bench.benchmark_case (
    id bigserial PRIMARY KEY,
    schema_name text NOT NULL,
    bench_name text NOT NULL,
    function_name text NOT NULL,
    setup_function text,
    transaction_mode text NOT NULL,
    source_file text,
    source_line integer,
    UNIQUE (schema_name, bench_name)
);

CREATE TABLE IF NOT EXISTS pgrx_bench.benchmark_run (
    id bigserial PRIMARY KEY,
    group_id uuid NOT NULL REFERENCES pgrx_bench.run_group(id) ON DELETE CASCADE,
    case_id bigint NOT NULL REFERENCES pgrx_bench.benchmark_case(id),
    status text NOT NULL,
    error_text text,
    started_at timestamptz NOT NULL,
    finished_at timestamptz,
    criterion_config jsonb NOT NULL,
    raw_result jsonb NOT NULL,
    UNIQUE (group_id, case_id)
);

CREATE TABLE IF NOT EXISTS pgrx_bench.benchmark_estimate (
    benchmark_run_id bigint NOT NULL REFERENCES pgrx_bench.benchmark_run(id) ON DELETE CASCADE,
    estimate_kind text NOT NULL,
    point_estimate_ns double precision NOT NULL,
    standard_error_ns double precision,
    confidence_level double precision,
    ci_lower_bound_ns double precision,
    ci_upper_bound_ns double precision,
    PRIMARY KEY (benchmark_run_id, estimate_kind)
);

CREATE TABLE IF NOT EXISTS pgrx_bench.benchmark_sample (
    benchmark_run_id bigint NOT NULL REFERENCES pgrx_bench.benchmark_run(id) ON DELETE CASCADE,
    sample_index integer NOT NULL,
    iteration_count bigint NOT NULL,
    elapsed_ns double precision NOT NULL,
    PRIMARY KEY (benchmark_run_id, sample_index)
);

CREATE TABLE IF NOT EXISTS pgrx_bench.benchmark_throughput (
    benchmark_run_id bigint PRIMARY KEY REFERENCES pgrx_bench.benchmark_run(id) ON DELETE CASCADE,
    kind text NOT NULL,
    value double precision NOT NULL
);

CREATE TABLE IF NOT EXISTS pgrx_bench.artifact (
    id bigserial PRIMARY KEY,
    benchmark_run_id bigint NOT NULL REFERENCES pgrx_bench.benchmark_run(id) ON DELETE CASCADE,
    artifact_kind text NOT NULL,
    media_type text NOT NULL,
    payload bytea,
    payload_json jsonb,
    metadata jsonb NOT NULL DEFAULT '{}'::jsonb
);

CREATE OR REPLACE VIEW pgrx_bench.v_run_group_summary AS
SELECT
    id,
    group_name,
    created_at,
    completed_at,
    status,
    compare_group_id,
    extname,
    extversion,
    pg_version_major,
    profile_name,
    git_commit,
    git_branch,
    git_dirty,
    git_describe
FROM pgrx_bench.run_group;

CREATE OR REPLACE VIEW pgrx_bench.v_run_group_nondefault_settings AS
SELECT *
FROM pgrx_bench.run_group_pg_setting
WHERE source IS DISTINCT FROM 'default'
   OR sourcefile IS NOT NULL
   OR setting IS DISTINCT FROM boot_val;

CREATE OR REPLACE VIEW pgrx_bench.v_primary_estimate AS
SELECT DISTINCT ON (benchmark_run_id)
    benchmark_run_id,
    estimate_kind,
    point_estimate_ns,
    standard_error_ns,
    confidence_level,
    ci_lower_bound_ns,
    ci_upper_bound_ns
FROM pgrx_bench.benchmark_estimate
ORDER BY
    benchmark_run_id,
    CASE
        WHEN estimate_kind = 'slope' THEN 0
        WHEN estimate_kind = 'mean' THEN 1
        ELSE 2
    END,
    estimate_kind;

CREATE OR REPLACE VIEW pgrx_bench.v_group_results AS
SELECT
    benchmark_run.id AS benchmark_run_id,
    benchmark_run.group_id,
    benchmark_run.case_id,
    benchmark_run.status,
    benchmark_case.schema_name,
    benchmark_case.bench_name,
    benchmark_case.function_name,
    benchmark_case.setup_function,
    benchmark_case.transaction_mode,
    benchmark_case.source_file,
    benchmark_case.source_line,
    primary_estimate.estimate_kind AS primary_estimate_kind,
    primary_estimate.point_estimate_ns
FROM pgrx_bench.benchmark_run
JOIN pgrx_bench.benchmark_case ON benchmark_case.id = benchmark_run.case_id
LEFT JOIN pgrx_bench.v_primary_estimate AS primary_estimate
    ON primary_estimate.benchmark_run_id = benchmark_run.id;

CREATE OR REPLACE VIEW pgrx_bench.v_default_comparison AS
SELECT
    current_group.id AS group_id,
    current_group.group_name,
    current_group.compare_group_id,
    compare_group.group_name AS compare_group_name,
    cases.case_id,
    COALESCE(current_result.schema_name, baseline_result.schema_name) AS schema_name,
    COALESCE(current_result.bench_name, baseline_result.bench_name) AS bench_name,
    current_result.point_estimate_ns AS current_point_estimate_ns,
    baseline_result.point_estimate_ns AS baseline_point_estimate_ns,
    CASE
        WHEN current_result.point_estimate_ns IS NOT NULL
         AND baseline_result.point_estimate_ns IS NOT NULL
         AND baseline_result.point_estimate_ns <> 0
        THEN ((current_result.point_estimate_ns - baseline_result.point_estimate_ns)
              / baseline_result.point_estimate_ns) * 100.0
    END AS delta_pct,
    CASE
        WHEN baseline_result.case_id IS NULL THEN 'new'
        WHEN current_result.case_id IS NULL THEN 'missing'
        WHEN current_result.point_estimate_ns IS NULL OR baseline_result.point_estimate_ns IS NULL
            THEN 'unchanged'
        WHEN current_result.point_estimate_ns < baseline_result.point_estimate_ns THEN 'faster'
        WHEN current_result.point_estimate_ns > baseline_result.point_estimate_ns THEN 'slower'
        ELSE 'unchanged'
    END AS comparison_status
FROM pgrx_bench.run_group AS current_group
LEFT JOIN pgrx_bench.run_group AS compare_group ON compare_group.id = current_group.compare_group_id
LEFT JOIN LATERAL (
    SELECT case_id
    FROM pgrx_bench.v_group_results
    WHERE group_id = current_group.id
    UNION
    SELECT case_id
    FROM pgrx_bench.v_group_results
    WHERE group_id = current_group.compare_group_id
) AS cases ON TRUE
LEFT JOIN pgrx_bench.v_group_results AS current_result
    ON current_result.group_id = current_group.id
   AND current_result.case_id = cases.case_id
LEFT JOIN pgrx_bench.v_group_results AS baseline_result
    ON baseline_result.group_id = current_group.compare_group_id
   AND baseline_result.case_id = cases.case_id;
"#;
