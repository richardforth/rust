use std::env;
use colored::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let name = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        "Rustacean".to_string()
    };

    if !name.chars().all(|c|
        c.is_alphabetic() || c.is_whitespace()) {
            eprintln!("[ {} ] Error: Name must only contain letters and spaces.",
            "!!".red());
            std::process::exit(1)
    }

    println!("[ {} ] Validated string: {}", 
    "OK".green(), name.yellow());
}
