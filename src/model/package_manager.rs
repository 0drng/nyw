use crate::{error::PackageManagerError, service::{command_service, language_service::Labels}};
use std::process::{Command, ExitStatus, Output};

pub enum PackageManagerEnum {
    PACMAN,
    PARU,
    YAY,
    APT,
    APK,
    WINGET,
    BREW,
}

impl PackageManagerEnum {
    pub fn get_aur_helper() -> Result<PackageManagerEnum, PackageManagerError> {
        if PackageManager::check_installed(&PackageManagerEnum::PARU).is_ok() {
            return Ok(PackageManagerEnum::PARU);
        }
        if PackageManager::check_installed(&PackageManagerEnum::YAY).is_ok() {
            return Ok(PackageManagerEnum::YAY);
        }
        Err(PackageManagerError::NotInstalled(
            Labels::Error_NoAURHelper,
            None,
        ))
    }

    pub fn get_package_manager() -> Result<PackageManagerEnum, PackageManagerError> {
        if PackageManager::check_installed(&PackageManagerEnum::PACMAN).is_ok() {
            return Ok(PackageManagerEnum::PACMAN);
        }
        if PackageManager::check_installed(&PackageManagerEnum::APT).is_ok() {
            return Ok(PackageManagerEnum::APT);
        }
        if PackageManager::check_installed(&PackageManagerEnum::APK).is_ok() {
            return Ok(PackageManagerEnum::APK);
        }
        if PackageManager::check_installed(&PackageManagerEnum::BREW).is_ok() {
            return Ok(PackageManagerEnum::BREW);
        }
        if PackageManager::check_installed(&PackageManagerEnum::WINGET).is_ok() {
            return Ok(PackageManagerEnum::WINGET);
        }
        Err(PackageManagerError::NotInstalled(
            Labels::Error_NoAURHelper,
            None,
        ))
    }

    pub fn get_command(&self) -> String {
        match self {
            PackageManagerEnum::PACMAN => "pacman".to_owned(),
            PackageManagerEnum::PARU => "paru".to_owned(),
            PackageManagerEnum::YAY => "yay".to_owned(),
            PackageManagerEnum::APT => "apt".to_owned(),
            PackageManagerEnum::APK => "apk".to_owned(),
            PackageManagerEnum::WINGET => "winget".to_owned(),
            PackageManagerEnum::BREW => "brew".to_owned(),
        }
    }

    pub fn get_install_param(&self, update: bool) -> String {
        let param = match self {
            PackageManagerEnum::PACMAN => "-S".to_owned(),
            PackageManagerEnum::PARU => "-S".to_owned(),
            PackageManagerEnum::YAY => "-S".to_owned(),
            PackageManagerEnum::APT => "install".to_owned(),
            PackageManagerEnum::APK => "add".to_owned(),
            PackageManagerEnum::WINGET => "install".to_owned(),
            PackageManagerEnum::BREW => "install".to_owned(),
        };

        return param;
    }

    pub fn get_list_programms_param(&self, update: bool) -> String {
        let param = match self {
            PackageManagerEnum::PACMAN => "-Qqe".to_owned(),
            PackageManagerEnum::PARU => "-Qqe".to_owned(),
            PackageManagerEnum::YAY => "-Qqe".to_owned(),
            PackageManagerEnum::APT => "list --installed 2>/dev/null | grep -v '^Listing\\.\\.\\.' | cut -d/ -f1".to_owned(),
            PackageManagerEnum::APK => "info".to_owned(),
            PackageManagerEnum::WINGET => "list".to_owned(),
            PackageManagerEnum::BREW => "leaves".to_owned(),
        };

        return param;
    }

    pub fn get_uninstall_param(&self, with_dependencies: bool) -> String {
        let mut param: String = match self {
            PackageManagerEnum::PACMAN => "-R".to_owned(),
            PackageManagerEnum::PARU => "-R".to_owned(),
            PackageManagerEnum::YAY => "-R".to_owned(),
            PackageManagerEnum::APT => "remove".to_owned(),
            PackageManagerEnum::APK => "del".to_owned(),
            PackageManagerEnum::WINGET => "uninstall".to_owned(),
            PackageManagerEnum::BREW => "uninstall".to_owned(),
        };

        if with_dependencies {
            let with_depenencies_param = match self {
                PackageManagerEnum::PACMAN => "ncs".to_owned(),
                PackageManagerEnum::PARU => "ncs".to_owned(),
                PackageManagerEnum::YAY => "ncs".to_owned(),
                PackageManagerEnum::APT => String::new(),
                PackageManagerEnum::APK => String::new(),
                PackageManagerEnum::WINGET => String::new(),
                PackageManagerEnum::BREW => String::new(),
            };
            param = format!("{}{}", param, with_depenencies_param);
        }

        return param;
    }

    pub fn needs_root(&self) -> bool {
        match self {
            PackageManagerEnum::PARU | PackageManagerEnum::YAY => false,
            _ => true,
        }
    }
}

pub struct PackageManager {
    package_manager: PackageManagerEnum,
}

impl PackageManager {
    pub fn new(package_manager: PackageManagerEnum) -> Result<Self, PackageManagerError> {
        Self::check_installed(&package_manager)?;
        Ok(PackageManager { package_manager })
    }

    pub fn install_packages(
        &self,
        packages: Vec<String>,
        update: bool,
    ) -> Result<(), PackageManagerError> {
        let program: String = self.package_manager.get_command();

        let args: Vec<String> = vec![self.package_manager.get_install_param(update)]
            .into_iter()
            .chain(packages.into_iter())
            .collect();

        let needs_root: bool = self.package_manager.needs_root();

        if let Err(error) = command_service::run_command(&program, args, needs_root, Some(Labels::Info_StartingPackageInstallation)) {
            return Err(PackageManagerError::InstallFailed(
                Labels::Error_InstallationFailed,
                None,
            ));
        }

        Ok(())
    }

    pub fn uninstall_packages(
        &self,
        packages: Vec<String>,
        with_dependencies: bool,
    ) -> Result<(), PackageManagerError> {

        let program: String = self.package_manager.get_command();

        let args: Vec<String> = vec![self.package_manager.get_uninstall_param(with_dependencies)]
            .into_iter()
            .chain(packages.into_iter())
            .collect();
                
        if let Err(error) = command_service::run_command(&program, args, true, Some(Labels::Info_StartingPackageRemoval)) {
            return Err(PackageManagerError::UninstallFailed(
                Labels::Error_UninstallFailed,
                None,
            ));
        }

        Ok(())
    }

    pub fn check_installed(
        package_manager: &PackageManagerEnum,
    ) -> Result<(), PackageManagerError> {
        let output_result: Result<Output, std::io::Error> = Command::new("which")
            .arg(package_manager.get_command())
            .output();

        let output = match output_result {
            Ok(output) => output,
            Err(_) => {
                return Err(PackageManagerError::WhichIsNotInstalled(
                    Labels::Error_Which_NotInstalled,
                    None,
                ))
            }
        };

        if !output.status.success() {
            return Err(PackageManagerError::NotInstalled(
                Labels::Error_PackageManager_NotInstalled,
                None,
            ));
        }

        Ok(())
    }

    pub fn get_installed(&self) -> Vec<String> {
        let output: Output = Command::new(&self.package_manager.get_command())
            .arg("-Qqe")
            .output()
            .unwrap();

        return String::from_utf8(output.stdout)
            .unwrap()
            .split("\n")
            .map(|f| f.to_owned())
            .collect();
    }
}