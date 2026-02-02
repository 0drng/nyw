use sqlx::{Pool, Sqlite};

use crate::{
    error::ApplicationError, model::{
        application::Application, config_file::ConfigFile, dot_config::DotConfig, lock::{Lock, LockAdd}, package::Package, package_manager::{PackageManager, PackageManagerEnum}, script::Script
    }, repository::{application_repository, lock_repository}, service::{
        application_service::{self, get_applications_to_install}, command_service, config_service, file_service, language_service::Labels
    }
};

mod environment;
mod error;
mod model;
mod repository;
mod service;

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    // Getting all data needed in further process
    let config_file: ConfigFile = 
        config_service::get_merged_config(file_service::get_platform_specific_path());
    let pool: Pool<Sqlite> = repository::db::get_pool().await?;

    let all_packages = config_file.get_packages();
    let packages: Vec<Package> = all_packages.iter().filter(|p| !p.is_aur()).cloned().collect();
    let packages_aur: Vec<Package> = all_packages.iter().filter(|p| p.is_aur()).cloned().collect();

    let packages_str: Vec<String> = packages.iter().map(Package::get_package_name).collect();
    let packages_aur_str: Vec<String> = packages_aur.iter().map(Package::get_package_name).collect();

    let pre_scripts: Vec<Script> = packages
        .iter()
        .filter_map(|p| p.get_pre_scripts())
        .flatten()
        .collect();

    let dot_files: Vec<DotConfig> = packages.iter().filter_map(|f| f.get_dot_configs()).flat_map(|f| f).collect();

    let post_scripts: Vec<Script> = packages
        .iter()
        .filter_map(|p| p.get_post_scripts())
        .flatten()
        .collect();

    for script_path in pre_scripts {
        command_service::run_command(
            "sh",
            vec![script_path.bin],
            Labels::Info_ExecutingPostScript,
        )?;
    }

    if !packages_aur_str.is_empty() {
        PackageManagerEnum::get_aur_helper()?;
    }

    let package_manager: PackageManager = PackageManager::new(PackageManagerEnum::get_package_manager()?)?;
    let package_manager_aur: PackageManager = PackageManager::new(PackageManagerEnum::get_aur_helper()?)?;

    if !packages_str.is_empty() {
        package_manager.install_packages(
            get_applications_to_install(packages_str.clone())?,
            true,
        )?;
    }

    if !packages_aur_str.is_empty() {
        package_manager_aur.install_packages(
            get_applications_to_install(packages_aur_str.clone())?,
            true,
        )?;
    }

    let packages_to_uninstall: Vec<String> = application_service::get_applications_to_remove(&pool, packages_str).await?;
    if !packages_to_uninstall.is_empty() {
        package_manager.uninstall_packages(
            packages_to_uninstall,
            true,
        )?;
    }

    for dot_file in dot_files {
        if let Some(src) = dot_file.src {
            file_service::copy_file(&src, &dot_file.dest).await?;
            continue;
        }

        if let Some(content) = dot_file.content {
            file_service::write_file(&content, &dot_file.dest)?;
            continue;
        }
    }

    for script_path in post_scripts {
        command_service::run_command(
            "sh",
            vec![script_path.bin],
            Labels::Info_ExecutingPostScript,
        )?;
    }

    let lock: Lock = lock_repository::save_lock(&pool, LockAdd { hash: "dynamic_hash_placeholder".to_owned() }).await?;
    application_repository::save_applications(&pool, packages.iter().map(|p| Application::new(lock.id, p.get_package_name(), "INSTALL".to_owned())).collect()).await?;

    Ok(())
}