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

    pub fn parse_functions(&mut self) -> io::Result<Vec<Function>> {
        let mut functions: Vec<Function> = Vec::new();
        
        while !self.is_end() {
            let function = self.parse_function()?;
            functions.push(function);
        }

        Ok(functions)
    }

    pub fn parse_function(&mut self) -> io::Result<Function> {
        self.consume_keyword("function")?;

        let function_name = self.consume_identefier()?;

        self.consume_token(Token::LeftParenthesis)?;

        let parameters = self.parse_parameters()?;

        let function_type = if self.match_token(&Token::Colon) {
            self.consume_data_type()?
        } else {
            DataType::Void
        };

        self.consume_token(Token::LeftBrace)?;
        let block = self.parse_block_statement()?;

        Ok(
            Function {
                name: function_name,
                return_type: function_type,
                parameters: parameters,
                body: block
            }
        )
    }

    fn parse_parameters(&mut self) -> io::Result<Vec<Parameter>> {
        if self.match_token(&Token::RightParenthesis) {
            return Ok(Vec::new());
        }
        
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

    pub fn parse_block_statement(&mut self) -> io::Result<Vec<Statement>> {
        let mut statements: Vec<Statement> = Vec::new();
        
        while !(self.match_token(&Token::RightBrace) || self.is_end()) {
            let statement = self.parse_statement()?;
            
            statements.push(statement);
        }

        Ok(statements)
    }

    pub fn parse_statement(&mut self) -> io::Result<Statement> {
        if self.match_keyword("let") {
            self.parse_variable_declaration_statement()
        } else if self.match_keyword("return") {
            self.parse_return_statement()
        } else if self.match_keyword("if") {
            self.parse_if_statement()
        } else if self.match_keyword("while") {
            self.parse_while_statement()
        } else if let Token::Identifier(name) = self.peek().to_owned() { 
            self.advance();

            if self.match_token(&Token::Equal) {
                self.parse_assigment_statement(name)
            } else if self.match_token(&Token::LeftParenthesis) {
                self.parse_function_call_statement(name)
            } else {
                self.advance();
                
                Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown statement with identefier"))
            }
        } else {
            self.advance();

            Err(io::Error::new(io::ErrorKind::InvalidData, format!("Invalid statement {}", self.previous())))
        }
    }

    fn parse_variable_declaration_statement(&mut self) -> io::Result<Statement> {
        let name = self.consume_identefier()?;
        
        let data_type = if self.match_token(&Token::Colon) {
            Some(self.consume_data_type()?)
        } else {
            None
        };
        
        self.consume_token(Token::Equal)?;
        let expression = self.parse_expression()?;
        
        self.consume_token(Token::Semicolon)?;

        Ok(
            Statement::VarableDeclaration {
                name: name,
                data_type: data_type,
                value: Some(expression)
            }
        )
    }

    fn parse_return_statement(&mut self) -> io::Result<Statement> {
        let expression = self.parse_expression()?;
        self.consume_token(Token::Semicolon)?;
        
        Ok(
            Statement::ReturnStatement {
                value: Some(expression)
            }
        )
    }

    fn parse_assigment_statement(&mut self, name: String) -> io::Result<Statement> {
        let expression = self.parse_expression()?;
        self.consume_token(Token::Semicolon)?;

        Ok(Statement::Assigment { name: name, value: expression })
    }

    fn parse_function_call_statement(&mut self, name: String) -> io::Result<Statement> {
        let parameters = self.parse_function_call_parameters()?;

        self.consume_token(Token::RightParenthesis)?;
        self.consume_token(Token::Semicolon)?;
        
        Ok(Statement::FunctionCall { name: name, parameters: parameters })
    }

    fn parse_function_call_parameters(&mut self) -> io::Result<Vec<Expression>> {
        let mut parameters: Vec<Expression> = Vec::new();
        
        loop {
            let expression = self.parse_expression()?;
            parameters.push(expression);

            if !self.match_token(&Token::Comma) {
                break;
            }
        }

        Ok(parameters)
    }

    fn parse_if_statement(&mut self) -> io::Result<Statement> {
        let expression = self.parse_expression()?;

        self.consume_token(Token::LeftBrace)?;
        let if_body = self.parse_block_statement()?;

        let else_block= if self.match_keyword("else") {
            self.consume_token(Token::LeftBrace)?;

            Some(self.parse_block_statement()?)
        } else {
            None
        };

        Ok(
            Statement::IfStatement {
                condition: expression,
                body: if_body,
                else_body: else_block
            }
        )
    }

    fn parse_while_statement(&mut self) -> io::Result<Statement> {
        let expression = self.parse_expression()?;
        self.consume_token(Token::LeftBrace)?;

        let block = self.parse_block_statement()?;

        Ok(
            Statement::WhileStatement {
                condition: expression,
                body: block
            }
        )
    }

    pub fn parse_expression(&mut self) -> io::Result<Expression> {
        self.parse_logic_or()
    }

    fn parse_logic_or(&mut self) -> io::Result<Expression> {
        let mut expression = self.parse_logic_and()?;
        
        while self.match_token(&Token::PipePipe) {
            let right = self.parse_logic_and()?;

            expression = Expression::BinaryOp { left: Box::new(expression), operator: Operator::Or, right: Box::new(right) }
        }
        
        Ok(expression)
    }

    fn parse_logic_and(&mut self) -> io::Result<Expression> {
        let mut expression = self.parse_comparison()?;

        while self.match_token(&Token::AmpersandAmpersand) {
            let right = self.parse_comparison()?;

            expression = Expression::BinaryOp { left: Box::new(expression), operator: Operator::And, right: Box::new(right) }
        }

        Ok(expression)
    }

    fn parse_comparison(&mut self) -> io::Result<Expression> {
        let mut expression = self.parse_addition()?;

        while self.match_token(&Token::EqualEqual) || self.match_token(&Token::TildeEqual) {
            let operator = match self.previous() {
                Token::EqualEqual => Operator::EqualEqual,
                Token::TildeEqual => Operator::NotEqual,
                _ => unreachable!()
            };

            let right = self.parse_addition()?;

            expression = Expression::BinaryOp { left: Box::new(expression), operator: operator, right: Box::new(right) }
        }

        Ok(expression)
    }

    fn parse_addition(&mut self) -> io::Result<Expression> {
        let mut expression = self.parse_multiplication()?;
        
        while self.match_token(&Token::Plus) || self.match_token(&Token::Minus) {
            let operator = match self.previous() {
                Token::Plus => Operator::Plus,
                Token::Minus => Operator::Minus,
                _ => unreachable!()
            };

            let right = self.parse_multiplication()?;

            expression = Expression::BinaryOp { left: Box::new(expression), operator: operator, right: Box::new(right) };
        }

        Ok(expression)
    }

    fn parse_multiplication(&mut self) -> io::Result<Expression> {
        let mut expression = self.parse_power()?;

        while self.match_token(&Token::Multiply) || self.match_token(&Token::Divide) {
            let operator = match self.previous() {
                Token::Multiply => Operator::Multiply,
                Token::Divide => Operator::Divide,
                _ => unreachable!()
            };

            let right = self.parse_power()?;

            expression = Expression::BinaryOp { left: Box::new(expression), operator: operator, right: Box::new(right) };
        }

        Ok(expression)
    }

    fn parse_power(&mut self) -> io::Result<Expression> {
        let mut expression = self.parse_unary()?;

        while self.match_token(&Token::Power) {
            let right = self.parse_unary()?;

            expression = Expression::BinaryOp { left: Box::new(expression), operator: Operator::Power, right: Box::new(right) }
        }

        Ok(expression)
    }

    fn parse_unary(&mut self) -> io::Result<Expression> {
        if self.match_token(&Token::Minus) || self.match_token(&Token::Tilde) {
            let operator = match self.previous() {
                Token::Minus => Operator::Minus,
                Token::Tilde => Operator::Tilde,
                _ => unreachable!()
            };
            
            let expression = self.parse_primary()?;
            
            Ok(
                Expression::UnaryOp {
                    expression: Box::new(expression),
                    operator: operator
                }
            )
        } else {
            self.parse_primary()
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
                self.advance();
                if self.match_token(&Token::LeftParenthesis) {
                    let parameters = self.parse_function_call_parameters()?;
                    self.consume_token(Token::RightParenthesis)?;

                    Ok(Expression::FunctionCall(name, parameters))
                } else {
                    let expression = Expression::Identifier(name);
                    
                    Ok(expression)
                }
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Expected expression")) 
        }
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_end() {
            return false;
        }
        
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
        if self.is_end() {
            return None;
        }

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

    fn next(&self) -> Option<&Token> {
        if self.current_token + 1 >= self.tokens.len() {
            None
        } else {
            Some(&self.tokens[self.current_token + 1])
        }

    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current_token - 1]
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current_token]
    }
}