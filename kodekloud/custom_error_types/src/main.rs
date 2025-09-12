use std::fmt;

#[derive(Debug)]
enum NetworkError {
    Disconnected,
    Timeout,
}

#[derive(Debug)]
enum CustomError {
    NotFound,
    PermissionDenied,
    ConnectionFailed,
    Network(NetworkError), // wraps a Network error
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NetworkError::Disconnected => write! (f, "Network disconnected"),
            NetworkError::Timeout => write! (f, "Network timeout"),
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::NotFound => write!(f, "Resource not found"),
            CustomError::PermissionDenied => write!(f, "Permission denied"),
            CustomError::ConnectionFailed => write!(f, "Connection failed"),
            CustomError::Network(e) => write!(f, "Network error: {}", e),
        }
    }
}

// Implement error trait to work seamlessly wth Rust's Error handling ecosystem
// using standard library features
impl std::error::Error for NetworkError {}
impl std::error::Error for CustomError {}

// Implement the From trait for converting NetworkError into CustomError
impl From<NetworkError> for CustomError {
    fn from(error: NetworkError) -> Self {
        CustomError::Network(error) // Wraps NetworkError inside CustomError:: Network
    }
}

// Simulating a function that returns a NetworkErrоr
fn connect_to_network() -> Result<(), NetworkError>{
    Err(NetworkError::Disconnected) // Simulating a network error
}

// Function that uses the From trait to convert NetworkError into CustomError
fn perform_task() -> Result<(), CustomError> {
    // connect_to_network returns Result<(), NetworkError>.
    // The 7 operator will convert NetworkError into CustomError using the From trait.
    connect_to_network()?;
    Ok(())
}

fn find_user(user_id: u32) -> Result<String, CustomError> {
    if user_id == 0 {
        return Err(CustomError::NotFound);
    } else if user_id == 1 {
        return Err(CustomError::PermissionDenied);
    } else if user_id == 2 {
        return Err(CustomError::ConnectionFailed);
    }
    Ok(String::from("User found"))
}

fn main() {
    match find_user(0) {
        Ok(user) => println!("Success: {}", user),
        Err(e) => println!("Error: {}", e),
    }
    match find_user(1) {
        Ok(user) => println!("Success: {}", user),
        Err(e) => println!("Error: {}", e),
    }
    match find_user(2) {
        Ok(user) => println!("Success: {}", user),
        Err(e) => println!("Error: {}", e),
    }
    match find_user(5) {
        Ok(user) => println!("Success: {}", user),
        Err(e) => println!("Error: {}", e),
    }

    match perform_task() {
        Ok(_) => println!("Task completed successfully"),
        Err(e) => println!("Task failed: {}", e),
    }
}
 
