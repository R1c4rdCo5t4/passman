use std::sync::mpsc;
use std::thread;
use ctrlc::set_handler;
use passman::cli::commands::execute_cmd;
use passman::cli::io::{read_line_with_prefix, clear_console};
use passman::cli::parser::parse_cmd;
use passman::domain::app::state::AppState;
use passman::repository::vault::vault_manager::VaultManager;
use passman::services::vault_service::VaultService;

fn main() {
    clear_console();
    println!("Welcome to Passman!");
    println!("Type 'help' to see the list of commands.");

    let (in_tx, in_rx) = mpsc::channel();
    let (out_tx, out_rx) = mpsc::channel();
    let tx_ctrl = in_tx.clone();

    // ctrl+c handler
    let _ = set_handler(move || {
        let _ = tx_ctrl.send(String::from("exit"));
    });

    // input thread
    thread::spawn(move || {
        let mut curr_vault: Option<String> = None;
        loop {
            let line = read_line_with_prefix(curr_vault.as_deref());
            if in_tx.send(line).is_err() {
                break;
            }
            curr_vault = out_rx.recv().unwrap_or(None);
        }
    });

    // main loop
    let mut state = AppState { session: None };
    let vault_service = VaultService::new(VaultManager);
    loop {
        match in_rx.recv() {
            Ok(line) => {
                match parse_cmd(&line) {
                    Ok(cmd) => {
                        match execute_cmd(cmd, &vault_service, &mut state) {
                            Ok(msg) => {
                                if let Some(m) = msg {
                                    println!("{}", m);
                                }
                            }
                            Err(err) => println!("{}", err)
                        }
                    },
                    Err(err) => println!("{}", err)
                }
                let vault = state.session.as_ref().map(|s| s.name.clone());
                let _ = out_tx.send(vault);
            }
            Err(_) => break
        }
    }
}
