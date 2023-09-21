use miette::Result;
use std::env;
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        rlox::run_file(&args[1])?;
    } else {
        rlox::run_prompt()?;
    }
    Ok(())
}
