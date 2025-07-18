fn main() {
    let opt: Option<i32> = None; // Using unwrap_or_else to handle None case
    let value = opt.unwrap_or_else(|| {
        println! ("No value found, returning default");
        10 // Default value when opt is None
    });
    println! ("The value is: {}", value); // Output: The value is: 10
}
