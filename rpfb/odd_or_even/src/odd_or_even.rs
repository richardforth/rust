fn main() {
    let mut num_str = String::new();
    println!("Enter a number:");
    std::io::stdin().read_line(&mut num_str).unwrap();

    let num: isize = num_str.trim().parse().unwrap();

    if num % 2 == 1 {
        println!("\n{} is odd.\n", num);
    } else {
        println!("\n{} is even.", num);
    }
}
