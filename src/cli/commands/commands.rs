use crate::cli::commands::enums::{Command, Credentials, Folders, Session};
use crate::utils::vectors::{get_opt_str, get_str};

pub fn parse_cmd(input: &str) -> Command {
    let trimmed = input.trim();
    let mut parts = trimmed.split_whitespace();
    let cmd = parts.next(); // first word
    let args = parts.collect::<Vec<&str>>(); // rest

    match cmd {
        Some("help") | Some("h") | Some("?") => Command::Help,
        Some("exit") | Some("quit") | Some("q") => Command::Exit,
        Some("session") | Some("sess") => {
            let sub_cmd = match args.get(0) {
                Some(&"new") => Session::New,
                Some(&"login") => Session::Login,
                Some(&"status") => Session::Status,
                Some(&"extend") => Session::Extend,
                Some(&"logout") => Session::Logout,
                Some(&"panic") => Session::Panic,
                Some(&"delete") => Session::Delete,
                _ => return Command::Unknown
            };
            Command::Session(sub_cmd)
        }
        Some("credentials") | Some("credential") | Some("creds") => {
            let sub_cmd = match args.get(0) {
                Some(&"show") => {
                    let name = get_opt_str(&args, 1);
                    let mask = args.contains(&"-m") || args.contains(&"-mask");
                    Credentials::Show(name, mask)
                },
                Some(&"add") => Credentials::Add,
                Some(&"update") => {
                    let field = get_str(&args, 1);
                    let value = get_str(&args, 2);
                    Credentials::Update(field, value)
                },
                Some(&"delete") => {
                    let name = get_str(&args, 1);
                    Credentials::Delete(name)
                },
                Some(&"search") => {
                    let query = get_str(&args, 1);
                    Credentials::Search(query)
                },
                Some(&"generate") => Credentials::Generate,
                Some(&"analyze") => {
                    let password = get_str(&args, 1);
                    Credentials::Analyze(password)
                },
                _ => return Command::Unknown
            };
            Command::Credentials(sub_cmd)
        }
        Some("folders") | Some("folder") => {
            let sub_cmd = match args.get(0) {
                Some(&"show") => {
                    let name = get_opt_str(&args, 1);
                    Folders::Show(name)
                },
                Some(&"add") => {
                    let name = get_str(&args, 1);
                    Folders::Add(name)
                },
                Some(&"delete") => {
                    let name = get_str(&args, 1);
                    Folders::Delete(name)
                },
                Some(&"rename") => {
                    let prev = get_str(&args, 1);
                    let new = get_str(&args, 2);
                    Folders::Rename(prev, new)
                }
                _ => return Command::Unknown
            };
            Command::Folders(sub_cmd)
        }
        _ => Command::Unknown
    }
}

pub fn execute_cmd(cmd: Command) {
    match cmd {
        Command::Exit => {
            println!("Exiting...");
            std::process::exit(0);
        }
        _ => {
            println!("{:?}", cmd);
            println!("Unknown or not implemented command");
        }
    }
}