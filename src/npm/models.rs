use node_semver::Version;
use std::collections::HashMap;

use crate::node::models::Repository;

/// Part of the response from the command
/// npm ls <package> --json
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Package {
    pub version: String,
    resolved: Option<String>,
    overridden: Option<bool>,
    dependencies: Option<HashMap<String, Package>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct NpmPackage {
    version: String,
    name: String,
    dependencies: HashMap<String, Package>,
}

impl NpmPackage {
    pub(crate) fn dependency_names(&self) -> Vec<String> {
        self.dependencies.keys().cloned().collect()
    }

    pub(crate) fn get_dependency(&self, name: &str) -> Option<&Package> {
        self.dependencies.get(name)
    }
}

/// Part of the response from the command
/// npm show <package> --json
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ShowPackageInfo {
    _id: String,
    _rev: String,
    name: String,
    description: String,
    dist_tags: Option<DistTags>,
    versions: Vec<String>,
    maintainers: Vec<String>,
    author: Option<String>,
    repository: Repository,
    pub(crate) version: String,
    #[serde(rename = "peerDependencies")]
    peer_dependencies: HashMap<String, String>,
    module: Option<String>,
    typings: Option<String>,
    dependencies: HashMap<String, String>,
}

impl ShowPackageInfo {
    pub(crate) fn get_peer_dependency_version(&self, package_name: &str) -> Option<String> {
        if let Some(version) = self.peer_dependencies.get(package_name) {
            return Some(version.to_string());
        }
        None
    }

    pub(crate) fn get_newer_available_versions(
        &self,
        current_version_number: &str,
        only_stable: bool,
    ) -> Vec<Version> {
        self.versions
            .iter()
            .filter(|version| {
                let version = node_semver::Version::parse(version).unwrap();
                let current_version = node_semver::Version::parse(current_version_number).unwrap();
                version > current_version
            })
            .map(|version| {
                let Ok(parsed_version) = version.parse::<Version>() else {
                    panic!("Invalid version: {version}")
                };
                parsed_version
            })
            .filter(|v| {
                if only_stable {
                    !v.is_prerelease()
                } else {
                    true
                }
            })
            .collect()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DistTags {
    latest: String,
}
