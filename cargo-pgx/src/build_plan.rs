use eyre::WrapErr;
use std::process::Command;

pub(crate) fn build_plan(features: &clap_cargo::Features, is_release: bool) -> eyre::Result<serde_json::Value> {
    let mut command_build_plan = Command::new("cargo");
    command_build_plan.env("RUSTC_BOOTSTRAP", "1");
    command_build_plan.arg("build");
    command_build_plan.args(["-Z", "unstable-options", "--build-plan" ]);
    if is_release {
        command_build_plan.arg("--release");
    }
    let features_arg = features.features.join(" ");
    if !features_arg.trim().is_empty() {
        command_build_plan.arg("--features");
        command_build_plan.arg(&features_arg);
    }

    if features.no_default_features {
        command_build_plan.arg("--no-default-features");
    }
    if features.all_features {
        command_build_plan.arg("--all-features");
    }
    let build_plan_output = command_build_plan.output()?;

    let build_plan_bytes = build_plan_output.stdout;
    let build_plan: serde_json::Value = serde_json::from_slice(&build_plan_bytes)
        .wrap_err("Could not parse build plan.")?;

    Ok(build_plan)
}