use std::fs::read_to_string;
use std::io;


use super::token::{Token, TokenLine};
use super::token::{is_keyword};

use super::lexer_state::LexerState;


pub struct Lexer {
    lines: Vec<TokenLine>,
    indexes: Vec<i32>,
    source_path: String,
    source_lines: Vec<String>,
    source: String,
    current_line: i32,
    output: Vec<String>
}


impl Lexer {
    pub fn new(source_path: &str) -> io::Result<Lexer> {
        let source: String = read_to_string(source_path)?;
        let source_lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();
        let lines: Vec<TokenLine> = Vec::new();
        let indexes: Vec<i32> = Vec::new();
        let output: Vec<String> = Vec::new();
        let current_line: i32 = 0;

        Ok(Lexer {
            lines,
            indexes,
            source_path: source_path.to_string(),
            source_lines,
            source,
            current_line,
            output
        })
    }

    fn add_token(&mut self, buffer: &str, token: &mut Vec<Token>) {
        if is_keyword(buffer) {
            token.push(Token::Keyword(buffer.to_string()));
        } else {
            token.push(Token::Identifier(buffer.to_string()));
        }
    }

    fn handle_symbol(&mut self, c: char, tokens: &mut Vec<Token>) {
        match c {
            '(' => tokens.push(Token::LeftParenthesis),
            ')' => tokens.push(Token::RightParenthesis),
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            ';' => tokens.push(Token::Semicolon),
            ':' => tokens.push(Token::Colon),
            '=' => tokens.push(Token::Equal),
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Multiply),
            '/' => tokens.push(Token::Divide),
            _ => {}
        }
    }

    pub fn read_line(&mut self, line: &str) -> io::Result<Vec<Token>> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut cur: usize = 0;
        let mut state = LexerState::None;

        let mut buffer = String::new();

        while cur < line.len() {
            let current_char = line.chars().nth(cur).unwrap();
            
            match state {
                LexerState::None => {
                    if current_char.is_whitespace() {
                        if buffer.len() > 0 {
                            self.add_token(&buffer, &mut tokens);
                            buffer.clear();
                        }
                    } else if current_char.is_alphabetic() {
                        state = LexerState::Identifier;
                        buffer.push(current_char);
                    } else if current_char.is_numeric() {
                        state = LexerState::Number;
                        buffer.push(current_char);
                    } else if current_char == '"' {
                        state = LexerState::String;
                    } else {
                        self.handle_symbol(current_char, &mut tokens);
                    }
                }

                LexerState::String => {
                    if current_char == '"' {
                        tokens.push(Token::StringLiteral(buffer.clone()));
                        buffer.clear();
                        state = LexerState::None;
                    } else {
                        buffer.push(current_char);
                    }
                }

                LexerState::Number => {
                    if current_char.is_numeric() || current_char == '.' {
                        buffer.push(current_char);
                    } else if current_char == 'f' {
                        if let Ok(num) = buffer.parse::<f32>() {
                            tokens.push(Token::FloatLiteral(num));
                            buffer.clear();
                            state = LexerState::None;
                        } else {
                            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid float literal"));
                        }
                    } else {
                        if let Ok(num) = buffer.parse::<i32>() {
                            tokens.push(Token::IntegerLiteral(num));
                            buffer.clear();
                            state = LexerState::None;
                        } else {
                            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid integer literal"));
                        }
                    }
                }

                LexerState::Identifier => {
                    if current_char.is_alphabetic() || current_char.is_numeric() {
                        buffer.push(current_char);
                    } else {
                        self.add_token(buffer.as_str(), &mut tokens);
                        buffer.clear();
                        state = LexerState::None;

                        continue;
                    }
                }
            }

            cur += 1;
        }  
        
        Ok(tokens)
    }
    
    pub fn lexer(&self) -> io::Result<()> {
        todo!()
    }
}

