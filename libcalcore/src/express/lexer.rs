use std::{str::Chars, iter::Peekable};

use super::error::LexerError;

/// Some section about to the symbols:
///
/// `OpenPh` => `"("`, `ClosePh` => `")"`, `Add` => `"+"`, `Subtract` => `"-"`, `Multiply` => `"*"`, `Divide` => `"/"`, `Exponential` => `"^"`
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OpenPh,
    ClosePh,
    Plus,
    Minus,
    Multiply,
    Division,
    Exponential,
    Percent,
    Number(f64),
    EOF,
}

pub static NUMBER_CHARS: &str = "0123456789.";
pub static OPERATOR_CHARS: &str = "()+-*/^% \n";

pub fn tokenize<T: FromIterator<Token>>(expr_str: &str) -> Result<T, LexerError> {
    Ok(Lexer::new(expr_str).collect()?.into_iter().collect::<T>())
}

pub struct Lexer<'a> {
    src: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src: src.chars().peekable(),
        }
    }

    pub fn read(&mut self) -> Result<Token, LexerError> {
        if let Some(ch) = self.src.next() {
            match ch {
                '(' => Ok(Token::OpenPh),
                ')' => Ok(Token::ClosePh),
                '+' => Ok(Token::Plus),
                '-' => Ok(Token::Minus),
                '*' => Ok(Token::Multiply),
                '/' => Ok(Token::Division),
                '^' => Ok(Token::Exponential),
                '%' => Ok(Token::Percent),
                _ if NUMBER_CHARS.contains(ch) => {
                    let mut buf = String::from(ch);
                    while let Some(ch) = self.src.peek().filter(|ch| NUMBER_CHARS.contains(**ch)) {
                        buf.push(*ch);
                        self.src.next();
                    }
                    Ok(Token::Number(buf.parse::<f64>().map_err(|_| LexerError::InvalidNumber(buf))?))
                }
                _ => {
                    Err(LexerError::UnknowChar(ch))
                }
            }
        } else {
            Ok(Token::EOF)
        }
    }

    pub fn collect(mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = vec![];
        loop {
            let token = self.read()?;
            if let Token::EOF = token {
                break;
            }
            tokens.push(token);
        }
        Ok(tokens.into_iter().collect())
    }
}