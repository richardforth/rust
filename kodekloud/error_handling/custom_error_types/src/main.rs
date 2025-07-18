// Custom Error Type
#[derive (Debug)]
enum CustomError {
    NotFound,
    PermissionDenied,
    ConnectionFailed,
}

fn main() {
    let error = CustomError::NotFound;
    println!("{:?}", error);
}

