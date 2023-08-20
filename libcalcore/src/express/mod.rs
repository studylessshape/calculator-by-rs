pub mod error;
pub mod lexer;
pub mod parser;

#[cfg(test)]
pub mod tests {

    use super::lexer::*;
    use super::parser::*;

    #[test]
    fn test_token() {
        let valid_express = "1+2*3/4+(1%4)^1.2";
        let invalid_express = "1+2.2.2";

        let valid_tokens = get_token(valid_express);
        let invalid_tokens = get_token(invalid_express);

        println!("Valid_tokens: [{:?}]", valid_tokens);
        println!("Invalid_tokens: [{:?}]", invalid_tokens);

        assert!(valid_tokens.is_ok());
        assert!(invalid_tokens.is_err());
    }

    #[test]
    fn test_parser() {
        let express = "1+(2-3)*4/5%6^7";

        let lexer_res = get_token(express);

        println!("Lexer: [{:?}]", lexer_res);

        assert!(lexer_res.is_ok());

        if let Ok(lexer) = lexer_res {
            let parser_res = Parser::parse_token(lexer);

            if let Ok(parser) = parser_res {
                let cal_res = parser.calculate();

                println!("Calculator: [{:?}]", cal_res);

                assert!(cal_res.is_ok());
                if let Ok(cal) = cal_res {
                    assert_eq!(cal, 0.2);
                }
            }
        }
    }
}
