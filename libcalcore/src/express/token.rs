use super::error::TokenError;

/// Some section about to the symbols:
/// 
/// `OpenPh` => `"("`, `ClosePh` => `")"`, `Add` => `"+"`, `Subtract` => `"-"`, `Multiply` => `"*"`, `Divide` => `"/"`, `Exponential` => `"^"`
pub enum Token {
    OpenPh,
    ClosePh,
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponential,
    Mod,
    IntNumber(i64),
    FloatNumber(f64)
}

pub static NUMBER_STR: &str = "0123456789.";
pub static OPERATOR_STR: &str = "()+-*/^%";

pub fn get_token(expr_str: &str) -> Result<Vec<Token>, TokenError> {
    let mut tokens = Vec::new();

    let mut stor_str = String::new();

    for ch in expr_str.chars() {
        // clear the contains of `store string`
        if OPERATOR_STR.contains(ch) {
            if stor_str.len() > 0 {
                if stor_str.contains('.') {
                    let fnum = match stor_str.parse::<f64>() {
                        Ok(r) => r,
                        Err(_) => return Err(TokenError),

                    };
                    tokens.push(Token::FloatNumber(fnum));
                } else {
                    let inum = match stor_str.parse::<i64>() {
                        Ok(r) => r,
                        Err(_) => return Err(TokenError),
                    };
                    tokens.push(Token::IntNumber(inum))
                }
                stor_str.clear();
            }
            match ch {
                '(' => tokens.push(Token::OpenPh),
                ')' => tokens.push(Token::ClosePh),
                '+' => tokens.push(Token::Add),
                '-' => tokens.push(Token::Subtract),
                '*' => tokens.push(Token::Multiply),
                '/' => tokens.push(Token::Divide),
                '^' => tokens.push(Token::Exponential),
                '%' => tokens.push(Token::Mod),
                _ => {},
            };
        } else if NUMBER_STR.contains(ch) {
            stor_str.push(ch);
        } else {
            return Err(TokenError);
        }
    }

    return Ok(tokens);
}