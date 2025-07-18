fn main() {
    let result: Result<i32, &str> = Err("An error occurred");
    // Using unwrap_or_else to handle Err case
    let value = result.unwrap_or_else(|err| {
        println! ("Error encountered: {}", err);
        -1 // Fallback value if result is Err
    });

    println! ("The result is: {}", value); // Output: The result is: -1
}
