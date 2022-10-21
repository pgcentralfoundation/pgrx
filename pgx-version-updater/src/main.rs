use clap::Parser;
use owo_colors::OwoColorize;
use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::{env, path::PathBuf};
use toml_edit::{value, Document};
use walkdir::{DirEntry, WalkDir};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Additional Cargo.toml file to include for processing that can't be detected automatically
    ///
    /// Add multiple values using --include /path/foo/Cargo.toml --include /path/bar/Cargo.toml
    #[clap(short, long)]
    include_for_dep_updates: Vec<String>,

    /// Exclude Cargo.toml files from [package] version updates
    ///
    /// Add multiple values using --exclude /path/foo/Cargo.toml --exclude /path/bar/Cargo.toml
    #[clap(short, long)]
    exclude_from_version_change: Vec<String>,

    /// Version to be used in all updates
    #[clap(short, long, required = true)]
    update_version: String,

    /// Do not make any changes to files
    #[clap(short, long)]
    dry_run: bool,

    /// Output diff between existing file and changes to be made
    #[clap(short, long)]
    show_diff: bool,

    /// Be verbose in output
    #[clap(short, long)]
    verbose: bool,
}

// List of directories to ignore while Walkdir'ing. Add more here as necessary.
const IGNORE_DIRS: &'static [&'static str] = &[".git", "target"];

