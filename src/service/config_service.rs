use std::fs::{create_dir_all, read_dir};
use crate::{model::config_file::ConfigFile, service::{file_service, language_service::{t, Labels}}};

use std::path::Path;

pub fn get_merged_config(path: &str) -> ConfigFile {
    let configs: Vec<ConfigFile> = get_configs(path);
    return ConfigFile::new(configs.iter().flat_map(ConfigFile::get_packages).collect());
}

pub fn get_configs(path: &str) -> Vec<ConfigFile> {
    let mut configs: Vec<ConfigFile> = Vec::new();

    let path_obj = Path::new(path);
    if !path_obj.exists() {
        if let Err(e) = create_dir_all(path_obj) {
            println!("Failed to create directory: {}", e);
            return configs;
        }
    }

    // Proceed with reading the directory
    for dir_entry in read_dir(path).unwrap() {
        let dir_entry_path = dir_entry.unwrap().path();
        let dir_entry_path_str: &str = dir_entry_path.to_str().unwrap();

        if dir_entry_path.is_dir() {
            configs.append(&mut get_configs(dir_entry_path_str));
        }

        if dir_entry_path.is_file() {
            if let Some(ext) = dir_entry_path.extension() {
                if ext == "jsonc" {
                    configs.push(get_config(dir_entry_path_str));
                }
            }
        }
    }

    return configs;
}


pub fn get_config(src: &str) -> ConfigFile {
    let content: String = file_service::read_file(src).expect(&format!(
        "{}",
        t(Labels::Error_FileOpenFailed, Some(vec![src.to_owned()]))
    ));

    return serde_jsonc::from_str(&content).unwrap();
}