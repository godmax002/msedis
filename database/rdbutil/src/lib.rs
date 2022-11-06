pub enum EncodeError {
    IOError(io::Error),
    OverflowError,
}

impl From<io::Error> for EncodeError {
    fn from(err: io::Error) -> Self {

    }
}

impl From<EncodeError> for io::Error {
    fn from(err: EncodeError) -> Self {

    }
}

pub fn encode_i64<W: io::Write>(value: i64, enc: &mut W) -> Result<(), EncodeError> {

}

