use crate::lexer::token::Token;

use super::operator::Operator;
use super::data_type::DataType;
use super::expression::Expression;
use super::function::Function;
use super::parameter::Parameter;
use super::statement::Statement;

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

    pub fn parse_function(&mut self) -> io::Result<Function> {
        self.consume_keyword("function")?;
        println!("{}", self.current_token);

        let function_name = self.consume_identefier()?;
        println!("{}", self.current_token);

        println!("Curearear a: {}", self.tokens[self.current_token]);
        self.consume_token(Token::LeftParenthesis)?;
        println!("{}", self.current_token);
        let parameters = self.parse_parameters()?;

        let mut function_type = DataType::Void;

        if self.match_token(&Token::Colon) {
            function_type = self.consume_data_type()?;
        }

        Ok(
            Function {
                name: function_name,
                return_type: function_type,
                parameters: parameters,
                body: Vec::new()
            }
        )
    }

    fn parse_parameters(&mut self) -> io::Result<Vec<Parameter>> {
        let mut parameters: Vec<Parameter> = Vec::new();

        loop {
            let parameter = self.parse_parameter()?;
            parameters.push(parameter);

            if self.match_token(&Token::RightParenthesis) {
                break;
            } else {
                self.consume_token(Token::Comma)?;
            }
        }

        Ok(parameters)
    }

    fn parse_parameter(&mut self) -> io::Result<Parameter> { 
        let name = self.consume_identefier()?;
        self.consume_token(Token::Colon)?;
        let data_type = self.consume_data_type()?;

        let parameter = Parameter {
            name: name,
            data_type: data_type
        };

        Ok(parameter)
    }

    pub fn parse_statement(&mut self) -> io::Result<Statement> {
        if self.match_keyword("let") {
            self.parse_variable_declaration_statement()
        } else if self.match_keyword("return") {
            self.parse_return_statement()
        } else {
            todo!()
        }
    }

    fn parse_variable_declaration_statement(&mut self) -> io::Result<Statement> {
        let name = self.consume_identefier()?;
        
        self.consume_token(Token::Colon)?;
        
        let data_type = self.consume_data_type()?;
        self.consume_token(Token::Equal)?;
        let expression = self.parse_expression()?;
        
        self.consume_token(Token::Semicolon)?;

        Ok(
            Statement::VarableDeclaration {
                name: name,
                data_type: Some(data_type),
                value: Some(expression)
            }
        )
    }

    fn parse_return_statement(&mut self) -> io::Result<Statement> {
        let expression = self.parse_expression()?;
        
        Ok(
            Statement::ReturnStatement {
                value: Some(expression)
            }
        )
    }

    pub fn parse_expression(&mut self) -> io::Result<Expression> {
        self.parse_addition()
    }

    fn parse_addition(&mut self) -> io::Result<Expression> {
        let mut expression = self.parse_multipilcation()?;
        
        while self.match_token(&Token::Plus) || self.match_token(&Token::Minus) {
            let operator = match self.previous() {
                Token::Plus => Operator::Plus,
                Token::Minus => Operator::Minus,
                _ => unreachable!()
            };

            let right = self.parse_multipilcation()?;

            expression = Expression::BinaryOp { left: Box::new(expression), operator: operator, right: Box::new(right) };
        }

        Ok(expression)
    }

    fn parse_multipilcation(&mut self) -> io::Result<Expression> {
        let mut expression = self.parse_unary()?;

        while self.match_token(&Token::Multiply) || self.match_token(&Token::Divide) {
            let operator = match self.previous() {
                Token::Multiply => Operator::Multiply,
                Token::Divide => Operator::Divide,
                _ => unreachable!()
            };

            let right = self.parse_unary()?;

            expression = Expression::BinaryOp { left: Box::new(expression), operator: operator, right: Box::new(right) };
        }

        Ok(expression)
    }

    fn parse_unary(&mut self) -> io::Result<Expression> {
        if self.match_token(&Token::Minus) {
            let expression = self.parse_primary()?;
            
            Ok(
                Expression::UnaryOp {
                    expression: Box::new(expression),
                    operator: Operator::Minus
                }
            )
        } else {
            let expression = self.parse_primary();

            expression
        }
    }

    fn parse_primary(&mut self) -> io::Result<Expression> {
        match self.peek() {
            Token::LeftParenthesis => {
                self.advance();
                let expression = self.parse_expression()?;
                
                if !self.match_token(&Token::RightParenthesis) {
                    Err(io::Error::new(io::ErrorKind::InvalidData, "Expected closed expression with right parenthesis."))
                } else {
                    Ok(expression)
                }
            },
            Token::IntegerLiteral(value) => {
                let value = value.to_owned();
                let expression = Expression::IntegerLiteral(value);
                self.advance();
                
                Ok(expression)
            },
            Token::FloatLiteral(value) => {
                let value = value.to_owned();
                let expression = Expression::FloatLiteral(value);
                self.advance();

                Ok(expression)
            },
            Token::StringLiteral(value) => {
                let value = value.to_owned();
                let expression = Expression::StringLiteral(value);
                self.advance();

                Ok(expression)
            },
            Token::Keyword(keyword) => {
                let keyword = keyword.as_str();
                
                match keyword {
                    "true" => {
                        self.advance();
                        Ok(Expression::BooleanLiteral(true))
                    },
                    "false" => {
                        self.advance();
                        Ok(Expression::BooleanLiteral(true))
                    },
                    _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unexpected keyword, expected boolean"))
                }
            },
            Token::Identifier(name) => {
                let name = name.to_owned();
                let expression = Expression::Identifier(name);
                self.advance();
                
                Ok(expression)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Expected expression")) 
        }
    }

    fn check(&self, token: &Token) -> bool {
        self.peek() == token
    }

    fn consume_data_type(&mut self) -> io::Result<DataType> {
        let token = self.advance().unwrap();
        
        if let Token::Keyword(data_type_str) = token {
            let data_type_str = data_type_str.as_str();

            match data_type_str {
                "int" => Ok(DataType::Int),
                "float" => Ok(DataType::Float),
                "string" => Ok(DataType::String),
                "bool" => Ok(DataType::Bool),
                _ => Err(io::Error::new(io::ErrorKind::InvalidData, format!("Expected type got {}", data_type_str)))
            }
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Expected keyword!"))
        }
    }

    fn consume_token(&mut self, token: Token) -> io::Result<Token> {
        if self.check(&token) {
            Ok(self.advance().unwrap())
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, format!("Expected token: {:?} got {:?}", token, self.peek())))
        }

    }

    fn consume_identefier(&mut self) -> io::Result<String> {
        let token= self.peek().clone();

        if let Token::Identifier(name) = token {
            self.advance();
            
            Ok(name.to_string())
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Expected token identefier!"))
        }
    }

    fn consume_keyword(&mut self, keyword: &str) -> io::Result<()> {
        if self.match_keyword(keyword) {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, format!("Expected keyword: {}", keyword)))
        }
    }

    fn advance(&mut self) -> Option<Token> {
        if self.current_token < self.tokens.len() {
            let token = self.peek().clone();
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

        if self.check(token) {
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

        if let Token::Keyword(ref k) = self.peek() {
            if k == keyword {
                self.advance();
                
                return true;
            }
        }

        false
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current_token - 1]
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current_token]
    }
}