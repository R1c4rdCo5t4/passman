use passman::cli::commands::commands::{execute_cmd};
use passman::cli::commands::parser::{parse_cmd};
use passman::cli::stdin::read_line;
use passman::cli::stdout::{clear_console, print_prefix};
use passman::services::error::AppError;
use passman::state::AppState;

fn main() {
    clear_console();
    println!("Welcome to Passman!");
    println!("Type 'help' to see the list of commands.");

    let mut state = AppState { session: None };
    loop {
        let vault = state.session.as_ref().map(|s| s.name.as_str());
        print_prefix(vault);
        let line = read_line();
        match parse_cmd(&line) {
            Ok(cmd) => {
                match execute_cmd(cmd, &mut state) {
                    Ok(msg) => if msg.is_some() {
                        println!("{}", msg.unwrap());
                    }
                    Err(err) => eprintln!("{}", err)
                }
            },
            Err(err) => match err {
                AppError::InvalidCommand => eprintln!("Invalid command: {}", line),
                AppError::InvalidArgument(arg) => eprintln!("Invalid argument: {}", arg),
                AppError::MissingArgument(arg) => eprintln!("Missing argument: {}", arg),
            }
        }
    }
}
