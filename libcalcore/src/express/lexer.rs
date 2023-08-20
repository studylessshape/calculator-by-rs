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

pub static NUMBER_STR: &str = "0123456789.";
pub static OPERATOR_STR: &str = "()+-*/^% \n";

pub fn get_token(expr_str: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::new();

    let mut stor_str = String::new();

    for ch in expr_str.chars() {
        // clear the contains of `store string`
        if OPERATOR_STR.contains(ch) {
            if stor_str.len() > 0 {
                match parse_num(&stor_str) {
                    Ok(num) => tokens.push(Token::Number(num)),
                    Err(e) => return Err(e),
                };
                stor_str.clear();
            }
            // push operator symbol
            match ch {
                '(' => tokens.push(Token::OpenPh),
                ')' => tokens.push(Token::ClosePh),
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Multiply),
                '/' => tokens.push(Token::Division),
                '^' => tokens.push(Token::Exponential),
                '%' => tokens.push(Token::Percent),
                _ => {}
            };
        } else if NUMBER_STR.contains(ch) {
            stor_str.push(ch);
        } else {
            return Err(LexerError::UnknowChar(ch));
        }
    }
    // check if has the lastest number
    if stor_str.len() > 0 {
        match parse_num(&stor_str) {
            Ok(num) => tokens.push(Token::Number(num)),
            Err(e) => return Err(e),
        };
    }

    Ok(tokens)
}

fn parse_num(str: &str) -> Result<f64, LexerError> {
    match str.parse::<f64>() {
        Ok(fnum) => Ok(fnum),
        Err(_) => Err(LexerError::InvalidNumber(str.to_string())),
    }
}
