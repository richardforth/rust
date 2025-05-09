fn main() {
    // Import Write trait, as well as std::io
    use std::io::{self, Write};

    // A silly string-reply program
    // while reading the first few pages
    // of the chapter on user input
    let mut user_input = String::new();
    print!("Please enter your first name (or nickname): ");
    io::stdout().flush().unwrap(); // <- flush after print!

    io::stdin().read_line(&mut user_input).unwrap();
    println!("Hello {}, nice to meet you!\n", user_input.trim());
}
