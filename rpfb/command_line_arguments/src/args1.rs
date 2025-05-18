// any script that collects command line arguments needs to import std::env
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Command line arguments:\n{:?}\nTotal number of arguments: {}\nArguments passed: {}", args, args.len(), (args.len() -1));
}
