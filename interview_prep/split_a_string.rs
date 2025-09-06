fn main() {
    let my_string = String::from("The Rust Programming Language Is Memory Safe Without Garbage Collection Thanks To Lifetimes and Borrowing.");
    for word in my_string.split_whitespace() {
        println!("{}", word)
    }
}
