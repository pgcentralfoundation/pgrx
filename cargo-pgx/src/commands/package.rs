use crate::commands::get::get_property;
use crate::commands::install::install_extension;
use pgx_utils::{get_pg_config_major_version, get_target_dir, handle_result};
use std::path::PathBuf;

pub(crate) fn package_extension(pg_config: &Option<String>, is_debug: bool) {
    let base_path = build_base_path(pg_config, is_debug);

    if base_path.exists() {
        handle_result!(
            format!(
                "failed to remove existing directory: `{}`",
                base_path.display()
            ),
            std::fs::remove_dir_all(&base_path)
        );
    }

    if !base_path.exists() {
        handle_result!(
            "failed to create package directory",
            std::fs::create_dir_all(&base_path)
        )
    }
    install_extension(pg_config, !is_debug, Some(base_path));
}

fn build_base_path(pg_config: &Option<String>, is_debug: bool) -> PathBuf {
    let mut target_dir = get_target_dir();
    let pgver = get_pg_config_major_version(pg_config);
    let extname = get_property("extname").expect("could not determine extension name");
    target_dir.push(if is_debug { "debug" } else { "release" });
    target_dir.push(format!("{}-pg{}", extname, pgver));
    target_dir
}
