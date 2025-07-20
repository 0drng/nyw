use std::{
    io::{BufRead, BufReader, Write},
    process::{Command, ExitStatus, Stdio},
};

use crate::{error::CommandError, service::{language_service::{t, Labels}, log_service::{log, LogType}}};

pub fn run_command(program: &str, args: Vec<String>, info_label: Labels) -> Result<ExitStatus, CommandError> {
    let mut command = Command::new(program);
    command.args(args);

    log(
        LogType::INFO,
        &t(info_label, Some(vec![format_command(&command)])),
    );

    if std::env::var("MOCK").is_ok() {
        log(LogType::INFO, &format!("MOCK: {}", format_command(&command)));
        return Ok(ExitStatus::default());
    }

    let mut child = command.stdout(Stdio::piped())
        .spawn()
        .map_err(|_| CommandError::CommandFailed(Labels::Error_CommandFailed, None))?;

    let stdout = child.stdout
        .take()
        .ok_or(CommandError::UserAbort(Labels::Error_UserAbort, None))?;

    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line.trim()),
            Err(_) => break,
        }
    }

    let status = child.wait()
        .map_err(|_| CommandError::CommandFailed(Labels::Error_CommandFailed, None))?;

    Ok(status)
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