fn main() {
    let args = Args::parse();
    let current_dir = env::current_dir().expect("Could not get current directory!");

    // Contains a set of package names (e.g. "pgx", "pgx-pg-sys") that wil be used
    // to search for updatable dependencies later on
    let mut updatable_package_names = HashSet::new();

    // This will eventually contain every file we want to process
    let mut files_to_process = HashSet::new();

    // Keep track of which files to exclude from a "package version" change.
    // For example, some Cargo.toml files do not need this updated:
    //   [package]
    //   version = "0.1.0"
    //   ...
    // Any such file is explicitly added via a command line argument.
    // Note that any files included here are still eligible to be processed for
    // *dependency* version updates.
    let mut exclude_version_files = HashSet::new();
    for file in args.exclude_from_version_change {
        exclude_version_files.insert(
            fullpath(&file).expect(format!("Could not get full path for file: {}", file).as_str()),
        );
    }

    // Recursively walk down all directories to extract out any existing Cargo.toml files
    for entry in WalkDir::new(&current_dir)
        .into_iter()
        .filter_entry(|e| is_not_excluded_dir(e))
        .filter_map(|v| v.ok())
    {
        if is_cargo_toml_file(&entry) {
            let filepath = fullpath(entry.path()).expect(
                format!("Could not get full path for file {}", entry.path().display()).as_str(),
            );

            let mut output = format!(
                "{} Cargo.toml file at {}",
                "Discovered".bold().green(),
                &filepath.display().cyan()
            );

            // Extract the package name if possible
            if !exclude_version_files.contains(&filepath) {
                match extract_package_name(&filepath) {
                    Some(package_name) => {
                        updatable_package_names.insert(package_name);
                    }
                    None => {
                        output.push_str(
                            "\n           * Could not determine package name due to [package] not existing -- skipping version bump."
                                .dimmed()
                                .to_string()
                                .as_str(),
                        )
                    }
                }
            }

            if args.verbose {
                println!("{output}");
            }

            files_to_process.insert(filepath.clone());
        }
    }

    // Loop through all files that are included for dependency updates via CLI params
    for file in args.include_for_dep_updates {
        let filepath =
            fullpath(&file).expect(format!("Could not get full path for file {}", file).as_str());

        let mut output = format!(
            "{} Cargo.toml file at {} for processing",
            " Including".bold().green(),
            &filepath.display().cyan()
        );

        // Extract the package name if possible
        if !exclude_version_files.contains(&filepath) {
            match extract_package_name(&filepath) {
                Some(package_name) => {
                    updatable_package_names.insert(package_name);
                }
                None => {
                    output.push_str(
                        "\n           * Could not determine package name due to [package] not existing -- skipping version bump."
                            .dimmed()
                            .to_string()
                            .as_str(),
                    )
                }
            }
        }

        if args.verbose {
            println!("{output}");
        }

        files_to_process.insert(filepath.clone());
    }

    // Print out information about package names that were automatically discovered
    // and parsed
    for package_name in &updatable_package_names {
        println!(
            "{} {} found for version updating",
            "   Package".bold().green(),
            package_name.cyan()
        );
    }

    // Loop through every TOML file (automatically discovered and manually included
    // via command line params) and update package versions and dependency
    // versions where applicable
    for filepath in files_to_process {
        let mut output = format!(
            "{} Cargo.toml file at {}",
            "Processing".bold().green(),
            &filepath.display().cyan()
        );

        let data = fs::read_to_string(&filepath)
            .expect(format!("Unable to open file at {}", &filepath.display()).as_str());

        let mut doc = data.parse::<Document>().expect(
            format!("File at location {} is an invalid Cargo.toml file", &filepath.display())
                .as_str(),
        );

        if exclude_version_files.contains(&filepath) {
            output.push_str(
                "\n           * Excluding from package version bump due to command line parameter"
                    .dimmed()
                    .to_string()
                    .as_str(),
            )
        } else {
            // Bump package version if we can
            if let Some(package_version) = doc.get_mut("package").and_then(|p| p.get_mut("version"))
            {
                *package_version = value(args.update_version.clone());
            }
        }

        // Process dependencies in each file. Generally dependencies can be found in
        // [dependencies], [dependencies.foo], [build-dependencies], [dev-dependencies]
        for updatable_table_name in ["dependencies", "build-dependencies", "dev-dependencies"] {
            if let Some(updatable_table) =
                doc.get_mut(updatable_table_name).and_then(|i| i.as_table_mut())
            {
                for package in &updatable_package_names {
                    match updatable_table.get_mut(package) {
                        // Tables can contain other tables, and if that's the case we're
                        // probably at a case of:
                        //   [dependencies.pgx]
                        //   version = "1.2.3"
                        Some(item) if item.is_table() => {
                            if let Some(current_version_specifier) =
                                item.get("version").and_then(|a| a.as_str())
                            {
                                item["version"] = value(parse_new_version(
                                    current_version_specifier,
                                    &args.update_version.as_str(),
                                ))
                            }
                        }

                        // Inline table covers the case of:
                        //   [dependencies]
                        //   pgx = { version = "1.2.3", features = ["..."] }
                        Some(item) if item.is_inline_table() => {
                            if let Some(current_version_specifier) = item
                                .as_inline_table()
                                .and_then(|i| i.get("version"))
                                .and_then(|v| v.as_str())
                            {
                                updatable_table[package]["version"] = value(parse_new_version(
                                    current_version_specifier,
                                    &args.update_version.as_str(),
                                ))
                            }
                        }

                        // Otherwise we are a string, such as:
                        //   [dependencies]
                        //   pgx = "0.1.2"
                        Some(item) => {
                            if let Some(current_version_specifier) = item.as_str() {
                                updatable_table[package] = value(parse_new_version(
                                    current_version_specifier,
                                    &args.update_version.as_str(),
                                ))
                            }
                        }

                        // Don't care!
                        None => {}
                    }
                }
            }
        }

        if args.show_diff {
            // Call diff command, it provides the easiest way to show context.
            let mut child = Command::new("diff")
                .arg(&filepath)
                .arg("-U")
                .arg("5")
                .arg("-")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to spawn child process");

            let mut stdin = child.stdin.take().expect("Failed to open stdin");
            let docstring = doc.to_string();

            std::thread::spawn(move || {
                stdin.write_all(docstring.as_bytes()).expect("Failed to write to stdin");
            });

            let child_output = child.wait_with_output().expect("Failed to read stdout");

            // Loop through all lines of the diff command output, if any. First 2 lines
            // from the diff output above will produce irrelevant information, so we
            // will skip it.
            let mut diff_output = String::new();
            for output_line in child_output.stdout.lines().skip(2) {
                if let Ok(line) = output_line {
                    match line.chars().nth(0) {
                        Some('-') => {
                            diff_output.push_str(format!("\n            {}", line.red()).as_str())
                        }
                        Some('+') => {
                            diff_output.push_str(format!("\n            {}", line.green()).as_str())
                        }
                        Some(_) => diff_output.push_str(format!("\n           {line}").as_str()),
                        _ => {}
                    }
                }
            }

            // The "diff" command will not print out anything if there is no difference.
            if diff_output.is_empty() {
                diff_output.push_str(
                    format!("\n           {}", "* No detectable diff found".dimmed()).as_str(),
                )
            } else {
                diff_output = format!("\n           {}", "* Diff:".dimmed()) + diff_output.as_str();
            }

            output.push_str(diff_output.as_str());
        }

        println!("{output}");

        // Write it out!
        if !args.dry_run {
            fs::write(filepath, doc.to_string()).expect("Unable to write file");
        }
    }
}

