use std::fmt;

const SYMBOLS: &str = "()[]{};:=+-*/<>~,^";

pub fn is_keyword(text: &str) -> bool {
    match text {
        "let" => true,
        "function" => true,
        "if" => true,
        "else" => true,
        "while" => true,
        "for" => true,
        "return" => true,
        "int" => true,
        "float" => true,
        "string" => true,
        "bool" => true,
        "true" => true,
        "false" => true,
        "using" => true,
        "void" => true,
        "struct" => true,
        "enum" => true,
        "list" => true,
        "tuple" => true,
        _ => false
        
    }
}

pub fn get_symbol(c: &str) -> Option<Token> {
    match c {
        "(" => Some(Token::LeftParenthesis),
        ")" => Some(Token::RightParenthesis),
        "{" => Some(Token::LeftBrace),
        "}" => Some(Token::RightBrace),
        "[" => Some(Token::LeftSquareBracket),
        "]" => Some(Token::RightSquareBracket),
        ";" => Some(Token::Semicolon),
        ":" => Some(Token::Colon),
        "=" => Some(Token::Equal),
        "+" => Some(Token::Plus),
        "-" => Some(Token::Minus),
        "*" => Some(Token::Multiply),
        "/" => Some(Token::Divide),
        "<" => Some(Token::LessThan),
        ">" => Some(Token::GreaterThan),
        "~" => Some(Token::Tilde),
        "," => Some(Token::Comma),
        "^" => Some(Token::Power),
        "==" => Some(Token::EqualEqual),
        "~=" => Some(Token::TildeEqual),
        _ => None
    }
}

pub fn is_symbol(c: char) -> bool {
    SYMBOLS.contains(c)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    StringLiteral(String),
    IntegerLiteral(i32),
    FloatLiteral(f64),
    LeftParenthesis, // (
    RightParenthesis, // )
    LeftBrace, // {
    RightBrace, // }
    LeftSquareBracket, // [
    RightSquareBracket, // ]
    Semicolon, // ;
    Colon, // :
    Comma, // ,
    Equal, // =
    Plus, // +
    Minus, // -
    Multiply, // *
    Divide, // /
    LessThan, // <
    GreaterThan, // >
    Tilde, // ~
    Power, // ^
    EqualEqual, // ==
    TildeEqual, // ~=
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Keyword(name) => write!(f, "keyword ({})", name),
            Token::Identifier(name) => write!(f, "identifier ({})", name),
            Token::StringLiteral(string_literal) => write!(f, "string literal ({})", string_literal),
            Token::IntegerLiteral(number) => write!(f, "integer literal ({})", number),
            Token::FloatLiteral(number) => write!(f, "float literal ({})", number),
            Token::RightParenthesis => write!(f, "right parenthesis"),
            Token::LeftParenthesis => write!(f, "left parenthesis"),
            Token::RightBrace => write!(f, "right brace"),
            Token::LeftBrace => write!(f, "left brace"),
            Token::RightSquareBracket => write!(f, "right square bracket"),
            Token::LeftSquareBracket => write!(f, "left square bracket"),
            Token::Semicolon => write!(f, "semicolon"),
            Token::Colon => write!(f, "colon"),
            Token::Comma => write!(f, "comma"),
            Token::Equal => write!(f, "equal"),
            Token::Plus => write!(f, "plus"),
            Token::Minus => write!(f, "minus"),
            Token::Multiply => write!(f, "multiply"),
            Token::Divide => write!(f, "divide"),
            Token::LessThan => write!(f, "less than"),
            Token::GreaterThan => write!(f, "greater than"),
            Token::Tilde => write!(f, "tilde"),
            Token::Power => write!(f, "power"),
            Token::EqualEqual => write!(f, "equal equal"),
            Token::TildeEqual => write!(f, "tilde equal")
        }
    }
}