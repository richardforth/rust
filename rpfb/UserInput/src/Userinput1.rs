fn main() {
    let mut text = String::new();
    println!("\nEnter some text:");
    std::io::stdin().read_line(&mut text).unwrap();
    println!("You have entered: {}", text);
}
