use std::{mem, str::Chars};

use crate::express::{
    error::CalError,
    lexer::{Lexer, Token},
};

use super::OpSymbol;

#[derive(Debug)]
pub enum Expr {
    Num(f64),
    UnaryOp(Box<UnaryOp>),
    BinOp(Box<BinOp>),
}

#[derive(Debug)]
pub struct UnaryOp {
    op: OpSymbol,
    num: Expr,
}

#[derive(Debug)]
pub struct BinOp {
    op: OpSymbol,
    lhs: Expr,
    rhs: Expr,
}

pub struct LookAhead<I: Iterator<Item = char>> {
    peek: Token,
    lexer: Lexer<I>,
}

fn get_tok_prec(token: &Token) -> i8 {
    match token {
        Token::Plus | Token::Minus => 5,
        Token::Multiply | Token::Division | Token::Percent => 10,
        Token::Exponential => 15,
        _ => -1,
    }
}

impl Expr {
    pub fn eval(&self) -> Result<f64, ()> {
        match self {
            Expr::Num(n) => Ok(*n),
            Expr::UnaryOp(uo) => match uo.op {
                OpSymbol::Add => uo.num.eval(),
                OpSymbol::Subtract => Ok(-uo.num.eval()?),
                _ => Err(()),
            },
            Expr::BinOp(bo) => {
                let lhv = bo.lhs.eval()?;
                let rhv = bo.rhs.eval()?;
                match bo.op {
                    OpSymbol::Add => Ok(lhv + rhv),
                    OpSymbol::Subtract => Ok(lhv - rhv),
                    OpSymbol::Multiply => Ok(lhv * rhv),
                    OpSymbol::Divide => Ok(lhv / rhv),
                    OpSymbol::Mod => Ok(lhv % rhv),
                    OpSymbol::Caret => Ok(lhv.powf(rhv)),
                    _ => Err(()),
                }
            }
        }
    }
}

impl<'a> TryFrom<&'a str> for LookAhead<Chars<'a>> {
    type Error = CalError;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::new(value.chars())
    }
}

impl<I: Iterator<Item = char>> LookAhead<I> {
    pub fn new(chars: I) -> Result<Self, CalError> {
        let mut lexer = Lexer::new(chars);
        Ok(Self {
            peek: lexer.read()?,
            lexer,
        })
    }

    pub fn peek(&self) -> &Token {
        &self.peek
    }

    pub fn pop(&mut self) -> Result<Token, CalError> {
        Ok(mem::replace(&mut self.peek, self.lexer.read()?))
    }

    pub fn parse_expr(&mut self) -> Result<Expr, CalError> {
        let lhs = self.parse_unit()?;

        self.parse_binop(0, lhs)
    }

    fn parse_binop(&mut self, expr_prec: i8, mut lhs: Expr) -> Result<Expr, CalError> {
        // The loop continues when the current operator's priority is same as the next operator's
        loop {
            let tok_prec = get_tok_prec(self.peek());
            if tok_prec < expr_prec {
                return Ok(lhs);
            }

            let op: OpSymbol = self.pop()?.into();
            let mut rhs = self.parse_unit()?;

            let next_prec = get_tok_prec(self.peek());
            if tok_prec < next_prec {
                // The higher the op-priority the deeper this method recursive calls
                rhs = self.parse_binop(tok_prec + 1, rhs)?;
            }

            lhs = Expr::BinOp(Box::new(BinOp { op, lhs, rhs }))
        }
    }

    pub fn parse_unit(&mut self) -> Result<Expr, CalError> {
        let token = self.peek();
        match token {
            Token::Number(_) => self.parse_num(),
            Token::Plus | Token::Minus => self.parse_unary(),
            Token::OpenPh => self.parse_ph(),
            _ => CalError::syn(format!("Unrecognized token '{token:?}'").as_str()),
        }
    }

    pub fn parse_num(&mut self) -> Result<Expr, CalError> {
        let token = self.pop()?;
        if let Token::Number(n) = token {
            Ok(Expr::Num(n))
        } else {
            CalError::syn(format!("Expect {{number}}, get '{token:?}'").as_str())
        }
    }

    pub fn parse_unary(&mut self) -> Result<Expr, CalError> {
        let op_tok = self.pop()?;
        let op = match op_tok {
            Token::Plus => OpSymbol::Add,
            Token::Minus => OpSymbol::Subtract,
            _ => CalError::syn(format!("Expect '+' or '-', get '{op_tok:?}'").as_str())?,
        };
        let num = self.parse_num()?;

        Ok(Expr::UnaryOp(Box::new(UnaryOp { op, num })))
    }

    pub fn parse_ph(&mut self) -> Result<Expr, CalError> {
        // pop '('
        let _ = self.pop();
        let expr = self.parse_expr()?;
        // pop ')'
        let close_tok = self.pop()?;
        if !matches!(close_tok, Token::ClosePh) {
            return CalError::syn(format!("Expect ')', get '{close_tok:?}'").as_str());
        }
        Ok(expr)
    }
}
