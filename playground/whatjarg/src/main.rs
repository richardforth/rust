use std::env;
use whatjarg;

fn main() {

    // collect all the args
    let args: Vec<String> = env::args().collect();

    //  Ensure only one argument is passed, if more than one, display usage and exit
    if args.len() != 2 {
        eprintln!("Error: Please supply exactly one argument.");
        whatjarg::usage();
        std::process::exit(1);
    }
    // get the shortcode from the args
    let shortcode = &args[1];
    
    // shortcodes only contain letters, not numbers at this time.
    if !shortcode.chars().all(|c| c.is_alphabetic()) {
        eprintln!("Error: Argument must contain only alphabetic characters.");
        std::process::exit(1);
    }
  
    let expansion = whatjarg::get_jarg(&shortcode); // pull from lib.rs

    println!("{}: {}", shortcode.to_uppercase(), expansion);

}

