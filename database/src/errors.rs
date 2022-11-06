use std::io;
use std::fmt;
use std::error::Error;
use std::num::ParseFloatError;
use std::num::ParseIntError;
use std::str::Utf8Error; 

pub enum OperationError {
    OverflowError,
    NotANumberError,
    ValueError(String),
    UnknownKeyError,
    WrongTypeError,
    OutOfBoundsError,
    IOError(io::Error),
}

impl OperationError {
    fn message(&self) -> &str {

    }
}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.message().fmt(f)
    }
}

impl Error for OperationError {
    fn description(&self) -> &str {
        self.message()
    }
}

impl From<Utf8Error> for OperationError {
    
}

impl From<ParseIntError> for OperationError {
    
}

impl From<ParseFloatError> for OperationError {
    
}

impl From<io::Error> for OperationError {
    
}