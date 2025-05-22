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

    // Todo:
    // * addition/division/difference/multiplication and etc result datatype

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
    
    fn binary_operation(&self, operator: Operator, left: DataType, right: DataType) -> io::Result<DataType> {
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

    pub fn get_data_type(&self, expression: Expression) -> io::Result<DataType> {
        match expression {
            Expression::BinaryOp { left, operator, right } => {
                let left = self.get_data_type(*left)?;
                let right = self.get_data_type(*right)?;

                self.binary_operation(operator, left, right)
            }

            _ => {
                todo!()
            }
        }
    }

    pub fn save_variable(&mut self, name: &str, expression: Expression) {
        todo!()
    }
}