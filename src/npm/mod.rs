use std::{path::PathBuf, process::Command};

use models::ShowPackageInfo;

mod models;

/// List packages that depend on the given package in the current project
/// npm ls <package> --json
pub(crate) fn find_dependant_packages(npm_path: &PathBuf, package_name: &str) -> Vec<String> {
    let output = Command::new(npm_path)
        .arg("ls")
        .arg(package_name)
        .arg("--json")
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8(output.stdout).unwrap();
    let packages: models::NpmPackage = serde_json::from_str(&output).unwrap();
    packages.dependency_names()
}

/// Show Details about the given package
/// npm show <package> --json
pub(crate) fn show_package_info(npm_path: &PathBuf, package_name: &str) -> ShowPackageInfo {
    let output = Command::new(npm_path)
        .arg("show")
        .arg(package_name)
        .arg("--json")
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8(output.stdout).unwrap();
    serde_json::from_str(&output).unwrap()
}
