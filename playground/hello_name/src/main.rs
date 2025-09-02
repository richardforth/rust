use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let name = if args.len() > 1 {
        // Add functionality to blend all args into one single string
        args[1..].join(" ")
    } else {
        "Rustacean".to_string()
    };

    // Force error if name contains numbers
    if !name.chars().all(|c|
        c.is_alphabetic() || c.is_whitespace()) {
        eprintln!("Error: Name must only contain letters and spaces");
        std::process::exit(1);
    }

    // Add condition if name is admin, let them know all systems are operational
    if name.to_lowercase() == "admin" {
        println!("Greetings, Administrator. All systems are operational.");
    } else {
        println!("Hello {}, welcome to Rust!", name);
    }
  
   
}
