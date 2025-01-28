use passman::cli::commands::commands::{execute_cmd, parse_cmd};
use passman::cli::stdin::read_line;
use passman::cli::stdout::{clear_console, print_prefix};

fn main() {
    clear_console();
    println!("Welcome to Passman!");
    println!("Type 'help' to see the list of commands.");
    loop {
        print_prefix();
        let line = read_line();
        let cmd = parse_cmd(&line);
        execute_cmd(cmd)
    }
}
