use crate::cli::commands::models::{Command, VaultCommand, VaultField};
use crate::services::error::AppError;

pub fn parse_cmd(input: &str) -> Result<Command, AppError> {
    let trimmed = input.trim();
    let mut parts = trimmed.split_whitespace();
    let cmd = parts.next();
    let args = parts.collect::<Vec<&str>>();

    match cmd {
        Some("help" | "h" | "?") => Ok(Command::Help(args.join(" ").into())),
        Some("clear" | "cls") => Ok(Command::Clear),
        Some("exit" | "quit" | "q") => Ok(Command::Exit),
        Some("generate" | "gen") => Ok(Command::Generate),
        Some("analyze" | "scan") => {
            let password = args.get(0).ok_or(AppError::MissingArgument(String::from("password")))?;
            Ok(Command::Analyze(password.to_string()))
        },
        Some("panic") => Ok(Command::Panic),
        Some("vault" | "vlt") => parse_vault_cmd(args),
        _ => Err(AppError::InvalidCommand),
    }
}

pub fn parse_vault_cmd(args: Vec<&str>) -> Result<Command, AppError> {
    let get_arg = |idx: usize, name: &str| args.get(idx)
        .map(|s| *s).ok_or(AppError::MissingArgument(String::from(name)));
    let sub_cmd = match args.first() {
        Some(&"new") => {
            let name = get_arg(1, "name")?;
            Ok(VaultCommand::New(name.to_string()))
        },
        Some(&"open" | &"enter") => {
            let name = get_arg(1, "name")?;
            Ok(VaultCommand::Open(name.to_string()))
        },
        Some(&"close" | &"exit") => Ok(VaultCommand::Close),
        Some(&"list" | &"lst") => Ok(VaultCommand::List),
        Some(&"show") => {
            let service = args.get(1).map(|s| s.to_string());
            let expose = ["-expose", "-unmask"].iter().any(|arg| args.contains(arg));
            Ok(VaultCommand::Show(service, expose))
        },
        Some(&"add") => {
            let service = get_arg(1, "service")?.to_string();
            Ok(VaultCommand::Add(service))
        },
        Some(&"update" | &"up") => {
            let service = get_arg(1, "service")?.to_string();
            let field = parse_vault_field(get_arg(2, "field")?)?;
            let value = get_arg(3, "value")?.to_string();
            Ok(VaultCommand::Update(service, field, value))
        },
        Some(&"delete" | &"del") => {
            let service = get_arg(1, "service")?;
            Ok(VaultCommand::Delete(service.to_string()))
        },
        Some(&"copy" | &"cp") => {
            let service = get_arg(1, "service")?;
            let field_opt = args.get(2);
            let field = match field_opt {
                Some(f) => parse_vault_field(f)?,
                None => VaultField::Password,
            };
            Ok(VaultCommand::Copy(service.to_string(), field))
        },
        Some(&"destroy" | &"wipe") => Ok(VaultCommand::Destroy),
        _ => return Err(AppError::InvalidCommand),
    };
    Ok(Command::Vault(sub_cmd?))
}

pub fn parse_vault_field(input: &str) -> Result<VaultField, AppError> {
    match input.to_lowercase().as_str() {
        "username" | "name" | "user" => Ok(VaultField::Username),
        "password" | "pass" | "pwd" => Ok(VaultField::Password),
        _ => Err(AppError::InvalidArgument(input.to_string()))
    }
}