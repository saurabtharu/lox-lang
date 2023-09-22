use miette::Result;
use std::env;
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        rlox::run_file(&args[1])?;
        /*
        match rlox::run_file(&args[1]) {
            Ok(_) => {}
            Err(error) => eprintln!("{}",error),
        }
        */
    } else {
        rlox::run_prompt()?;
        /*
        match rlox::run_prompt(){
            Ok(_) => {}
            Err(error) => eprintln!("{}",error),
        }
        */
    }
    Ok(())
}
