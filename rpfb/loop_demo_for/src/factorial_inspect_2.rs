fn main() {
    let mut num_str = String::new();

    println!("Enter a number:");
    std::io::stdin().read_line(&mut num_str).unwrap();

    let num: isize = num_str.trim().parse().unwrap();
    let mut factorial: isize = 1;
    if num >= 0 {
        for i in  1..=num {
            factorial = factorial * i;
            println!("Factorial of {} is {}", i, factorial);
        }
        println!("\nFactorial of {} is {}", num, factorial);
    } else {
        println!("Factorial of a negative number cannot be calculated.\n");
    }
}
