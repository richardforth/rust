/*
  This program demonstrates the key concepts related to Rust functions as discussed in the chapter.
  It includes:
      1. A function to calculate the area of a circle using the formula: area = * r2.
      2. A recursive function to compute the factorial of a number.
      3. Functions that illustrate parameter passing (by value and by reference),
      ownership, borrowing, explicit return statements, and implicit returns.
      4. Examples of nested function calls and expression evaluation.
*/
use std::f64::consts::PI;
// Function to compute the area of a circle.
// Formula: area = * r2
// The function takes the radius (r) as a parameter and returns the computed area.
fn circle_area(r: f64) -> f64 {
    // Here, the final expression without a semicolon is implicitly returned.
    PI * r * r
}

// Recursive function to compute factorial of a given number.
// Demonstrates value passing and pattern matching.
// For any n < 2, factorial is 1; otherwise, it multiplies n by factorial(n - 1).
fn factorial(n: u64) -> u64 {
    if n < 2 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// Function to demonstrate parameter passing by reference.
// It takes a string s7lice, prints its content, and returns its length.
fn print_and_count(text: &str) -> usize {
    println!("The provided text is: '{}'", text);
    text.len() // Implicit return of the length of the text.
}

// Function that computes both the sum and the product of two integers.
// It shows the use of explicit return with a tuple containing both calculated values.
fn sum_and_product(a: i32, b: i32) -> (i32, i32) {
    let sum = a + b;
    let product = a * b;
    // Explicit return statement ensures the tuple is returned.
    return (sum, product);
}

fn main() {
    // Demonstrate function invocation and execution.
    // Calculate the area of a circle with a given radius using the circle_area function.
    let radius = 5.0;
    let area = circle_area(radius);
    println!("Area of a circle with radius {} is {:.2}", radius, area);
    // Compute the factorial of 5 using the recursive factorial function.
    let num = 5;
    let fact = factorial(num);
    println!("Factorial of {} is {}", num, fact);
    // Demonstrate parameter passing by reference.
    let message = "Hello, Rust!";
    let count = print_and_count(message);
    println!("Length of the message is {}", count);
    // Calculate the sum and product of two integers.
    let (sum, product) = sum_and_product(3, 7);
    println!("Sum: {}, Product: {}", sum, product);
    // Demonstrate implicit return in a block expression.
    let implicit_result = {
        let x = 10;
        let y = 20;
        // This expression is implicitly returned as it lacks a semicolon.
        x + y
    };

    println!("Result of implicit return block: {}", implicit_result);

    // Nested function calls example:
    // Calculate the factorial of 3 to use as the radius for computing the area of a circle.
    let radius_from_factorial = factorial(3) as f64;
    let dynamic_area = circle_area(radius_from_factorial);
    println!("Area of a circle with radius (3!): {:.2}", dynamic_area);
}
