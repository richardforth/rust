use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let name = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        "Rustacean".to_string()
    };

    if !name.chars().all(|c|
        c.is_alphabetic() || c.is_whitespace()) {
            eprintln!("Error: Name must only contain letters and spaces.");
            std::process::exit(1)
    }
}
