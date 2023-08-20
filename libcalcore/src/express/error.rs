use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum LexerError {
    InvalidNumber(String),
    UnknowChar(char),
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            LexerError::InvalidNumber(fnum) => format!("Invalid number: '{}'! | TokenError::InvalidNumber", fnum),
            LexerError::UnknowChar(ch) => format!("Unknow char: '{}'! | TokenError::UnknowChar", ch),
        };
        write!(f, "{}", message)
    }
}

impl Error for LexerError {}

#[derive(Debug)]
pub enum ParserError {
    AddExpr,
    MulExpr,
    ExponExpr,
    UnionExpr(i32),
    PhExpr,
    NumberExpr,
}

#[derive(Debug)]
pub struct CalculateError;