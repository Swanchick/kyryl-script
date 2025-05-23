use std::fs::read_to_string;
use std::io;

use super::token::Token;
use super::token::COMMENT;
use super::token::{is_keyword, get_symbol, is_symbol};

use super::lexer_state::LexerState;


pub struct Lexer {
    tokens: Vec<Token>,
    source_lines: Vec<String>,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        let source_lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();
        
        Lexer {
            tokens: Vec::new(),
            source_lines: source_lines,
        }
    }
    
    pub fn load(source_path: &str) -> io::Result<Lexer> {
        let source: String = read_to_string(source_path)?;
        let source_lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();

        Ok(Lexer {
            tokens: Vec::new(),
            source_lines,
        })
    }

    fn add_token(&mut self, buffer: &str, token: &mut Vec<Token>) {
        if is_keyword(buffer) {
            token.push(Token::Keyword(buffer.to_string()));
        } else {
            token.push(Token::Identifier(buffer.to_string()));
        }
    }

    pub fn lex_line(&mut self, line: &str) -> io::Result<Vec<Token>> { 
        // Todo:
        // Remove this thing
        // I mean, lexer won't work without it, but it could be done better
        let mut line = line.to_string();
        line.push(' ');

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
                    } else if is_symbol(current_char) {
                        state = LexerState::Symbol;
                        buffer.push(current_char);
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
                        if let Ok(num) = buffer.parse::<f64>() {
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

                            continue;
                        } else {
                            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid integer literal"));
                        }
                    }
                }

                LexerState::Identifier => {
                    if current_char.is_alphabetic() || current_char.is_numeric() || current_char == '_' {
                        buffer.push(current_char);
                    } else {
                        self.add_token(buffer.as_str(), &mut tokens);
                        buffer.clear();
                        state = LexerState::None;

                        continue;
                    }
                }

                LexerState::Symbol => {
                    if is_symbol(current_char) && cur != line.len() {
                        buffer.push(current_char);
                    } else {
                        // Checking if the symbol is a comment and if it is indeed then break the loop to proceed to the next line
                        // easy 
                        if buffer.contains(COMMENT) {
                            break;
                        }

                        let mut symbols = self.get_symbols(&buffer);

                        tokens.append(&mut symbols);
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

    fn get_symbols(&self, buffer: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = buffer.chars().collect();
        let mut i = 0;
    
        while i < chars.len() {
            let mut matched = false;

            for j in (i + 1..=chars.len()).rev() {
                let slice: String = chars[i..j].iter().collect();
    
                if let Some(token) = get_symbol(&slice) {
                    tokens.push(token);
                    i = j;
                    matched = true;
                    break;
                }
            }

            if !matched {
                let single = chars[i].to_string();
                if let Some(token) = get_symbol(&single) {
                    tokens.push(token);
                }
    
                i += 1;
            }
        }
    
        tokens
    }

    pub fn lexer(&mut self) -> io::Result<()> {
        for line in self.source_lines.clone() {
            let tokens = self.lex_line(&line)?;
            self.tokens.extend(tokens);
        }

        Ok(())
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}
