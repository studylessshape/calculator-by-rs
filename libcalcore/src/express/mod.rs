pub mod token;
pub mod lexer;
pub mod error;

pub enum Either<L, R> {
    Left(L),
    Right(R)
}

#[cfg(test)]
pub mod tests {
    use crate::express::lexer::*;

    use super::token::*;

    #[test]
    fn test_token() {
        let valid_express = "1+2*3/4+(1%4)^1.2";
        let invalid_express = "1+2.2.2";

        let invalid_tokens = get_token(invalid_express);
        let valid_tokens = get_token(valid_express);

        println!("Valid_tokens: [{:?}]", valid_tokens);
        println!("Invalid_tokens: [{:?}]", invalid_tokens);

        let lexer_res = parse_token(valid_tokens.unwrap());

        println!("{:?}", lexer_res);

        assert!(invalid_tokens.is_err());
    }
}