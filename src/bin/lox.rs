use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        rlox::run_file(&args[1]).expect("Could not run file");
    } else {
        rlox::run_prompt();
    }
}
