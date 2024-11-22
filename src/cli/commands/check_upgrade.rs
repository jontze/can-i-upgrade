use miette::{Context, IntoDiagnostic};
use node_semver::{Range, Version};
use std::path::PathBuf;

use prettytable::Table;

use crate::{
    node::models::PackageJson,
    npm::{find_dependant_packages, show_package_info},
};

pub(crate) fn execute(
    npm_executable_path: &PathBuf,
    package_name: &str,
    target_version: &str,
) -> miette::Result<()> {
    let package_json = PackageJson::read();
    let Some(current_version) = package_json.get_dependency_version(package_name) else {
        bail!("Couldn't find the dependency in your package.json")
    };
    println!("Current version of {package_name} is {current_version}");
    let semver_target_version: Version = target_version
        .parse()
        .into_diagnostic()
        .wrap_err("Invalid target version")?;

    let dependant_packages = find_dependant_packages(npm_executable_path, package_name)
        .into_iter()
        .fold(
            Vec::new() as Vec<(String, String, String, bool)>,
            |mut collected, package| {
                //Remove self reference
                if package == package_name {
                    return collected;
                }

                let package_info = show_package_info(npm_executable_path, &package);

                println!("Checking {package}...");
                let latest_dependant_version = package_info
                    .get_peer_dependency_version(package_name)
                    .unwrap();

                let Ok(latest_dependant_range) = latest_dependant_version.parse::<Range>() else {
                    panic!("Invalid version range")
                };

                let is_compatible = latest_dependant_range.satisfies(&semver_target_version);

                collected.push((
                    package,
                    package_info.version,
                    latest_dependant_version,
                    is_compatible,
                ));
                collected
            },
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
            is_compatible
        ]);
    }
    // Print the table to stdout
    table.printstd();

    Ok(())
}
