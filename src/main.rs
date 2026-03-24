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

/// Helper function to filter packages by AUR status.
fn filter_packages_by_aur(packages: Vec<Package>, is_aur: bool) -> Vec<Package> {
    packages.into_iter().filter(|p| p.is_aur() == is_aur).collect()
}

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    // Program requires root access to run packagemanager
    file_service::check_permission()?;

    // Getting all data needed in further process
    let config_file: ConfigFile = config_service::get_merged_config(file_service::get_platform_specific_path());
    let pool: Pool<Sqlite> = repository::db::get_pool().await?;

    // Filter packages by AUR status
    let packages = filter_packages_by_aur(config_file.get_packages(), false);
    let packages_aur = filter_packages_by_aur(config_file.get_packages(), true);

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

    // Execute pre-install scripts
    for script_path in pre_scripts {
        command_service::run_command(
            "sh",
            vec![script_path.bin],
            true,
            Some(Labels::Info_ExecutingPostScript),
        )?;
    }

    // Install AUR helper if needed
    if !packages_aur_str.is_empty() {
        PackageManagerEnum::get_aur_helper()?;
    }

    // Initialize package managers
    let package_manager = PackageManager::new(PackageManagerEnum::get_package_manager()?)?;
    let package_manager_aur = PackageManager::new(PackageManagerEnum::get_aur_helper()?)?;

    // Install non-AUR packages
    if !packages_str.is_empty() {
        package_manager.install_packages(
            get_applications_to_install(packages_str.clone())?,
            true,
        )?;
    }

    // Install AUR packages
    if !packages_aur_str.is_empty() {
        package_manager_aur.install_packages(
            get_applications_to_install(packages_aur_str.clone())?,
            true,
        )?;
    }

    // Uninstall packages not in config
    let packages_to_uninstall: Vec<String> = application_service::get_applications_to_remove(&pool, packages_str).await?;
    if !packages_to_uninstall.is_empty() {
        package_manager.uninstall_packages(packages_to_uninstall, true)?;
    }

    // Handle dotfiles
    for dot_file in dot_files {
        match dot_file.src {
            Some(src) => file_service::copy_file(&src, &dot_file.dest).await?,
            None => match dot_file.content {
                Some(content) => file_service::write_file(&content, &dot_file.dest)?,
                None => continue,
            },
        }
    }

    // Execute post-install scripts
    for script_path in post_scripts {
        command_service::run_command(
            "sh",
            vec![script_path.bin],
            true,
            Some(Labels::Info_ExecutingPostScript),
        )?;
    }

    // Save lock and applications
    let lock = lock_repository::save_lock(&pool, LockAdd { hash: config_file.get_hash() }).await?;
    application_repository::save_applications(
        &pool,
        packages.iter().map(|p| Application::new(lock.id, p.get_package_name(), "INSTALL".to_owned())).collect()
    ).await?;

    Ok(())
}