// Always return full path
fn fullpath<P: AsRef<Path>>(test_path: P) -> Result<PathBuf, std::io::Error> {
    match test_path.as_ref() {
        path if path.is_absolute() => Ok(PathBuf::from(path)),
        path => {
            let mut current_dir = env::current_dir()?;
            current_dir.push(path);
            current_dir.canonicalize()?;
            Ok(current_dir)
        }
    }
}

// Walkdir filter, ensure we don't traverse down a directory that should be ignored
// e.g. .git/ and target/ directories should never be traversed.
fn is_not_excluded_dir(entry: &DirEntry) -> bool {
    let metadata = entry.metadata().expect(
        format!("Could not get metadata for: {}", entry.file_name().to_string_lossy()).as_str(),
    );

    if metadata.is_dir() {
        return !IGNORE_DIRS.contains(&entry.file_name().to_string_lossy().as_ref());
    }

    true
}

// Check if a specific DirEntry is named "Cargo.toml"
fn is_cargo_toml_file(entry: &DirEntry) -> bool {
    let metadata = entry.metadata().expect(
        format!("Could not get metadata for: {}", entry.file_name().to_string_lossy()).as_str(),
    );

    if metadata.is_file() {
        return entry.file_name().eq_ignore_ascii_case("Cargo.toml");
    }

    false
}

// Replace old version specifier with new updated version.
// For example, if this line exists in a Cargo.toml file somewhere:
//   pgx = "=1.2.3"
// and the new version is meant to be:
//   "1.3.0"
// return the new version specifier as:
//   "=1.3.0"
// so that the resulting line in the Cargo.toml file will be:
//   pgx = "=1.3.0"
// It was necessary to keep the requirements specifications, such as "=" or "~".
// The assumption here is that versions (sans requirement specifier) will always
// start with a number.
fn parse_new_version(current_version_specifier: &str, new_version: &str) -> String {
    let mut result = String::new();

    match current_version_specifier.chars().nth(0) {
        // If first character is numeric, then we have just a version specified,
        // such as "0.5.2" or "4.15.0"
        Some(c) if c.is_numeric() => result.push_str(current_version_specifier),

        // Otherwise, we have a specifier such as "=0.5.2" or "~0.4.6" or ">= 1.2.0"
        // Extract out the non-numeric prefix and join it with the new version to
        // be used. e.g. "=0.5.2" to new version "0.5.4" would result in "=0.5.4"
        // TODO: This does not currently handle any specifiers with wildcards,
        // such as "1.*"
        Some(_) => {
            if let Some(version_pos) = current_version_specifier.find(|c: char| c.is_numeric()) {
                result.push_str(&current_version_specifier[..version_pos]);
                result.push_str(&new_version.clone());
            } else {
                panic!(
                    "Could not find an actual version in specifier: '{}'",
                    current_version_specifier
                );
            }
        }
        None => panic!("Version specifier '{}' is not valid!", current_version_specifier),
    }

    result
}

// Given a filepath pointing to a Cargo.toml file, extract out the [package] name
// if it has one
fn extract_package_name<P: AsRef<Path>>(filepath: P) -> Option<String> {
    let filepath = filepath.as_ref();

    let data = fs::read_to_string(&filepath)
        .expect(format!("Unable to open file at {}", &filepath.display()).as_str());

    let doc = data.parse::<Document>().expect(
        format!("File at location {} is an invalid Cargo.toml file", &filepath.display()).as_str(),
    );

    doc.get("package")?.as_table()?.get("name")?.as_str().map(|s| s.to_string())
}
