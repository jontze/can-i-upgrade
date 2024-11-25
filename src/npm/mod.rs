use std::{path::PathBuf, process::Command};

use miette::{Context, IntoDiagnostic};
use models::{NpmPackage, ShowPackageInfo};

mod models;

/// List packages that depend on the given package in the current project
/// npm ls <package> --json
pub(crate) fn find_dependant_packages(
    npm_path: &PathBuf,
    package_name: &str,
) -> miette::Result<NpmPackage> {
    let output = Command::new(npm_path)
        .arg("ls")
        .arg(package_name)
        .arg("--json")
        .output()
        .into_diagnostic()
        .wrap_err(format!(
            "Unable to execute '{} ls {package_name} --json'",
            npm_path.display()
        ))?;

    let output = String::from_utf8(output.stdout).unwrap();
    serde_json::from_str(&output).into_diagnostic().wrap_err(
        "Unable to parse project details. Have you run `npm install` in the project directory?",
    )
}

/// Show Details about the given package
/// npm show <package> --json
pub(crate) fn show_package_info(
    npm_path: &PathBuf,
    package_name: &str,
) -> miette::Result<ShowPackageInfo> {
    let output = Command::new(npm_path)
        .arg("show")
        .arg(package_name)
        .arg("--json")
        .output()
        .into_diagnostic()
        .wrap_err(format!(
            "Unable to execute '{} show {package_name} --json'",
            npm_path.display()
        ))?;

    let output = String::from_utf8(output.stdout).unwrap();
    serde_json::from_str(&output)
        .into_diagnostic()
        .wrap_err(format!(
            "Unable to parse package details from remote: {package_name}."
        ))
}
