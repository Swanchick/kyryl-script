use std::io;
use std::collections::HashMap;

use crate::parser::operator::Operator;

use super::data_type::DataType;
use super::expression::Expression;


pub struct SemanticAnalyzer {
    variables: HashMap<String, DataType>
}


impl SemanticAnalyzer {
    pub fn init() -> SemanticAnalyzer {
        SemanticAnalyzer { variables: HashMap::new() }
    }

    fn plus(&self, left: DataType, right: DataType) -> io::Result<DataType> {
        match (left, right) {
            (DataType::Int, DataType::Int) => Ok(DataType::Int),
            (DataType::Float, DataType::Int) => Ok(DataType::Float),
            (DataType::Int, DataType::Float) => Ok(DataType::Float),
            (DataType::Float, DataType::Float) => Ok(DataType::Float),
            (DataType::String, DataType::String) => Ok(DataType::String),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Arithmetic type error!"))
        }
    }

    fn arithmetic(&self, left: DataType, right: DataType) -> io::Result<DataType> {
        match (left, right) {
            (DataType::Int, DataType::Int) => Ok(DataType::Int),
            (DataType::Float, DataType::Int) => Ok(DataType::Float),
            (DataType::Int, DataType::Float) => Ok(DataType::Float),
            (DataType::Float, DataType::Float) => Ok(DataType::Float),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Arithmetic type error!"))
        }
    }

    fn boolean(&self, left: DataType, right: DataType) -> io::Result<DataType> {
        match (left, right) {
            (DataType::Bool, DataType::Bool) => Ok(DataType::Bool),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Logic type error!"))
        }
    }

    fn comparison(&self, left: DataType, right: DataType) -> io::Result<DataType> {
        match (left, right) {
            (DataType::Int, DataType::Int) 
            | (DataType::Float, DataType::Int)
            | (DataType::Int, DataType::Float)
            | (DataType::Float, DataType::Float)
            | (DataType::String, DataType::String) => Ok(DataType::Bool),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Arithmetic type error!"))
        }
    }

    fn logic(&self, left: DataType, right: DataType) -> io::Result<DataType> {
        match (left, right) {
            (DataType::Int, DataType::Int) 
            | (DataType::Float, DataType::Int)
            | (DataType::Int, DataType::Float)
            | (DataType::Float, DataType::Float) => Ok(DataType::Bool),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Arithmetic type error!"))
        }
    }
    
    fn binary_operation(&self, operator: &Operator, left: DataType, right: DataType) -> io::Result<DataType> {
        match operator {
            Operator::Plus => self.plus(left, right),
            Operator::Minus => self.arithmetic(left, right),
            Operator::Multiply => self.arithmetic(left, right),
            Operator::Divide => self.arithmetic(left, right),
            Operator::And => self.boolean(left, right),
            Operator::Or => self.boolean(left, right),
            Operator::EqualEqual => self.comparison(left, right),
            Operator::NotEqual => self.comparison(left, right),
            Operator::GreaterEqual => self.logic(left, right),
            Operator::Greater => self.logic(left, right),
            Operator::LessEqual => self.logic(left, right),
            Operator::Less => self.logic(left, right),
            _ => unreachable!()
        }
    }

    fn unary_operation(&self, operator: &Operator, right: DataType) -> io::Result<DataType> {
        match (operator, right) {
            (Operator::Minus, DataType::Int) => Ok(DataType::Int),
            (Operator::Minus, DataType::Float) => Ok(DataType::Float),
            (Operator::Tilde, DataType::Bool) => Ok(DataType::Bool),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid operator in unary operation!"))
        }
    }

    fn front_unary_operation(&self, operator: &Operator, left: DataType) -> io::Result<DataType> {
        match (operator, left) {
            (Operator::PlusPlus, DataType::Int) => Ok(DataType::Int),
            (Operator::PlusPlus, DataType::Float) => Ok(DataType::Float),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid operator in front unary operation!"))
        }
    }

    fn identefier_index(&self, left: DataType, index: DataType) -> io::Result<DataType> {
        match (left, index) {
            (DataType::List(children_type), DataType::Int) => Ok(*children_type),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid data in list indexing operation!"))
        }
    }

    pub fn get_data_type(&self, expression: &Expression) -> io::Result<DataType> {
        match expression {
            Expression::BinaryOp { left, operator, right } => {
                let left = self.get_data_type(left)?;
                let right = self.get_data_type(right)?;

                self.binary_operation(operator, left, right)
            },

            Expression::UnaryOp { expression, operator } => {
                let right = self.get_data_type(expression)?;
                self.unary_operation(operator, right)
            },

            Expression::FrontUnaryOp { expression, operator } => {
                let left = self.get_data_type(expression)?;
                self.front_unary_operation(operator, left)
            },

            Expression::ListLiteral(children) => {
                if children.len() == 0 {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "List empty!"));
                }

                let first = self.get_data_type(&children[0].clone())?;

                for child in children.iter() {
                    let child = self.get_data_type(&child.clone())?;

                    if first == child {
                        continue;
                    }

                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Children has different types in list!"));
                }

                Ok(DataType::List(Box::new(first)))
            },
            
            Expression::Identifier(name) => {
                match self.variables.get(name) {
                    Some(DataType::Void) => Ok(DataType::Void),
                    Some(data_type) => Ok(data_type.clone()),
                    None => Err(io::Error::new(io::ErrorKind::InvalidData, format!("Variable {} not found!", name)))
                }
            },

            Expression::FunctionCall(name, parameters) => {
                todo!()
            },

            Expression::IdentifierIndex { left, index } => {
                let left = self.get_data_type(left)?;
                let index = self.get_data_type(index)?;
                
                self.identefier_index(left, index)
            },

            Expression::IntegerLiteral(_) => Ok(DataType::Int),
            Expression::FloatLiteral(_) => Ok(DataType::Float),
            Expression::StringLiteral(_) => Ok(DataType::String),
            Expression::BooleanLiteral(_) => Ok(DataType::Bool)
        }
    }

    pub fn save_variable(&mut self, name: String, expression: DataType) {
        self.variables.insert(name, expression);
    }
}