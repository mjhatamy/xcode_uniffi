use colored::Colorize;
use std::io::{stdin, stdout, Write};

pub(crate) struct CommandLineParser {}

impl CommandLineParser {
    pub(crate) fn get_user_yes_or_no_input(msg: &str) -> bool {
        let mut input_str = String::new();
        loop {
            let _ = stdout().flush();
            input_str.clear();
            println!("\n{} [Y/N](default: no) ? ", msg.underline());
            stdin()
                .read_line(&mut input_str)
                .expect("Did not enter a correct string");
            input_str = input_str.trim().to_lowercase();

            if input_str.is_empty() {
                break false;
            } else if input_str.starts_with('y') || input_str.starts_with("yes") {
                break true;
            } else if input_str.starts_with('n') || input_str.starts_with("no") {
                break false;
            } else {
                println!("Invalid input.");
                continue;
            }
        }
    }
}
