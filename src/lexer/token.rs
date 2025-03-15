use std::fmt;

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
        _ => false
        
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    StringLiteral(String),
    IntegerLiteral(i32),
    FloatLiteral(f32),
    RightParenthesis,
    LeftParenthesis,
    RightBrace,
    LeftBrace,
    Semicolon,
    Colon,
    Equal,
    Plus,
    Minus,
    Multiply,
    Divide
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
            Token::Semicolon => write!(f, "semicolon"),
            Token::Colon => write!(f, "colon"),
            Token::Equal => write!(f, "equal"),
            Token::Plus => write!(f, "plus"),
            Token::Minus => write!(f, "minus"),
            Token::Multiply => write!(f, "multiply"),
            Token::Divide => write!(f, "divide")
        }
    }
}

pub struct TokenLine {
    token_line: Vec<Token>
}

impl TokenLine {
    pub fn new(token_line: Vec<Token>) -> TokenLine {
        TokenLine {
            token_line: token_line
        }
    }

    pub fn show(&self) {
        // for token in self.token_line.iter() {
        //     println!("{} -> {}", token.value(), token.token_type());
        // }
    }
}