fn main() {
    let mut char_str = String::new();

    println!("Enter an alphabet:");

    std::io::stdin().read_line(&mut char_str).unwrap();

    let c: char = char_str.trim().parse().unwrap();

    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
            println!("\n{} is a vowel.\n", c)
        }
        _ => {
            println!("\n{} is a consonant.\n", c)
        }
    };
}
