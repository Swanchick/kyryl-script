mod lexer;

use lexer::lexer::Lexer; 

fn main() {
    let mut lexer = Lexer::new("test.kys").unwrap();
    let line = String::from("print(\"Hello World\");");

    let tokens = lexer.read_line(&line).unwrap();

    for token in tokens.iter() {
        println!("{}", token);
    }  
}


#[cfg(test)]
mod tests {
    use super::*;
    use lexer::token::Token;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("test.kys").unwrap();

        let expected_tokens: Vec<Token> = vec![
            Token::Identifier("print".to_string()),
            Token::LeftParenthesis,
            Token::StringLiteral("Hello World".to_string()),
            Token::RightParenthesis,
            Token::Semicolon
        ];

        let line = String::from("print(\"Hello World\");");

        let tokens = lexer.read_line(&line).unwrap();

        assert_eq!(tokens, expected_tokens);
    }
}

