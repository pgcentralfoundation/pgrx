use crate::commands::{
    get::find_control_file,
};
use pgx_utils::pg_config::PgConfig;
use std::process::{Command, Stdio};
use crate::commands::get::get_property;
use pgx_utils::{exit_with_error, handle_result};

pub(crate) fn generate_schema(
    pg_config: &PgConfig,
    is_release: bool,
    additional_features: &[&str],
    path: impl AsRef<std::path::Path>,
    dot: Option<impl AsRef<std::path::Path>>,
) -> Result<(), std::io::Error> {
    // TODO: Ensure a `src/bin/sql_generator.rs` exists and is up to date.
    let (control_file, _extname) = find_control_file();
    let major_version = pg_config.major_version()?;

    if get_property("relocatable") != Some("false".into()) {
        exit_with_error!(
            "{}:  The `relocatable` property MUST be `false`.  Please update your .control file.",
            control_file.display()
        )
    }

    let mut features =
        std::env::var("PGX_BUILD_FEATURES").unwrap_or(format!("pg{}", major_version));
    let flags = std::env::var("PGX_BUILD_FLAGS").unwrap_or_default();
    if !additional_features.is_empty() {
        use std::fmt::Write;
        let mut additional_features = additional_features.join(" ");
        let _ = write!(&mut additional_features, " {}", features);
        features = additional_features
    }
    let mut command = Command::new("cargo");
    command.args(&["run", "--bin", "sql-generator"]);
    if is_release {
        command.arg("--release");
    }

    if !features.trim().is_empty() {
        command.arg("--features");
        command.arg(&features);
        command.arg("--no-default-features");
    }

    for arg in flags.split_ascii_whitespace() {
        command.arg(arg);
    }

    let path = path.as_ref();
    let _ = path.parent().map(|p| std::fs::create_dir_all(&p).unwrap());
    command.arg("--");
    command.arg(path);
    if let Some(dot) = dot {
        command.arg(dot);
    }

    let command = command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    let command_str = format!("{:?}", command);
    println!(
        "running SQL generator features `{}`\n{}",
        features, command_str
    );
    let status = handle_result!(
        command.status(),
        format!("failed to spawn cargo: {}", command_str)
    );
    if !status.success() {
        exit_with_error!("failed to run SQL generator");
    }
    Ok(())
}
