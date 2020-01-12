use std::process::{Command, Stdio};

pub fn test_extension(version: &str) -> Result<(), std::io::Error> {
    let versions = if version == "all" {
        vec![
            "pgx/pg10".to_string(),
            "pgx/pg11".to_string(),
            "pgx/pg12".to_string(),
        ]
    } else {
        let version = format!("pgx/{}", version);
        vec![version]
    };

    for version in versions {
        let result = Command::new("cargo")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .arg("test")
            .arg("--target-dir")
            .arg(std::env::var("CARGO_TARGET_DIR").unwrap_or("target".to_string()))
            .arg("--all")
            .arg("--features")
            .arg(version)
            .arg("--no-default-features")
            .env("RUST_BACKTRACE", "1")
            .status();

        if !result.is_ok() {
            return Err(result.err().unwrap());
        }
    }

    Ok(())
}
