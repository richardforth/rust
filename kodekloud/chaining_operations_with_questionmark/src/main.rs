fn read_username() -> Result<String, String> {
    // Simulate reading from a file or input
    Ok("john_doe".to_string())
}

fn validate_username(username: &str) -> Result<(), String> {
    if username == "john_doe" {
        Ok(())
    } else {
        Err("Invalid username".to_string())
    }
}


fn main() -> Result<(), String> {
    // First, read the username
    let username = read_username()?; // If this fails, it will return the error
    
    // Then, validate the username
    validate_username(&username)?; // If this fails, it will return the error
    
    // If both succeeded, print the success message
    println! ("Username is valid!");
    
    Ok(()) // Everything is okay, so we return Ok
}
