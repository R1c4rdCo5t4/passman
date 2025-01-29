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
        Some("generate") => Ok(Command::Generate),
        Some("analyze") => {
            let password = args.get(0).ok_or(AppError::MissingArgument(String::from("password")))?;
            Ok(Command::Analyze(password.to_string()))
        },
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
        Some(&"open") => {
            let name = get_arg(1, "name")?;
            Ok(VaultCommand::Open(name.to_string()))
        },
        Some(&"close") => Ok(VaultCommand::Close),
        Some(&"list") => Ok(VaultCommand::List),
        Some(&"show") => {
            let name = args.get(1).map(|s| s.to_string());
            let unmask = args.contains(&"-unmask");
            Ok(VaultCommand::Show(name, unmask))
        },
        Some(&"add") => {
            let service = get_arg(1, "service")?.to_string();
            let username = get_arg(2, "username")?.to_string();
            let password = get_arg(3, "password")?.to_string();
            Ok(VaultCommand::Add(service, username, password))
        },
        Some(&"update") => {
            let service = get_arg(1, "service")?.to_string();
            let field = parse_vault_field(get_arg(2, "field")?)?;
            let value = get_arg(3, "value")?.to_string();
            Ok(VaultCommand::Update(service, field, value))
        },
        Some(&"delete") => {
            let name = get_arg(1, "name")?;
            Ok(VaultCommand::Delete(name.to_string()))
        },
        Some(&"copy") => {
            let field = parse_vault_field(get_arg(1, "field")?)?;
            Ok(VaultCommand::Copy(field))
        },
        Some(&"search") => {
            let query = get_arg(1, "query")?;
            Ok(VaultCommand::Search(query.to_string()))
        },
        Some(&"destroy") => Ok(VaultCommand::Destroy),
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