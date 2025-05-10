fn main() {
    let mut num_str = String::new();

    println!("Enter a number:");

    std::io::stdin().read_line(&mut num_str).unwrap();

    let num: isize = num_str.trim().parse().unwrap();

    if num > 0 {
        println!("\n{} is positive.\n", num);
    } else if num < 0 {
        println!("\n{} is negative.\n", num);
    } else {
        println!("\n{} is zero.\n", num);
    }
}
