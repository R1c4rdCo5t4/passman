use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use ctrlc::set_handler;
use passman::cli::commands::commands::{execute_cmd};
use passman::cli::commands::parser::{parse_cmd};
use passman::cli::stdin::{read_line_with_prefix};
use passman::cli::stdout::{clear_console};
use passman::services::error::AppError;
use passman::state::AppState;

fn main() {
    clear_console();
    println!("Welcome to Passman!");
    println!("Type 'help' to see the list of commands.");

    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let (vault_tx, vault_rx): (Sender<Option<String>>, Receiver<Option<String>>) = mpsc::channel();
    let tx_ctrl = tx.clone();
    set_handler(move || {
        let _ = tx_ctrl.send(String::from("exit")); // send exit command on ctrl+c
    }).expect("Error setting ctrl+c handler");

    let input_tx = tx.clone();
    let input_thread = thread::spawn(move || {
        let mut curr_vault: Option<String> = None;
        loop {
            let line = read_line_with_prefix(curr_vault.as_deref());
            if input_tx.send(line).is_err() {
                break; // stop if main thread is gone
            }
            curr_vault = vault_rx.recv().unwrap_or(None);
        }
    });

    let mut state = AppState { session: None };
    loop {
        match rx.recv() {
            Ok(line) => {
                match parse_cmd(&line) {
                    Ok(cmd) => {
                        match execute_cmd(cmd, &mut state) {
                            Ok(msg) => {
                                if let Some(m) = msg {
                                    println!("{}", m);
                                }
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
                let vault = state.session.as_ref().map(|s| s.name.clone());
                vault_tx.send(vault).unwrap_or_else(|e| {
                    eprintln!("Failed to send session update: {}", e);
                });
            }
            Err(_) => break
        }
    }
    input_thread.join().expect("Input thread panicked");
}
