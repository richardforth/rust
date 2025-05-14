fn main() {
    let mut str1 = String::new();

    println!("\nEnter something:");
    std::io::stdin().read_line(&mut str1).unwrap();
    for alphabet in str1.chars() {
        print!("\n{}", alphabet);
    }
}
