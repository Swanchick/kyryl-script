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

    pub fn parse_function(&mut self) -> io::Result<()> {
        self.consume_keyword("function")?;
        self.consume_token(Token::LeftBrace)?;


        Ok(())
    }

    pub fn parse_parameter(&mut self) -> io::Result<()> {


        Ok(())
    }

    fn check(&self, token: &Token) -> bool {
        self.peek() == token
    }

    fn consume_token(&mut self, token: Token) -> io::Result<Token> {
        if self.check(&token) {
            Ok(self.advance().unwrap())
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, format!("Expected token: {:?} got {:?}", token, self.peek())))
        }

    }

    fn consume_keyword(&mut self, keyword: &str) -> io::Result<()> {
        if (self.match_keyword(keyword)) {
            return Ok(());
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Expected keyword: {}", keyword)));
        }

        Ok(())
    }

    fn advance(&mut self) -> Option<Token> {
        if self.current_token < self.tokens.len() {
            let token = self.tokens[self.current_token].clone();
            self.current_token += 1;
            Some(token)
        } else {
            None
        }
    }

    fn is_end(&self) -> bool {
        self.current_token >= self.tokens.len()
    }

    fn current_token(&self) -> Option<&Token> {
        if self.current_token < self.tokens.len() {
            Some(&self.tokens[self.current_token])
        } else {
            None
        }
    }

    fn match_token(&mut self, token: &Token) -> bool {
        if self.is_end() {
            return false;
        }

        if &self.tokens[self.current_token] == token {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_keyword(&mut self, keyword: &str) -> bool {
        if self.is_end() {
            return false;
        }

        if let Token::Keyword(ref k) = self.tokens[self.current_token] {
            if k == keyword {
                self.advance();
                return true;
            }
        }

        false
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current_token]
    }
}