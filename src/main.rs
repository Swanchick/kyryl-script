mod lexer;

use lexer::lexer::Lexer; 

fn main() {
    let mut lexer = Lexer::load("test.kys").unwrap();
    
    lexer.lexer().unwrap();

    for token in lexer.get_tokens() {
        println!("{}", token);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use lexer::token::Token;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::load("test.kys").unwrap();

        let expected_tokens: Vec<Token> = vec![
            Token::Identifier("print".to_string()),
            Token::LeftParenthesis,
            Token::StringLiteral("Hello World".to_string()),
            Token::RightParenthesis,
            Token::Semicolon
        ];

        let line = String::from("print(\"Hello World\");");

        let tokens = lexer.lex_line(&line).unwrap();

        assert_eq!(tokens, expected_tokens);
    }


    #[test]
    fn test_lexer_from_file() {
        // let mut lexer = Lexer::load("test.kys").unwrap();
    }
}

