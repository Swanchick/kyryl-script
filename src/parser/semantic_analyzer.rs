use std::io;
use std::collections::HashMap;

use crate::parser::expression;
use crate::parser::operator::Operator;

use super::data_type::DataType;
use super::expression::Expression;
use super::operator;

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
        todo!()
    }
    
    fn binary_operation(&self, operator: Operator, left: DataType, right: DataType) -> io::Result<DataType> {
        match operator {
            Operator::Plus => {
                self.plus(left, right)
            }

            Operator::Minus => {
                todo!()
            }

            Operator::Multiply => {
                todo!()
            }

            Operator::Divide => {
                todo!()
            }

            Operator::And => {
                todo!()
            }

            Operator::Or => {
                todo!()
            }

            Operator::EqualEqual => {
                todo!()
            }

            Operator::NotEqual => {
                todo!()
            }

            Operator::GreaterEqual => {
                todo!()
            }

            Operator::Greater => {
                todo!()
            }

            Operator::LessEqual => {
                todo!()
            }

            Operator::Less => {
                todo!()
            }

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