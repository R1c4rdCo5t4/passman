use crate::cli::io::get_clipboard;
use crate::domain::app::error::AppError;
use crate::domain::cli::commands::{Command, VaultCommand};
use crate::domain::cli::field::Field;
use crate::domain::cli::password_params::PasswordParams;
use crate::utils::constants::PASSWORD_MIN_LEN;
use crate::utils::validation::validate_arg;

pub fn parse_cmd(input: &str) -> Result<Command, AppError> {
    let trimmed = input.trim();
    let mut parts = trimmed.split_whitespace();
    let cmd = parts.next();
    let collected = parts.collect::<Vec<&str>>();
    let args: Vec<&str> = collected.iter().filter(|arg| !arg.starts_with('-')).cloned().collect();
    let opts: Vec<&str> = collected.iter().filter(|arg| arg.starts_with('-')).cloned().collect();

    match cmd {
        Some("help" | "h" | "?") => {
            let cmd_arg: Option<String> = {
                let joined = args.join(" ");
                if joined.is_empty() {
                    None
                } else {
                    validate_arg(&joined, "command")?;
                    Some(joined)
                }
            };
            Ok(Command::Help(cmd_arg.into()))
        }
        Some("clear" | "cls") => Ok(Command::Clear),
        Some("exit" | "quit" | "q") => Ok(Command::Exit),
        Some("generate" | "gen") => {
            let length = args.get(0).and_then(|s| s.parse::<usize>().ok()).unwrap_or(PASSWORD_MIN_LEN);
            let params = PasswordParams {
                length,
                avoid_ambiguous: opts.contains(&"-avoid-ambiguous"),
            };
            let copy = opts.contains(&"-copy") || opts.contains(&"-cp") || opts.contains(&"-c");
            Ok(Command::Generate(params, copy))
        },
        Some("analyze" | "score") => {
            let arg = args.get(0).map(|s| s.to_string());
            let password: String = if let Some(pass) = arg {
                pass
            } else {
                let paste = opts.contains(&"-paste") || opts.contains(&"-p");
                if !paste {
                    return Err(AppError::MissingArgument("password".to_string()));
                }
                get_clipboard()
            };
            Ok(Command::Analyze(password.to_string()))
        },
        Some("panic") => Ok(Command::Panic),
        Some("vault" | "vlt") => parse_vault_cmd(&args, opts),
        _ => Err(AppError::InvalidCommand),
    }
}

pub fn parse_vault_cmd(args: &Vec<&str>, opts: Vec<&str>) -> Result<Command, AppError> {
    let get_arg = |index: usize, name: &str| {
        let arg = args
            .get(index)
            .map(|s| *s)
            .ok_or(AppError::MissingArgument(name.to_string()))?;
        validate_arg(&arg, name)?;
        Ok(arg)
    };
    let sub_cmd = match args.first() {
        Some(&"new" | &"create") => {
            let name = get_arg(1, "name")?;
            Ok(VaultCommand::New(name.to_string()))
        },
        Some(&"open" | &"enter" | &"unlock") => {
            let name = get_arg(1, "name")?;
            Ok(VaultCommand::Open(name.to_string()))
        },
        Some(&"close" | &"exit" | &"lock") => Ok(VaultCommand::Close),
        Some(&"list" | &"lst") => Ok(VaultCommand::List),
        Some(&"show" | &"inspect") => {
            let entry = args.get(1).map(|s| s.to_string());
            let expose = ["-expose", "-unmask"].iter().any(|opt| opts.contains(opt));
            Ok(VaultCommand::Show(entry, expose))
        },
        Some(&"add") => {
            let entry = get_arg(1, "entry")?.to_string();
            Ok(VaultCommand::Add(entry))
        },
        Some(&"update" | &"up") => {
            let entry = get_arg(1, "entry")?.to_string();
            let field = parse_vault_field(opts.get(0).unwrap_or(&""))?;
            let value = get_arg(2, "value")?.to_string();
            Ok(VaultCommand::Update(entry, field, value))
        },
        Some(&"delete" | &"del") => {
            let entry = get_arg(1, "entry")?;
            Ok(VaultCommand::Delete(entry.to_string()))
        },
        Some(&"copy" | &"cp") => {
            let entry = get_arg(1, "entry")?;
            let field_opt = opts.get(1);
            let field = match field_opt {
                Some(f) => parse_vault_field(f)?,
                None => Field::Password,
            };
            Ok(VaultCommand::Copy(entry.to_string(), field))
        },
        Some(&"destroy" | &"wipe") => Ok(VaultCommand::Destroy),
        _ => return Err(AppError::InvalidCommand),
    };
    Ok(Command::Vault(sub_cmd?))
}

pub fn parse_vault_field(input: &str) -> Result<Field, AppError> {
    match input.to_lowercase().as_str() {
        "-username" | "-name" | "-user" => Ok(Field::Username),
        "-password" | "-pass" | "-pwd" => Ok(Field::Password),
        _ => Err(AppError::InvalidArgument(input.to_string()))
    }
}