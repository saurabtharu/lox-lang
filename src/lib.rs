use std::fs::{self};
use std::io::{self, stdout, BufRead, Write};

pub mod error;
pub mod scanner;

use error::LoxError;
use scanner::Scanner;

pub fn run(source: String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}

pub fn run_file(path: &String) -> io::Result<()> {
    let buf = fs::read_to_string(path)?;
    match run(buf) {
        Ok(_) => {}
        Err(message) => {
            message.report("".to_string());
            std::process::exit(65);
        }
    }
    Ok(())
}

pub fn run_prompt() {
    let stdin = io::stdin();

    print!("> ");
    stdout().flush().unwrap();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            match run(line) {
                Ok(_) => {}
                Err(_) => {
                    // Ignore: error was already reported
                }
            }
        } else {
            break;
        }
        print!("> ");
        stdout().flush().unwrap();
    }
}
