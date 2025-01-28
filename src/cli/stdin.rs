use std::io;
use std::io::Write;

pub fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read cli");
    input.trim().to_string()
}

pub fn read_line_with(content: &str) -> String {
    print!("{}", content);
    io::stdout().flush().expect("Failed to flush stdout");
    read_line()
}
