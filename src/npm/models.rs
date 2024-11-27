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
    dependencies: Option<HashMap<String, Package>>,
}

impl NpmPackage {
    pub(crate) fn dependency_names(&self) -> Vec<String> {
        self.dependencies
            .as_ref()
            .map(|deps| deps.keys().cloned().collect::<Vec<String>>())
            .unwrap_or(vec![])
    }

    pub(crate) fn get_dependency(&self, name: &str) -> Option<&Package> {
        self.dependencies.as_ref().and_then(|deps| deps.get(name))
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
    maintainers: Option<Vec<String>>,
    author: Option<String>,
    repository: Option<Repository>,
    pub(crate) version: String,
    #[serde(rename = "peerDependencies")]
    peer_dependencies: Option<HashMap<String, String>>,
    module: Option<String>,
    typings: Option<String>,
    dependencies: Option<HashMap<String, String>>,
}

impl ShowPackageInfo {
    pub(crate) fn get_peer_dependency_version(&self, package_name: &str) -> Option<String> {
        self.peer_dependencies
            .as_ref()
            .and_then(|peer_deps| peer_deps.get(package_name))
            .map(|version| version.to_string())
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
