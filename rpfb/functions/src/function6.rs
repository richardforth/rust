fn sum(a: isize, b: isize) -> isize {
    let sum = a + b;
    return sum;
}
fn difference(a: isize, b: isize) -> isize {
    let d = a - b;
    return d;
}

fn product(a: isize, b: isize) -> isize {
    let p = a * b;
    return p;
}

fn quotient(a: isize, b: isize) -> f64 {
    let q: f64 = (a as f64) / (b as f64);
    return q;
}

fn main() {
    let mut num1_str = String::new();
    let mut num2_str = String::new();
    println!("Enter a number:");
    std::io::stdin().read_line(&mut num1_str).unwrap();
    let num1: isize = num1_str.trim().parse().unwrap();

    println!("Enter another number:");
    std::io::stdin().read_line(&mut num2_str).unwrap();
    let num2: isize = num2_str.trim().parse().unwrap();

    let s = sum(num1, num2);
    let d = difference(num1, num2);
    let p = product(num1, num2);
    let q = quotient(num1, num2);
    println!(
        "\nsum = {}\nDifference = {}\nProduct = {}\nQuotient = {}",
        s, d, p, q
    );
}
