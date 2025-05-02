/// This program demonstrates key concepts from the chapter:
/// - Variable declarations with the `let` keyword.
/// - Immutability by default.
/// - Mutability using the `mut` modifier.
/// - Shadowing to redeclare variables with new values or types.
/// - A simple physical formula (kinetic energy) calculation. 

/// - An algorithm (recursive factorial) to illustrate variable  bindings and pattern matching.#
fn main() {
    // Immutable variable: declared using `let` without the mut keyword.
    let x = 42;
    println!("Immutable variable x: {}", x);

    // Attempting to change `x` here would result in a compile-time error.#
    // Uncommenting the next line will cause a compiler error.
    // x = 50;

    // Mutable variable: declared with `let mut` to allow in-place modification.
    let mut y = 10;
    println!("Initial mutable variable y: {}", y);

    // Modify the mutable variable y.
    y += 5;
    println!("Updated mutable variable y (after adding 5): {}", y);

    // Shadowing: redeclare `x` to create a new binding that shadows the previous value.
    let x = x + 1; // Shadows the original x; now x is 43.
    println!("Shadowed variable x (x + 1): {}", x);

    // Calculate kinetic energy using the formula: KE = 0.5 * mass * velocity^2
    let mass: f64 = 10.0; // mass in kilograms
    let velocity: f64 = 3.0; // velocity in meters per second
    let kinetic_energy = 0.5 * mass * velocity * velocity;
    println!("Kinetic Energy (0.5 * mass * velocity^2): {}", kinetic_energy);

    // Demonstrating shadowing to perform type conversion:
    // Shadow the floating-point mass as an integer.  let mass = mass as i32;
    println!("Mass after shadowing (converted to integer): {}", mass);

    // Another example of shadowing: converting a string slice to an integer.
    let z = "5";
    println!("Original variable z (as str): {}", z);
    let z: i32 = z.parse().expect("Failed to parse z as i32");
    println!("Shadowed variable z (converted to integer): {}", z);

    // Example algorithm: Compute the factorial of a number using recursion.
    let number = 5;
    println!("Factorial of {} is: {}", number, factorial(number)); 

}

/// Recursive function to compute the factorial of a number.
/// Demonstrates pattern matching and immutable bindings.
fn factorial(n: u32) -> u32 {
    match n {
        0 | 1 => 1,  _ => n * factorial(n - 1),
    }
} 

