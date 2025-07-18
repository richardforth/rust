fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero is not allowed"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result = divide(10, 2);
    match result {
        Ok(value) => println! ("Result: {}", value),
        Err(e) => println! ("Error: {}", e),
    }
}
