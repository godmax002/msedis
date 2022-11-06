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

    }
}

impl Error for OperationError {
    fn description(&self) -> &str {

    }
}

impl From<Utf8Error> for OperationError {
    fn from(_: Utf8Error) -> OperationError {

    }

}

impl From<ParseIntError> for OperationError {
    fn from(_: ParseIntError) -> OperationError {

    }
}

impl From<ParseFloatError> for OperationError {
    fn from(_: ParseFloatError) -> OperationError {

    }
}

impl From<io::Error> for OperationError {
    fn from(e: io::Error) -> OperationError {
        
    }
}

