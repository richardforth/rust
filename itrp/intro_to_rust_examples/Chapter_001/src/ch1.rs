fn main() {
    // Print the classic "Hello, World!" message.
    println!("Hello, World!");

    // Demonstrate block scoping with local variables.
    {
        let welcome_msg = "Welcome to Rust Programming!";
        println!("{}", welcome_msg);
    } // 'welcome_msg' goes out of scope here

    // Using important formula: Calculate the area of a circle.
    // Equation: area = pi * r^2
    let radius: f64 = 5.0;
    let area = calculate_circle_area(radius);
    println!(
        "The area of a circle with radius {:2} is {:2}",
        radius, area
    );

    // Demonstration of an algorithm: Compute the factorial of a number recursively.
    let number: u32 = 6;
    let fact = factorial(number);
    println!("The factorial of {} is {}", number, fact);

    // An example of a function that implicitly returns the unit value, ()
    display_message("This message shows an implicit unit return from a function.");

    // Function to calculate the area of a circle given its radius.
    // Uses the formula area = pi * r^2.
    fn calculate_circle_area(radius: f64) -> f64 {
        // The constant PI is obtained from Rust's standard library.
        std::f64::consts::PI * radius * radius
    }
   
    // Recursive function to compute the factorial of a non-negative integer.
    // Base case: factorial(0) = 1. Recursive case: factorial(n) - n * factorial(n -1).
    fn factorial(n: u32) -> u32 {
        if n == 0 {
            1 // Implicitly returns the unit value () in main, while here returns 1 as base case.
        } else {
            n * factorial(n -1)
        }
    }

    fn display_message(msg: &str) {
        println!("{}", msg);
    }
}
