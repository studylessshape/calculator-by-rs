use std::mem;

use crate::express::{lexer::{Lexer, Token}, error::LexerError};

pub enum Primary {
    Num(f64),
    Expr(Box<BinExpr>),
}

const OPERATORS: &[(char, usize)] = &[('+', 10), ('-', 10), ('*', 100), ('/', 100)];

pub struct BinExpr {
    op_code: char,
    lhs: Primary,
    rhs: Primary,
}

pub struct LookAhead<'a> {
    peek: Token,
    lexer: Lexer<'a>,
}

impl<'a> LookAhead<'a> {
    pub fn new(src: &'a str) -> Result<Self, LexerError> {
        let mut lexer = Lexer::new(src);
        Ok(Self { peek: lexer.read()?, lexer })
    }

    pub fn peek(&self) -> &Token {
        &self.peek
    }

    pub fn next(&mut self) -> Result<Token, LexerError> {
        Ok(mem::replace(&mut self.peek, self.lexer.read()?))
    }
}

