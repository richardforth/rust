fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err(String::from("Division by zero error"));
    }
    Ok(a / b)
}


fn calculate() -> Result<(), String> {
    let result = divide(10.0, 0.0)?;
    println! ("Result: {}", result);
    Ok(())
}

fn main() {
    if let Err(e) = calculate() {
        println! ("Failed to calculate: {}", e);
    }
}
