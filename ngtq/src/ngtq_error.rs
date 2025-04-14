use std::{error::Error, fmt::{self}};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum NGTQErrorType {
    Initialisation(String),
    IdQueue(String),
    CategoryQueue(String),
    Serialisation(String),
    ServerError(String)
}

#[derive(Debug, Serialize, Deserialize)]
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
            NGTQErrorType::Initialisation(error) => write!(f,"Initialisation Error {}", error),
            NGTQErrorType::IdQueue(error) => write!(f, "Id Queue Error {}", error),
            NGTQErrorType::CategoryQueue(error) => write!(f, "Category Queue Error {}", error),
            NGTQErrorType::Serialisation(error) => write!(f, "Serialisation Error {}", error),
            NGTQErrorType::ServerError(error) => write!(f, "Server Unexpected error occured: {}", error)
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