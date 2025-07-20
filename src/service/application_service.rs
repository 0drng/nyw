use sqlx::{Pool, Sqlite};

use crate::{
    error::ApplicationError,
    model::{
        application::Application,
        package_manager::{PackageManager, PackageManagerEnum},
    },
    repository::{application_repository, lock_repository},
};

pub fn get_applications_to_install(
    packages: Vec<String>,
) -> Result<Vec<String>, ApplicationError> {
    let package_manager: PackageManager = PackageManager::new(PackageManagerEnum::get_package_manager()?)?;
    let installed_packages: Vec<String> = package_manager.get_installed();

    return Ok(packages
        .into_iter()
        .filter(|package| !installed_packages.contains(&package))
        .collect());
}

pub async fn get_last_installed_applications(
    pool: &Pool<Sqlite>,
) -> Result<Vec<Application>, ApplicationError> {
    let last_lock_id: u32 = match lock_repository::get_latest_lock_id(pool).await? {
        Some(id) => id,
        None => return Ok(vec![]),
    };
    return Ok(
        application_repository::get_applications_by_lock_id_and_action(
            pool,
            last_lock_id,
            "INSTALL".to_owned(),
        )
        .await?,
    );
}

pub async fn get_applications_to_remove(
    pool: &Pool<Sqlite>,
    packages: Vec<String>,
) -> Result<Vec<String>, ApplicationError> {
    let last_installed_packages: Vec<String> = get_last_installed_applications(pool)
        .await?
        .iter()
        .map(Application::get_name)
        .collect();

    let packages_to_remove: Vec<String> = last_installed_packages.clone().into_iter().filter(|package| !packages.contains(&package)).collect();

    return Ok(packages_to_remove);
}
