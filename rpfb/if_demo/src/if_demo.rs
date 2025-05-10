fn main() {
    let mut num_str = String::new();
    println!("Enter a number:");
    std::io::stdin().read_line(&mut num_str).unwrap();

    let num: isize = num_str.trim().parse().unwrap();

    if num > 10 {
        println!("\n{} is greater than 10.\n", num);
    }
}
