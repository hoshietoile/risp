use std::convert::From;
use std::error;
use std::fmt;
use std::num;

#[derive(Debug)]
pub enum RispError {
    LexerInitialize(String),
    Parse(num::ParseFloatError),
    Peek(String),
    Read(String),
}

// Start: From implementations
impl From<num::ParseFloatError> for RispError {
    fn from(err: num::ParseFloatError) -> RispError {
        RispError::Parse(err)
    }
}
// End: From implementations

impl fmt::Display for RispError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use RispError::{LexerInitialize, Parse, Peek, Read};
        match self {
            LexerInitialize(str) => write!(f, "{}", str),
            Parse(err) => write!(f, "Parse error: {}", err),
            Peek(str) => write!(f, "{}", str),
            Read(str) => write!(f, "{}", str),
        }
    }
}

// TODO: Implementation of error::Error for RispError
impl error::Error for RispError {}
