fn main() {
    let mut num1_str = String::new();
    let mut num2_str = String::new();

    println!("Enter a floating point number:");

    std::io::stdin().read_line(&mut num1_str).unwrap();

    println!("Enter another floating point number:");

    std::io::stdin().read_line(&mut num2_str).unwrap();

    let num1: f64 = num1_str.trim().parse().unwrap();
    let num2: f64 = num2_str.trim().parse().unwrap();

    let prod: f64 = num1 * num2;

    println!("\nnum1 = {}, num2 = {}\nprod = {}\n", num1, num2, prod);
}
