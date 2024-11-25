use console::{style, Emoji};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use miette::{Context, IntoDiagnostic};
use node_semver::{Range, Version};
use prettytable::Table;
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use crate::{
    node::models::PackageJson,
    npm::{find_dependant_packages, show_package_info},
};

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");
static CHECK_MARK: Emoji<'_, '_> = Emoji("✅ ", "y");
static CROSS_MARK: Emoji<'_, '_> = Emoji("❌ ", "x");

struct DependencyInfo {
    name: String,
    current_version: Version,
    newer_versions: Vec<Version>,
    newer_compatible_versions: Vec<Version>,
}

impl DependencyInfo {
    fn new(name: &str, current_version: Version, newer_versions: &[Version]) -> Self {
        Self {
            name: name.to_owned(),
            current_version,
            newer_versions: newer_versions.to_owned(),
            newer_compatible_versions: Vec::new(),
        }
    }

    /// Collects a version that is compatible
    fn add_compatible_version(&mut self, version: &Version) {
        self.newer_compatible_versions.push(version.to_owned());
    }

    /// Report if there is at all a compatible version
    fn is_compatible(&self) -> bool {
        !self.newer_compatible_versions.is_empty()
    }

    /// Take the first and the last compatible version and construct a range
    fn compatible_version_range(&self) -> Option<node_semver::Range> {
        if self.newer_compatible_versions.is_empty() {
            return None;
        }

        let first_version = &self.newer_compatible_versions[0];
        let last_version =
            &self.newer_compatible_versions[self.newer_compatible_versions.len() - 1];

        let range_str = format!(">= {} <= {}", first_version, last_version);
        let Ok(range) = range_str.parse::<node_semver::Range>() else {
            panic!("Invalid version range")
        };
        Some(range)
    }

    /// Returns the latest released version
    fn latest_version(&self) -> Option<&Version> {
        self.newer_versions.last()
    }
}

pub(crate) fn execute(
    npm_executable_path: &PathBuf,
    package_name: &str,
    target_version: &str,
    ignore_glob_patterns: Vec<String>,
    stable: bool,
) -> miette::Result<()> {
    let started = Instant::now();
    println!(
        "{} {}Analyze project...",
        style("[1/3]").bold().dim(),
        LOOKING_GLASS
    );
    let package_json = PackageJson::read();

    let Some(_) = package_json.get_dependency_version(package_name) else {
        bail!("Couldn't find the dependency in your package.json")
    };

    let semver_target_version: Version = target_version
        .parse()
        .into_diagnostic()
        .wrap_err("Invalid target version")?;

    println!(
        "{} {}Collect information of affected dependencies...",
        style("[2/3]").bold().dim(),
        TRUCK
    );
    let dependant_package_details = find_dependant_packages(npm_executable_path, package_name);
    let mut affected_dependency_names = dependant_package_details.dependency_names();

    // Filter out ignored dependencies
    let glob_set = ignore_glob_patterns
        .iter()
        .fold(globset::GlobSetBuilder::new(), |mut builder, pattern| {
            builder.add(globset::Glob::new(pattern).unwrap());
            builder
        })
        .build()
        .into_diagnostic()
        .wrap_err("Invalid ignore glob pattern provided")?;
    affected_dependency_names.retain(|name| !glob_set.is_match(name));

    // Construct Multi Progress Bar to maintain all following progress bars
    let multi_progress_bar = MultiProgress::new();

    // Collect details for each dependant package
    let mut dependant_packages = affected_dependency_names.into_iter().fold(
        Vec::new() as Vec<DependencyInfo>,
        |mut collected, package| {
            // Remove self reference
            if package == package_name {
                return collected;
            }
            // Create the Progress Spinner for the package
            let spinner_style =
                ProgressStyle::with_template("{spinner} {msg} [{elapsed}]").unwrap();
            let spinner = multi_progress_bar.add(
                ProgressBar::new_spinner()
                    .with_style(spinner_style)
                    .with_message(package.clone())
                    .with_elapsed(Duration::from_secs(1)),
            );
            spinner.enable_steady_tick(Duration::from_millis(50));

            // Fetch Package Details from Remote
            let remote_package_details = show_package_info(npm_executable_path, &package);
            let current_local_version = &dependant_package_details
                .get_dependency(&package)
                .unwrap()
                .version;
            let mut newer_available_versions =
                remote_package_details.get_newer_available_versions(current_local_version, stable);
            // Add the current version to the list of newer versions
            newer_available_versions.insert(0, current_local_version.parse::<Version>().unwrap());

            // Create the progress bar for the package based on the amount of versions to process
            let progress_bar =
                multi_progress_bar.add(ProgressBar::new(newer_available_versions.len() as u64));

            let mut dependency_info = DependencyInfo::new(
                &package,
                current_local_version.parse::<Version>().unwrap(),
                &newer_available_versions,
            );

            // Loop over each version and check if the it's compatible with the desired target version
            for newer_version in newer_available_versions {
                spinner.set_message(format!("{package}@{newer_version}"));
                let remote_package_version_details =
                    show_package_info(npm_executable_path, &format!("{package}@{newer_version}"));

                let peer_dependency_range = remote_package_version_details
                    .get_peer_dependency_version(package_name)
                    .map(|range_str| {
                        let Ok(range) = range_str.parse::<Range>() else {
                            panic!("Invalid version range")
                        };
                        range
                    })
                    .unwrap();

                let is_compatible = peer_dependency_range.satisfies(&semver_target_version);

                if is_compatible {
                    dependency_info.add_compatible_version(&newer_version);
                }
                progress_bar.inc(1);
            }

            // Clear Progress bar and spinner for the loop
            progress_bar.finish_and_clear();
            spinner.finish_and_clear();

            // Report results of the loop
            collected.push(dependency_info);
            collected
        },
    );

    //progress_bar.finish_and_clear();
    println!(
        "{} {}Summarize infos...",
        style("[3/3]").bold().dim(),
        PAPER,
    );

    // Create the table
    let mut table = Table::new();
    // Add Header
    table.add_row(row![
        "Package",
        "Current",
        "Latest",
        "Supported Range",
        "Supported"
    ]);

    // Sort the packages by  name alphabetically
    dependant_packages.sort_by(|a, b| a.name.cmp(&b.name));

    // Fill the table with data
    dependant_packages.iter().for_each(|package| {
        table.add_row(row![
            package.name,
            package.current_version,
            package.latest_version().unwrap_or(&package.current_version),
            if let Some(range) = package.compatible_version_range() {
                range.to_string()
            } else {
                "/".to_string()
            },
            if package.is_compatible() {
                CHECK_MARK
            } else {
                CROSS_MARK
            }
        ]);
    });

    // Print the table to stdout
    table.printstd();

    println!("{} Done in {}", SPARKLE, HumanDuration(started.elapsed()));

    Ok(())
}
