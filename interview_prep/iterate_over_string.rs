fn main() {
    let my_string = String::from("The Rust Programming Language Is Memory Safe Without Garbage Collection Thanks To Lifetimes and Borrowing.");
    for c in my_string.chars() { 
        println!("c: {} -> (String::from(c).to_lowercase()) -> {} -> (String::from(c).to_uppercase()) -> {}",
            c,
            String::from(c).to_lowercase(),
            String::from(c).to_uppercase()
        )
    }
}
