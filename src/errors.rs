use std::string::FromUtf8Error;

pub enum Error {
    ParsingError,
    ConversionError,
}

impl From<FromUtf8Error> for Error {
    fn from(_: FromUtf8Error) -> Self {
        return Error::ConversionError;
    }
}
