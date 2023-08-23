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

        let valid_tokens = tokenize::<Vec<Token>>(valid_express);
        let invalid_tokens = tokenize::<Vec<Token>>(invalid_express);

        println!("Valid_tokens: [{:?}]", valid_tokens);
        println!("Invalid_tokens: [{:?}]", invalid_tokens);

        assert!(valid_tokens.is_ok());
        assert!(invalid_tokens.is_err());
    }

    #[test]
    fn test_parser() {
        let express = "1+2/3*4+(1%4)^1.2";

        let lexer_res = tokenize(express);

        println!("Lexer: [{:?}]", lexer_res);

        assert!(lexer_res.is_ok());

        if let Ok(lexer) = lexer_res {
            let parser_res = AST::parse(lexer);
            println!("Parser: {:?}", parser_res);

            if let Ok(parser) = parser_res {
                let cal_res = parser.eval();

                println!("Calculator: [{:?}]", cal_res);

                assert!(cal_res.is_ok());
                if let Ok(cal) = cal_res {
                    assert_eq!(cal, 4.66666667);
                }
            }
        }
    }

    #[test]
    fn test_lookahead_parser() {
        use lookahead::LookAhead;
        let mut parser = LookAhead::try_from("1+2/3*4+(1%4)^1.2").unwrap();
        let ast = parser.parse_expr().unwrap();

        assert!(f64::abs(ast.eval().unwrap() - (1.0 + 2.0 / 3.0 * 4.0 + f64::powf(1.0 % 4.0, 1.2))) < f64::EPSILON)
    }
}
