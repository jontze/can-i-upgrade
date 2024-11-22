use std::collections::HashMap;

/// Represents the content a package.json file.
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PackageJson {
    name: Option<String>,
    version: Option<String>,
    description: Option<String>,
    main: Option<String>,
    scripts: Option<HashMap<String, String>>,
    dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "devDependencies")]
    dev_dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "peerDependencies")]
    peer_dependencies: Option<HashMap<String, String>>,
    author: Option<String>,
    license: Option<String>,
    repository: Option<Repository>,
    bugs: Option<Bugs>,
    homepage: Option<String>,
}

impl PackageJson {
    pub(crate) fn read() -> Self {
        let package_json =
            std::fs::read_to_string("package.json").expect("Unable to read package.json");
        serde_json::from_str(&package_json).expect("Unable to parse package.json")
    }

    pub(crate) fn get_dependency_version(&self, package_name: &str) -> Option<String> {
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

/// Represents the repository details in package.json.
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Repository {
    r#type: Option<String>,
    url: Option<String>,
}

/// Represents the bugs reporting infos in package.json.
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Bugs {
    url: Option<String>,
    email: Option<String>,
}
