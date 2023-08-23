use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum CalError {
    LexError(LexerError),
    SyntaxError(String),
}

impl CalError {
    pub fn lex<T>(le: LexerError) -> Result<T, Self> {
        Err(Self::LexError(le))
    }

    pub fn syn<T>(msg: &str) -> Result<T, Self> {
        Err(Self::SyntaxError(String::from(msg)))
    }
}

impl Display for CalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalError::LexError(le) => {
                write!(f, "Lex error:")?;
                le.fmt(f)
            },
            CalError::SyntaxError(se) => {
                write!(f, "Syntax error: {se}")
            },
        }
    }
}


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