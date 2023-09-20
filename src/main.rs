use std::env::args;
use std::fs::{self};
use std::io::{self, stdout, BufRead, Write};

mod error;
mod scanner;
mod token;
mod token_type;

use error::*;
use scanner::Scanner;

pub fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]).expect("Could not run file");
    } else {
        run_prompt();
    }
}

fn run_file(path: &String) -> io::Result<()> {
    let buf = fs::read_to_string(path)?;
    match run(buf) {
        Ok(_) => {}
        Err(message) => {
            message.report("".to_string());
            std::process::exit(65);
        }
    }

    /*
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();

    reader.read_to_end(&mut buf).unwrap();

    run(&String::from_utf8(buf).unwrap());
    */

    Ok(())
}
fn run_prompt() {
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

fn run(source: String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("\t{:?}", token);
    }
    Ok(())
}
