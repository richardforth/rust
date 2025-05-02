fn main() {
    // Demonstration of scalar data types
    let integer: i32 = 42;
    let floating_point: f64 = 3.14159;
    let boolean: bool = true;
    let character: char = 'R';

    println!("Scalar Types:"); 
    println!("Integer (i32): {}", integer); 
    println!("Floating Point (f64): {}", floating_point);
    println!("Boolean (bool): {}", boolean);
    println!("Character (char): {}", character);

    // Demonstration of compund data types: Tuple and Array

    // Tuple: a heterogeneous collection of values
    let tuple: (i32, f64, char) = (100, 6.28, 'Z');
    // Descructure the tuple into individual components
    let (num, pi_twice, letter) = tuple;
    println!("\nTuple example:");
    println!("Desctructured Tuple: num = {}, pi_twice = {}, letter = {}", num, pi_twice, letter);

    // Array: a homogeneous collection with fixed size
    let array: [i32; 5] = [10, 20, 30, 40, 50];
    println!("Array Example:");
    for (index, value) in array.iter().enumerate() {
        println!("array[{}] = {}", index, value);
    }
   
    // Type reference and Data Annotation
    // Rust can infer types based on the assigned values
    let inferred_number = 123; // inferred as i32 by default
    let annotated_number: u32 = 123; 
    println!("\nType Inference vs Explicit Annotation:");
    println!("Inferred number (i32): {}", inferred_number);
    println!("Annotated number (u32): {}", annotated_number);

    // Important Equation and Algorithn: Quadratic Equation Solver
    // For a quadratic equation ax^2 + bx + c = 0
    // the roots are computed using the quadratic formula:
    //     x = (-b +/- sqrt(b ^2 -4ac)) / (2a)
    let a_coeff: f64 = 1.0;
    let b_coeff: f64 = -5.0;
    let c_coeff: f64 = 6.0;

    println!("\nSolving Quadratic Equation: {}x^2 + {}x + {} = 0", a_coeff, b_coeff, c_coeff);

    match quadratic_roots(a_coeff, b_coeff, c_coeff) {
        Some((root1, root2)) => {
            println!("The roots of the equation are: {} and: {}", root1, root2);
        }
        None => {
            println!("The equation has no real roots.");
        }
    }
}

// Function to compute the roots of a quadratic equation
// Returns Some((root1, root2)) if real roots exist, otherwise returns None
fn quadratic_roots(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        // No real roots exist when the discriminant is negative
        None
    } else {
        // Compute the square root of the discriminant
        let sqrt_disc = discriminant.sqrt();
        let two_a = 2.0 * a;
        // Calculate roots using the quadratic formula
        let root1 = (-b + sqrt_disc) / two_a;
        let root2 = (-b - sqrt_disc) / two_a;
        Some((root1, root2))
    }
}

