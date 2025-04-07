use crate::*;
use lexer::lexer::Lexer;
use lexer::token::Token;

#[test]
fn test_lexer_easy() {
    let source = concat!(
        "function main() {\n",
        "}\n"
    );

    let mut lexer = Lexer::new(source.to_string());
    lexer.lexer().unwrap();

    let expected_tokens: Vec<Token> = vec![
        Token::Keyword("function".to_string()),
        Token::Identifier("main".to_string()),
        Token::LeftParenthesis,
        Token::RightParenthesis,
        Token::LeftBrace,
        Token::RightBrace
    ];

    let tokens = lexer.get_tokens();

    assert_eq!(tokens, &expected_tokens);
}


#[test]
fn test_lexer_from_file() {
    let source = concat!(
        "function main() {\n",
        "    let value: float = 10.2f;\n",
        "    let value2: int = 10;\n",
        "    print(\"Hello World\");\n",
        "}\n"
    );

    let mut lexer = Lexer::new(source.to_string());
    lexer.lexer().unwrap();

    let expected_tokens: Vec<Token> = vec![
        Token::Keyword("function".to_string()),
        Token::Identifier("main".to_string()),
        Token::LeftParenthesis,
        Token::RightParenthesis,
        Token::LeftBrace,
        Token::Keyword("let".to_string()),
        Token::Identifier("value".to_string()),
        Token::Colon,
        Token::Keyword("float".to_string()),
        Token::Equal,
        Token::FloatLiteral(10.2),
        Token::Semicolon,
        Token::Keyword("let".to_string()),
        Token::Identifier("value2".to_string()),
        Token::Colon,
        Token::Keyword("int".to_string()),
        Token::Equal,
        Token::IntegerLiteral(10),
        Token::Semicolon,
        Token::Identifier("print".to_string()),
        Token::LeftParenthesis,
        Token::StringLiteral("Hello World".to_string()),
        Token::RightParenthesis,
        Token::Semicolon,
        Token::RightBrace
    ];

    let tokens = lexer.get_tokens();

    assert_eq!(tokens, &expected_tokens);
}


#[test]
fn test_lexer_identefier_underscore() {
    let source = concat!(
        "function test_function() {\n",
        "}\n"
    );

    let mut lexer = Lexer::new(source.to_string());
    lexer.lexer().unwrap();

    let expected_tokens: Vec<Token> = vec![
        Token::Keyword("function".to_string()),
        Token::Identifier("test_function".to_string()),
        Token::LeftParenthesis,
        Token::RightParenthesis,
        Token::LeftBrace,
        Token::RightBrace
    ];

    let tokens = lexer.get_tokens();

    assert_eq!(tokens, &expected_tokens);
}