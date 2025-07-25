use std::{
    io::{self, Write}, process::{Command, Output}
};

use crate::{error::{ApplicationError, CommandError}, service::{language_service::{t, Labels}, log_service::{log, LogType}}};

use duct::{cmd, Expression};
use nix::unistd::Uid;

pub fn run_command(program: &str, args: Vec<String>, require_root: bool, info_label: Option<Labels>) -> Result<String, ApplicationError> {
    if let Some(label) = info_label {
        log(
            LogType::INFO,
            &t(label, Some(vec![format!("{} {}", program, args.join(" "))])),
        );
    }

    let expression: Expression = build_command(program, args, require_root)?;

    if std::env::var("MOCK").is_ok() {
        let mock_output: String = format!("MOCK: {:?}", expression);
        log(LogType::DEBUG, &mock_output);
        return Ok(mock_output);
    }

    let output: String = expression
        .stderr_to_stdout() 
        .read()?;

    Ok(output)
}

pub fn ask_continue() -> Result<(), CommandError> {
    print!("{}", t(Labels::Info_ConfirmContinue, None));
    std::io::stdout().flush().unwrap();
    let mut val: String = String::new();
    std::io::stdin().read_line(&mut val).unwrap();
    let val = val.trim();
    if val.eq("yes") | val.eq("y") {
        return Ok(());
    }
    Err(CommandError::UserAbort(Labels::Error_UserAbort, None))
}

pub fn build_command(program: &str, args: Vec<String>, require_root: bool) -> Result<Expression, ApplicationError> {
    let uid: u32 = Uid::effective().as_raw();
    let root_uid: u32 = 0;

    if require_root && uid != root_uid {
        let sudo_available: bool = Command::new("which").arg("sudo").output()
            .map(|o: Output| o.status.success()).unwrap_or(false);
        let doas_available: bool = Command::new("which").arg("doas").output()
            .map(|o: Output| o.status.success()).unwrap_or(false);

        let mut privileged_args: Vec<String> = vec![program.to_string()];
        privileged_args.extend(args);
        if sudo_available {
            return Ok(cmd("sudo", privileged_args));
        } else if doas_available {
            return Ok(cmd("doas", privileged_args));
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Neither sudo nor doas is available",
            ))?;
        }
    }

    if !require_root && uid == root_uid {
        let sudo_uid: u32 = std::env::var("SUDO_UID").unwrap().parse().unwrap();

        let username_output: Output = Command::new("id")
            .args(&["-un", &sudo_uid.to_string()])
            .output()?;
        let username: String = String::from_utf8_lossy(&username_output.stdout).trim().to_string();

        let command_str: String = format!("{} {}", program, args.join(" "));
        return Ok(cmd("su", vec![username, "-c".to_string(), command_str]));
    }

    todo!()
}