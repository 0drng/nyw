use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref LOCALE: String = std::env::var("LANG")
    .unwrap_or("en".to_owned())
    .split("_")
    .nth(0)
    .unwrap()
    .to_lowercase();

    static ref TRANSLATIONS:HashMap<String, HashMap<Labels, String>> = HashMap::from([
        ("en".to_owned(), HashMap::from([
            (Labels::Error_PackageManager_NotInstalled, "ERROR: The package manager {0} is not installed".to_owned()),
            (Labels::Error_Which_NotInstalled, "ERROR: The program which is not installed. This prevents dependency resolution".to_owned()),
            (Labels::Error_UserAbort, "ERROR: The user has aborted the operation".to_owned()),
            (Labels::Error_NoRoot, "ERROR: The program should be run as root. Try using sudo/doas.".to_owned()),
            (Labels::Error_NoAURHelper, "ERROR: No AUR helper found".to_owned()),
            (Labels::Error_InstallationFailed, "ERROR: Failed to install packages".to_owned()),
            (Labels::Error_CommandFailed, "ERROR: Failed to spawn command".to_owned()),
            (Labels::Error_CommandStdoutFailed, "ERROR: Failed to get stdout from command".to_owned()),
            (Labels::Error_UninstallFailed, "ERROR: Package uninstall failed".to_owned()),
            (Labels::Error_FileOpenFailed, "ERROR: Failed to open file {0}".to_owned()),
            (Labels::Error_FileAlreadyExists, "ERROR: File {0} already exists. Overwriting.".to_owned()),
            (Labels::Error_GetPoolError, "ERROR: Failed to get sqlite pool".to_owned()),
            (Labels::Error_DatabaseError, "ERROR: Database error: {0}".to_owned()),
            (Labels::Error_IO, "ERROR: An error occured: {0}".to_owned()),
            (Labels::Info_NewlyUninstalledPackages, "INFO: {0} packages got deleted".to_owned()),
            (Labels::Info_ConfirmContinue, "INFO: Do you want to continue? y/N: ".to_owned()),
            (Labels::Info_StartingPackageInstallation, "INFO: Starting package installation with: {0}".to_owned()),
            (Labels::Info_StartingPackageRemoval, "INFO: Starting package removal with: {0}".to_owned()),
            (Labels::Info_ExecutingPreScript, "INFO: Executing pre-script: {0}".to_owned()),
            (Labels::Info_ExecutingPostScript, "INFO: Executing post-script: {0}".to_owned()),
            (Labels::Info_NewlyInstalledPackages, "INFO: {0} newly/installed packages".to_owned()),
            (Labels::Info_CopyingFile, "INFO: Copying file {0} to {1}".to_owned()),
            (Labels::Info_WritingFile, "INFO: Writing content into file {0}".to_owned()),
            (Labels::Info_Priviledge_Deeskalation, "INFO: Deesklation of priviledges to {0}".to_owned())
        ]))
    ]);
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Hash)]
pub enum Labels {
    // ERROR
    Error_PackageManager_NotInstalled,
    Error_Which_NotInstalled,
    Error_UserAbort,
    Error_NoRoot,
    Error_NoAURHelper,
    Error_InstallationFailed,
    Error_CommandStdoutFailed,
    Error_CommandFailed,
    Error_UninstallFailed,
    Error_FileOpenFailed,
    Error_FileAlreadyExists,
    Error_GetPoolError,
    Error_DatabaseError,
    Error_IO,
    // INFO
    Info_ConfirmContinue,
    Info_StartingPackageInstallation,
    Info_StartingPackageRemoval,
    Info_ExecutingPreScript,
    Info_ExecutingPostScript,
    Info_NewlyInstalledPackages,
    Info_NewlyUninstalledPackages,
    Info_CopyingFile,
    Info_WritingFile,
    Info_Priviledge_Deeskalation,
}

pub fn t(label: Labels, params: Option<Vec<String>>) -> String {
    let text: String = TRANSLATIONS
        .get(&LOCALE.to_string())
        .unwrap_or(TRANSLATIONS.get("en").unwrap())
        .get(&label)
        .unwrap()
        .to_owned();

    if let Some(params) = params {
        return replace(text, params);
    }

    return text;
}

fn replace(mut text: String, params: Vec<String>) -> String {
    for (i, param) in params.iter().enumerate() {
        let placeholder: String = format!("{{{}}}", i);
        text = text.replace(&placeholder, &param);
    }
    return text;
}