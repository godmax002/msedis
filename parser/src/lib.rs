use std::{fmt::Error, str::Utf8Error, num::{ParseIntError, ParseFloatError}, ops::Bound};

pub struct Argument {
    pub pos: usize,
    pub len: usize,
}

pub struct ParsedCommand<'a> {
    data: &'a [u8],
    pub argv: Vec<Argument>,
}

pub struct OwnedParsedCommand {
    data: Vec<u8>,
    pub argv: Vec<Argument>,
}

pub enum ParseError {
    Incomplete,
    BadProtocol(String),
    InvalidArgument,
}

impl ParseError {
    pub fn is_incomplete(&self) -> bool {

    }

    fn response_string(&self) -> String {

    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt:: Result {

    }
}

impl Error for ParseError {
    fn description(&self) -> &str {

    }

    fn cause(&self) -> Option<&dyn Error> {

    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        
    }
}
impl From<ParseIntError> for ParseError {
    fn from(_: ParseIntError) -> Self {
        
    }
}

impl From<ParseFloatError> for ParseError {
    fn from(_: ParseFloatError) -> Self {
        
    }
}

impl OwnedParsedCommand {
    pub fn new(dat: Vec<u8>, argv: Vec<Argument>) -> Self {

    }

    pub fn get_command(&self) -> ParsedCommand {

    }
}

impl<'a> ParsedCommand<'a> {
    pub fn new(data: &[u8], argv: Vec<Argument>) -> ParsedCommand {

    }

    pub fn get_f64_bound(&self, pos: usize) -> Result<Bound<f64>, ParseError> {

    }

    pub fn get_f64(&self, pos: usize) -> Result<f64, ParseError> {

    }

    pub fn get_i64(&self, pos: usize) -> Result<i64, ParseError> {

    }

    pub fn get_str(&self, pos: usize) -> Result<&str, ParseError> {

    }

    pub fn get_vec(&self, pos: usize) -> Result<Vec<u8>, ParseError> {

    }

    pub fn get_slice(&self, pos: usize) -> Result<&[u8], ParseError> {

    }

    pub fn get_data(&self) -> &'a [u8] {

    }

    pub fn into_owned(self) -> OwnedParsedCommand {

    }

    pub fn to_owned(self) -> OwnedParsedCommand {

    }



}

impl<'a> fmt::Debug for ParsedCommand<'a> {

}

fn parse_int(input: &[u8], len: usize, name: &str) -> Result<(Option<usize>, usize), ParseError> {

}

pub fn parse(input: &[u8]) -> Result<(ParsedCommand, usize), ParseError> {

}

pub struct Parser {
    data: Vec<u8>,
    pub position: usize,
    pub written: usize,
}

impl Default for Parser {
    fn default() -> Self {
        
    }
}

impl Parser {
    pub fn new() -> Parser {

    }

    pub fn allocate(&mut self) {

    }

    pub fn get_mut(&mut self) -> &mut Vec<u8> {

    }

    pub fn is_incomplete(&self) -> bool {

    }

    pub fn next(&mut self) -> Result<ParsedCommand, ParseError> {

    }

}

impl fmt::Debug for Parser {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {

    }
}