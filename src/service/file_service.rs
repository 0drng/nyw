use std::{fs::{self, File}, io::{self, Read, Write}, os::unix::fs::MetadataExt, path::Path};

use reqwest::Client;

use crate::{error::ApplicationError, service::language_service::{t, Labels}};

pub fn read_file(src: &str) -> io::Result<String> {
    let mut file: File = File::open(src)?;
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;

    return Ok(String::from_utf8(buf).unwrap());
}

pub async  fn copy_file(source: &str, destination: &str) -> io::Result<()> {
    let content: String = if source.starts_with("https://") {
        let client: Client = Client::builder()
            .https_only(true)
            .build().unwrap();
    
        let response: reqwest::Response = client.get(source).send().await.unwrap();
        response.text().await.unwrap()
    } else {
        println!("{}", t(Labels::Info_CopyingFile, Some(vec![source.to_owned(), destination.to_owned()])));
        let source_path: &Path = Path::new(source);
        let content: Vec<u8> = fs::read(source_path)?;
        String::from_utf8_lossy(&content).to_string()
    };

    write_file(&content, destination)?;
    Ok(())
}

pub fn write_file(content: &str, destination: &str) -> io::Result<()> {
    println!("{}", t(Labels::Info_WritingFile, Some(vec![destination.to_owned()])));
    let destination_path = Path::new(destination);

    if let Some(parent) = destination_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut dest_file = fs::File::create(destination_path)?;
    dest_file.write_all(content.as_bytes())?;

    Ok(())
}

pub fn get_uid() -> u32 {
    return std::fs::metadata("/proc/self")
        .map(|m| m.uid())
        .unwrap_or(1000);
}

pub fn check_permission() -> Result<(), ApplicationError> {
    let uid: u32 = get_uid();
    if uid != 0 {
        return Err(ApplicationError::new(Labels::Error_NoRoot, None));
    }
    return Ok(());
}

pub fn get_platform_specific_path() -> &'static str {
    if cfg!(target_os = "windows") {
        "C:\\Program Files\\nyw"
    } else {
        "/etc/nyw"
    }
}