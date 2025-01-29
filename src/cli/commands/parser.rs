use crate::cli::commands::enums::{Command, Vault};
use crate::utils::vectors::{get_str};

pub fn parse_cmd(input: &str) -> Result<Command, &'static str> {
    let trimmed = input.trim();
    let mut parts = trimmed.split_whitespace();
    let cmd = parts.next();
    let args = parts.collect::<Vec<&str>>();
    let fst = get_str(&args, 0);
    let snd = get_str(&args, 1);
    let thd = get_str(&args, 2);
    let fth = get_str(&args, 3);

    match cmd {
        Some("help") | Some("h") | Some("?") => Ok(Command::Help(Option::from(args.join(" ")))),
        Some("clear") | Some("cls") => Ok(Command::Clear),
        Some("exit") | Some("quit") | Some("q") => Ok(Command::Exit),
        Some("generate") => Ok(Command::Generate),
        Some("analyze") => fst.map(|v| Command::Analyze(v.to_string())).ok_or("Invalid argument\nUsage: analyze <password>"),
        Some("vault") | Some("vlt") => {
            let sub_cmd = match args.get(0) {
                Some(&"new") => snd.map(|v| Vault::New(v.to_string())).ok_or("Invalid argument\nUsage: vault new <name>"),
                Some(&"open") => snd.map(|v| Vault::Open(v.to_string())).ok_or("Invalid argument\nUsage: vault open <name>"),
                Some(&"close") => Ok(Vault::Close),
                Some(&"list") => Ok(Vault::List),
                Some(&"show") => {
                    let unmask = args.contains(&"-unmask");
                    Ok(Vault::Show(snd, unmask))
                },
                Some(&"add") => {
                    match (snd, thd, fth) {
                        (Some(service), Some(username), Some(password)) =>
                            Ok(Vault::Add(service, username, password)),
                        _ => Err("Invalid arguments\nUsage: vault add <service> <username> <password>")
                    }
                }
                Some(&"update") => {
                    match (snd, thd, fth) {
                        (Some(service), Some(field), Some(value)) => Ok(Vault::Update(service, field, value)),
                        _ => Err("Invalid arguments\nUsage: vault update <service> <field> <value>")
                    }
                },
                Some(&"delete") => snd.map(|v| Vault::Delete(v.to_string())).ok_or("Invalid argument\nUsage: vault delete <name>"),
                Some(&"copy") => snd.map(|v| Vault::Copy(v.to_string())).ok_or("Invalid argument\nUsage: vault copy <name>"),
                Some(&"search") => snd.map(|v| Vault::Search(v.to_string())).ok_or("Invalid argument\nUsage: vault search <name>"),
                Some(&"destroy") => Ok(Vault::Destroy),
                _ => return Err("Invalid argument\nUsage: vault <command>"),
            };
            match sub_cmd {
                Ok(cmd) => Ok(Command::Vault(cmd)),
                Err(err) => Err(err),
            }
        }
        _ => Err("Invalid command"),
    }
}
