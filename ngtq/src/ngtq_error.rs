use std::{error::Error, fmt::{self}};

#[derive(Debug)]
pub enum NGTQErrorType {
    INITIALISATION(String),
    IDQUEUE(String),
    CATEGORYQUEUE(String)
}

#[derive(Debug)]
pub struct NGTQError {
    pub error_type: NGTQErrorType,
    pub error_description: String
}

impl NGTQError {
    pub fn generate_error(error_type: NGTQErrorType, description: String) -> Self {
        NGTQError { error_type: error_type, error_description: description }
    }
}

impl fmt::Display for NGTQError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.error_type {
            NGTQErrorType::INITIALISATION(error) => write!(f,"Initialisation Error {}", error),
            NGTQErrorType::IDQUEUE(error) => write!(f, "Id Queue Error {}", error),
            NGTQErrorType::CATEGORYQUEUE(error) => write!(f, "Category Queue Error {}", error),
        }
    }
}

impl Error for NGTQError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}