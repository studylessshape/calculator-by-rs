use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct LexerError;

#[derive(Debug)]
pub enum TokenError {
    InvalidNumber(String),
    UnknowChar(char),
}

impl Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            TokenError::InvalidNumber(fnum) => format!("Invalid number: '{}'! | TokenError::InvalidNumber", fnum),
            TokenError::UnknowChar(ch) => format!("Unknow char: '{}'! | TokenError::UnknowChar", ch),
        };
        write!(f, "{}", message)
    }
}

impl Error for TokenError {}