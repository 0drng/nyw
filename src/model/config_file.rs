use serde::Deserialize;

use crate::model::package::Package;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigFile {
    // imports: Vec<String>,
    packages: Vec<Package>,
}

impl ConfigFile {

    pub fn new(packages: Vec<Package>) -> Self {
        return Self {
            packages,
        }
    }

    pub fn get_packages(&self) -> Vec<Package> {
        return self.packages.clone()
    }

    pub fn set_packages(&mut self, packages: Vec<Package>) {
        self.packages = packages;
    }
}