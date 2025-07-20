use serde::{Deserialize};

use crate::model::{dot_config::DotConfig, script::Script};

static DEFAULT_ENABLED: bool = false;
static DEFAULT_AUR: bool = false;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Package {
    Simple(String),
    Complex {
        name: String,
        enabled: Option<bool>,
        aur: Option<bool>,
        dot_configs: Option<Vec<DotConfig>>,
        pre_script: Option<Vec<Script>>,
        post_script: Option<Vec<Script>>,
    },
}

impl Package {
    pub fn get_package_name(&self) -> String {
        match &self {
            Package::Simple(name) => return name.to_owned(),
            Package::Complex { name, enabled: _, aur: _, dot_configs: _, pre_script: _, post_script: _ } => return name.to_owned(),
        }
    }

    pub fn is_enabled(&self) -> bool {
        match &self {
            Package::Simple(_) => return DEFAULT_ENABLED,
            Package::Complex { name: _, enabled, aur: _, dot_configs: _, pre_script: _, post_script: _ } => return enabled.unwrap_or(DEFAULT_ENABLED),
        }
    }

    pub fn is_aur(&self) -> bool {
        match &self {
            Package::Simple(_) => return DEFAULT_AUR,
            Package::Complex { name: _, enabled: _, aur, dot_configs: _, pre_script: _, post_script: _ } => return aur.unwrap_or(DEFAULT_AUR),
        }
    }

    pub fn get_dot_configs(&self) -> Option<Vec<DotConfig>> {
        match &self {
            Package::Simple(_) => return Option::None,
            Package::Complex { name: _, enabled: _, aur: _, dot_configs, pre_script: _, post_script: _ } => return dot_configs.to_owned(),
        }
    }

    pub fn get_pre_scripts(&self) -> Option<Vec<Script>> {
        match &self {
            Package::Simple(_) => return Option::None,
            Package::Complex { name: _, enabled: _, aur: _, dot_configs: _, pre_script, post_script: _ } => return pre_script.to_owned(),
        }
    }

    pub fn get_post_scripts(&self) -> Option<Vec<Script>> {
        match &self {
            Package::Simple(_name) => return Option::None,
            Package::Complex { name: _, enabled: _, aur: _, dot_configs: _, pre_script: _, post_script } => return post_script.to_owned(),
        }
    }
}