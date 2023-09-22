use miette::{self, NamedSource,Report};
use std::fs::{self};
use std::io::{self, stdout, BufRead, Write};

pub mod error;
pub mod scanner;

use error::LoxError;
use scanner::Scanner;

pub fn run_file(path: &String) -> miette::Result<()> {
    let buf = fs::read_to_string(path).map_err(LoxError::IOError)?;

    run(&buf).map_err(|err| {
        let named_source = NamedSource::new(path, buf);
        err.with_source_code(named_source)
    })?;
    /*
    match run(buf) {
        Ok(_) => {}
        Err(message) => {

            // message.report("".to_string());
            // std::process::exit(65);
        }
    }
    */
    Ok(())
}

pub fn run_prompt() -> miette::Result<()> {
    let stdin = io::stdin();

    print!("> ");
    stdout().flush().map_err(LoxError::IOError)?;
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            run(&line).map_err(|err| err.with_source_code(line))?;
            // match run(&line) {
            //     Ok(_) => {}
            //     Err(err) => {
            //         err.with_source_code(line)?
            //         // Ignore: error was already reported
            //     }
            // }
        } else {
            break;
        }
        print!("> ");
        stdout().flush().unwrap();
    }
    Ok(())
}

pub fn run(source: &String) -> miette::Result<()> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    // println!("line  (from:to)     Lexeme \t\tToken Type \tLiteral ");
    // println!("----------------------------------------------------");
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}
