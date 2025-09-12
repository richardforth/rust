use std::fmt;

#[derive(Debug)]
enum NetworkError {
    Disconnected,
    Timeout,
    Failed,
}

#[derive(Debug)]
enum FileSystemError {
    FileNotFound,
    PermissionDenied,
    ReadOnly,
}


#[derive(Debug)]
enum CustomError {
    Network(NetworkError), // wraps a Network error
    File(FileSystemError), // wraps a Filesystem error
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NetworkError::Disconnected => write! (f, "The network connection has disconnected"),
            NetworkError::Timeout => write! (f, "The network connection timed out"),
            NetworkError::Failed => write! (f, "The network connection failed"),
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::Network(e) => write!(f, "Network error: {}", e),
            CustomError::File(e) => write!(f, "Filesystem error: {}", e),
        
        }
    }
}

impl fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileSystemError::FileNotFound => write!(f, "The specified file was not found"),
            FileSystemError::ReadOnly => write!(f, "Unable to write to the specified file"),
            FileSystemError::PermissionDenied => write!(f, "Permission denied"),
        }
    }
}


// Implement error trait to work seamlessly wth Rust's Error handling ecosystem
// using standard library features
impl std::error::Error for NetworkError {}
impl std::error::Error for CustomError {}
impl std::error::Error for FileSystemError {}

// Implement the From trait for converting NetworkError into CustomError
impl From<NetworkError> for CustomError {
    fn from(error: NetworkError) -> Self {
        CustomError::Network(error) // Wraps NetworkError inside CustomError::Network
    }
}

// Implement the From trait for converting FileSystemError into CustomError
impl From<FileSystemError> for CustomError {
    fn from(error: FileSystemError) -> Self {
        CustomError::File(error) // Wraps FileSystemError inside CustomError::File
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_human_readable_message() {
        let err = FileSystemError::ReadOnly;
        assert_eq!(err.to_string(), "Unable to write to the specified file");
        // ^ note: “specified”, not “specifed”
    }

    #[test]
    fn result_has_expected_variant() {
        let error1: Result<(), FileSystemError> = Err(FileSystemError::ReadOnly);
        assert!(matches!(error1, Err(FileSystemError::ReadOnly)));
        // or: assert_eq!(error1.unwrap_err(), FileSystemError::ReadOnly);
    }
}
