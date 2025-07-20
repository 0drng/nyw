use std::{
    io::{BufRead, BufReader, Write},
    process::{Command, ExitStatus, Stdio},
};

use crate::{error::CommandError, service::{language_service::{t, Labels}, log_service::{log, LogType}}};

use duct::cmd;

pub fn run_command(program: &str, args: Vec<String>, info_label: Labels) -> Result<ExitStatus, CommandError> {
    log(
        LogType::INFO,
        &t(info_label, Some(vec![format!("{} {}", program, args.join(" "))])),
    );

    if std::env::var("MOCK").is_ok() {
        log(LogType::INFO, &format!("MOCK: {} {}", program, args.join(" ")));
        return Ok(ExitStatus::default());
    }

    let output = cmd(program, &args)
        .run()
        .map_err(|_| CommandError::CommandFailed(Labels::Error_CommandFailed, None))?;

    Ok(output.status)
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

fn format_command(cmd: &Command) -> String {
    let program = cmd.get_program().to_string_lossy();
    let args: Vec<String> = cmd.get_args()
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect();
    format!("{} {}", program, args.join(" "))
}