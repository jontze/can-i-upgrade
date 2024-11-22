use clap::{Parser, Subcommand};
use miette::{bail, Context, IntoDiagnostic, Result};
use node_semver::{Range, Version};
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, fs, path::PathBuf, process::Command};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Check if you can upgrade the package in your project
    CheckDep {
        /// The name of the npm package to check
        package_name: String,
        /// Target Version to check compatibility with
        target_version: String,
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let npm_executable_path = which::which("npm")
        .into_diagnostic()
        .wrap_err("NPM not found")?;

    if !is_npm_project() {
        panic!("Not an npm project");
    }

    match args.command {
        Commands::CheckDep {
            package_name,
            target_version,
        } => {
            let package_json = PackageJson::read();
            let Some(current_version) = package_json.get_dependency_version(&package_name) else {
                bail!("Couldn't find the dependency in your package.json")
            };
            println!(
                "Curren version of {} is {}",
                &package_name, &current_version
            );
            let semver_target_version: Version = target_version
                .parse()
                .into_diagnostic()
                .wrap_err("Invalid target version")?;

            let dependant_packages = find_dependant_packages(&npm_executable_path, &package_name)
                .into_iter()
                .fold(
                    Vec::new() as Vec<(String, String, bool)>,
                    |mut collected, package| {
                        //Remove self reference
                        if package == package_name {
                            return collected;
                        }

                        let package_info = show_package_info(&npm_executable_path, &package);

                        println!(
                            "Checking {} for compatibility with {}",
                            &package, &package_name
                        );
                        let latest_dependant_version =
                            package_info.get_dependency_version(&package_name).unwrap();

                        let Ok(latest_dependant_range) = latest_dependant_version.parse::<Range>()
                        else {
                            panic!("Invalid version range")
                        };

                        let is_compatible =
                            latest_dependant_range.satisfies(&semver_target_version);

                        collected.push((
                            package,
                            package_info.get_dependency_version(&package_name).unwrap(),
                            is_compatible,
                        ));
                        collected
                    },
                );

            for (package, latest_dependant_version, is_compatible) in dependant_packages {
                println!(
                    "{} depends on {}. Latest support is {}. An upgrade is possible: {}",
                    &package, &package_name, latest_dependant_version, is_compatible
                );
            }
        }
    }
    Ok(())
}

/// Check if the current directory is an npm project
/// by checking if package.json exists
fn is_npm_project() -> bool {
    fs::metadata("package.json").is_ok()
}

/// npm ls <package> --json
fn find_dependant_packages(npm_path: &PathBuf, package_name: &str) -> Vec<String> {
    let output = Command::new(npm_path)
        .arg("ls")
        .arg(package_name)
        .arg("--json")
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8(output.stdout).unwrap();
    let packages: NpmPackage = serde_json::from_str(&output).unwrap();
    packages.dependency_names()
}

// npm show <package> --json
fn show_package_info(npm_path: &PathBuf, package_name: &str) -> ShowPackageInfo {
    let output = Command::new(npm_path)
        .arg("show")
        .arg(package_name)
        .arg("--json")
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8(output.stdout).unwrap();
    serde_json::from_str(&output).unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
struct Package {
    version: String,
    resolved: Option<String>,
    overridden: Option<bool>,
    dependencies: Option<HashMap<String, Package>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct NpmPackage {
    version: String,
    name: String,
    dependencies: HashMap<String, Package>,
}

impl NpmPackage {
    fn dependency_names(&self) -> Vec<String> {
        self.dependencies.keys().cloned().collect()
    }
}

// Read Package.json

#[derive(Serialize, Deserialize, Debug)]
struct PackageJson {
    name: Option<String>,
    version: Option<String>,
    description: Option<String>,
    main: Option<String>,
    scripts: Option<HashMap<String, String>>,
    dependencies: Option<HashMap<String, String>>,
    dev_dependencies: Option<HashMap<String, String>>,
    peer_dependencies: Option<HashMap<String, String>>,
    optional_dependencies: Option<HashMap<String, String>>,
    author: Option<String>,
    license: Option<String>,
    repository: Option<Repository>,
    bugs: Option<Bugs>,
    homepage: Option<String>,
}

trait ExtractDependencyInfo {
    fn get_dependency_version(&self, package_name: &str) -> Option<String>;
}

impl PackageJson {
    fn read() -> Self {
        let package_json = fs::read_to_string("package.json").expect("Unable to read package.json");
        serde_json::from_str(&package_json).expect("Unable to parse package.json")
    }
}

impl ExtractDependencyInfo for PackageJson {
    fn get_dependency_version(&self, package_name: &str) -> Option<String> {
        if let Some(dependencies) = &self.dependencies {
            if let Some(version) = dependencies.get(package_name) {
                return Some(version.to_string());
            }
        }
        if let Some(dev_dependencies) = &self.dev_dependencies {
            if let Some(version) = dev_dependencies.get(package_name) {
                return Some(version.to_string());
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Repository {
    r#type: Option<String>,
    url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Bugs {
    url: Option<String>,
    email: Option<String>,
}

// npm show <package> --json

#[derive(Serialize, Deserialize, Debug)]
struct ShowPackageInfo {
    _id: String,
    _rev: String,
    name: String,
    description: String,
    dist_tags: Option<DistTags>,
    versions: Vec<String>,
    time: Time,
    maintainers: Vec<String>,
    author: String,
    repository: Repository,
    license: String,
    homepage: String,
    bugs: Bugs,
    readmeFilename: String,
    _contentLength: Option<u64>,
    version: String,
    peerDependencies: HashMap<String, String>,
    // Apparently sometimes it's an array of strings
    // sideEffects: Option<bool>,
    module: Option<String>,
    typings: String,
    exports: Option<Exports>,
    dependencies: HashMap<String, String>,
    _nodeVersion: String,
    _npmVersion: String,
    dist: Dist,
    directories: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DistTags {
    latest: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Time {
    created: String,
    modified: String,
    #[serde(flatten)]
    versions: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Exports {
    #[serde(rename = "./package.json")]
    package_json: ExportDetail,
    #[serde(rename = ".")]
    root: ExportDetail,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExportDetail {
    types: Option<String>,
    esm2022: Option<String>,
    esm: Option<String>,
    default: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Dist {
    integrity: String,
    shasum: String,
    tarball: String,
    fileCount: u32,
    unpackedSize: u64,
    attestations: Option<Attestations>,
    signatures: Vec<Signature>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Attestations {
    url: String,
    provenance: Provenance,
}

#[derive(Serialize, Deserialize, Debug)]
struct Provenance {
    predicateType: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Signature {
    keyid: String,
    sig: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct NpmOperationalInternal {
    host: String,
    tmp: String,
}

impl ExtractDependencyInfo for ShowPackageInfo {
    fn get_dependency_version(&self, package_name: &str) -> Option<String> {
        if let Some(version) = self.dependencies.get(package_name) {
            return Some(version.to_string());
        }

        if let Some(version) = self.peerDependencies.get(package_name) {
            return Some(version.to_string());
        }
        None
    }
}
