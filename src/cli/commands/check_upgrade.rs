use console::{style, Emoji};
use indicatif::{HumanDuration, ProgressBar};
use miette::{Context, IntoDiagnostic};
use node_semver::{Range, Version};
use prettytable::Table;
use std::{path::PathBuf, time::Instant};

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

pub(crate) fn execute(
    npm_executable_path: &PathBuf,
    package_name: &str,
    target_version: &str,
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
    let dependant_packages = find_dependant_packages(npm_executable_path, package_name);

    // Create the progress bar with the length of the dependant packages
    let progress_bar = ProgressBar::new(dependant_packages.len() as u64 - 1);

    // Collect details for each dependant package
    let dependant_packages = dependant_packages.into_iter().fold(
        Vec::new() as Vec<(String, String, String, bool)>,
        |mut collected, package| {
            //Remove self reference
            if package == package_name {
                return collected;
            }

            let package_info = show_package_info(npm_executable_path, &package);

            // println!("Checking {package}...");
            let latest_dependant_version = package_info
                .get_peer_dependency_version(package_name)
                .unwrap();

            let Ok(latest_dependant_range) = latest_dependant_version.parse::<Range>() else {
                panic!("Invalid version range")
            };

            let is_compatible = latest_dependant_range.satisfies(&semver_target_version);

            progress_bar.inc(1);
            collected.push((
                package,
                package_info.version,
                latest_dependant_version,
                is_compatible,
            ));
            collected
        },
    );

    progress_bar.finish_and_clear();
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
        "Latest Version",
        "Latest Supported Range",
        "Supported"
    ]);

    // Fill the table with data
    for (package, latest_version, latest_dependant_version, is_compatible) in dependant_packages {
        table.add_row(row![
            package,
            latest_version,
            latest_dependant_version,
            if is_compatible {
                CHECK_MARK
            } else {
                CROSS_MARK
            }
        ]);
    }
    // Print the table to stdout
    table.printstd();

    println!("{} Done in {}", SPARKLE, HumanDuration(started.elapsed()));

    Ok(())
}
