fn main() {
    let mut num1_str = String::new();
    let mut num2_str = String::new();

    println!("Enter a number:");

    std::io::stdin().read_line(&mut num1_str).unwrap();

    println!("Enter another number:");

    std::io::stdin().read_line(&mut num2_str).unwrap();

    let num1: isize = num1_str.trim().parse().unwrap();
    let num2: isize = num2_str.trim().parse().unwrap();

    let sum: isize = num1 + num2;

    println!("\nnum1 = {}, num2 = {}\nsum = {}\n", num1, num2, sum);
}
