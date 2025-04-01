use crate::lexer::token::Token;
use std::io;

pub struct Parser {
    tokens: Vec<Token>,
    current_token: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current_token: 0
        }
    }

    pub fn parse(&mut self) -> io::Result<()> {
        
        
        Ok(())
    }
}