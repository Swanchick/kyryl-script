use std::rc::Rc;
use std::cell::RefCell;
use std::io;

use crate::lexer::token::Token;
use crate::parser::expression::Expression;
use crate::parser::function::Function;
use crate::parser::operator::Operator;
use crate::parser::statement::Statement;

use super::enviroment::Environment;
use super::value::Value;

pub struct Interpreter {
    global: Rc<RefCell<Environment>>,
    local: Rc<RefCell<Environment>>
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let global = Rc::new(RefCell::new(Environment::new()));

        Interpreter {
            global: global.clone(),
            local: global
        }
    }

    pub fn interpret_program(&mut self, functions: Vec<Function>) -> io::Result<()> {
        for function in &functions {
            self.global.borrow_mut().define_variable(
                function.name.clone(),
                Value::Function(Rc::new(RefCell::new(function.clone())))
            );
        }
        
        let main = self.global.borrow().get_variable("main");

        match main {
            Ok(Value::Function(_)) => self.call_function("main", Vec::new()),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Entry point not fond!"))
        }
    }

    fn call_function(&mut self, name: &str, args: Vec<Value>) -> io::Result<()> {
        let function_value = self.global.borrow().get_variable(name)?;

        if let Value::Function(function) = function_value {
            let previous_enviroment = self.local.clone();
            self.local = Rc::new(RefCell::new(Environment::with_parent(self.global.clone())));

            if function.borrow().parameters.len() != args.len() {
                return Err(
                    io::Error::new(
                        io::ErrorKind::InvalidInput, 
                        format!("Expected {} amount of parameters, got {}", function.borrow().parameters.len(), args.len()))
                );
            }

            for (parameter, arg) in function.borrow().parameters.iter().zip(args) {
                self.local.borrow_mut().define_variable(parameter.name.clone(), arg);
            }

            let mut return_value = Value::Void;

            for statement in function.borrow().body.iter() {
                match self.interpret_statement(statement.clone()) {
                    Ok(Some(statement)) => {
                        
                    }

                    Err(e) => return Err(e),
                    _ => {}
                } 
            }
        }
        
        Ok(())
    }

    fn interpret_statement(&self, statement: Statement) -> io::Result<Option<Value>> {
        todo!()
    }

    pub fn interpret_expression(&self, expression: Expression) -> io::Result<Value> {
        match expression {
            Expression::BinaryOp { left, operator, right } => {
                let left_value = self.interpret_expression(*left)?;
                let right_value  = self.interpret_expression(*right)?;


                self.interpret_binary_operation(left_value, right_value, operator)
            },
            Expression::IntegerLiteral(value) => {
                Ok(Value::Integer(value.clone()))
            },
            Expression::FloatLiteral(value) => {
                Ok(Value::Float(value.clone()))
            },
            Expression::BooleanLiteral(value) => {
                Ok(Value::Boolean(value.clone()))
            },
            Expression::StringLiteral(value) => {
                Ok(Value::String(value.clone()))
            },
            _ => {
                Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown data in expression"))
            }
        }
    }

    fn interpret_binary_operation(&self, left: Value, right: Value, operator: Operator) -> io::Result<Value> {
        match operator {
            Operator::Plus => {
                self.interpret_plus(left, right)
            },
            Operator::Minus => {
                self.interpret_minus(left, right)
            },
            Operator::Multiply => {
                todo!()
            },
            Operator::Divide => {
                todo!()
            },
            _ => {
                Err(io::Error::new(io::ErrorKind::InvalidData, "Unsupported operator!"))
            }
        }
    }

    fn interpret_plus(&self, left: Value, right: Value) -> io::Result<Value> {
        match (left, right) {
            (Value::Integer(n1), Value::Integer(n2)) => {
                let value = Value::Integer(n1 + n2);

                Ok(value)
            },
            (Value::Float(n1), Value::Float(n2)) => {
                let value = Value::Float(n1 + n2);

                Ok(value)
            },
            (Value::Float(n1), Value::Integer(n2)) => {
                let value = Value::Float(n1 + (n2 as f64));

                Ok(value)
            },
            (Value::Integer(n1), Value::Float(n2)) => {
                let value = Value::Float((n1 as f64) + n2);

                Ok(value)
            },
            (Value::String(mut str1), Value::String(str2)) => {
                str1.push_str(&str2);
                let value = Value::String(str1);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    } 

    fn interpret_minus(&self, left: Value, right: Value) -> io::Result<Value> {
        match (left, right) {
            (Value::Integer(n1), Value::Integer(n2)) => {
                let value = Value::Integer(n1 - n2);

                Ok(value)
            },
            (Value::Float(n1), Value::Float(n2)) => {
                let value = Value::Float(n1 - n2);

                Ok(value)
            },
            (Value::Float(n1), Value::Integer(n2)) => {
                let value = Value::Float(n1 - (n2 as f64));

                Ok(value)
            },
            (Value::Integer(n1), Value::Float(n2)) => {
                let value = Value::Float((n1 as f64) - n2);

                Ok(value)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }
    } 
}