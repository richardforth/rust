use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let name = if args.len() > 1 {
        &args[1]
    } else {
        "Rustacean"
    };

    // Add condition if name is admin, let them know all systems are operational
    if name.to_lowercase() == "admin" {
        println!("Greetings, Administrator. All systems are operational.");
    } else {
        println!("Hello {}, welcome to Rust!", name);
    }
}
