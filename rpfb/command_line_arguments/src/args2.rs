use std::env;

fn sum(a: f64, b: f64) -> f64 {
    let sum = a + b;
    return sum;
}

fn difference(a: f64, b: f64) -> f64 {
    let d = a - b;
    return d;
}

fn product(a: f64, b: f64) -> f64 {
    let p = a * b;
    return p;
}

fn quotient(a: f64, b: f64) -> f64 {
    let q = a / b;
    return q;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("\nPlease enter exactly two numbers as arguments.")
    } else {
        let num1: f64 = args[1].trim().parse().unwrap();
        let num2: f64 = args[2].trim().parse().unwrap();
        let s = sum(num1, num2);
        let d = difference(num1, num2);
        let p = product(num1, num2);
        let q = quotient(num1, num2);
        println!(
            "\nSum = {}\nDifference = {}\nProduct = {}, Quotient = {}",
            s, d, p, q
        );
    }
}
