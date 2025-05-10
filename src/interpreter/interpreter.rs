use std::rc::Rc;
use std::cell::RefCell;
use std::io;

use crate::parser::data_type::DataType;
use crate::parser::expression::Expression;
use crate::parser::function::Function;
use crate::parser::statement::Statement;

use super::enviroment::Environment;
use super::interpret_expression::InterpretExpression;
use super::interpret_statement::InterpretStatement;
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

    pub fn get_global(&self) -> Rc<RefCell<Environment>> {
        self.global.clone()
    }

    pub fn get_local(&self) -> Rc<RefCell<Environment>> {
        self.local.clone()
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
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Entry point not found!"))
        }?;

        Ok(())
    }

    pub fn register_rust_function(&mut self, name: &str, function: fn(args: Vec<Value>) -> io::Result<Value>) {
        let value = Value::RustFunction(function);

        self.global.borrow_mut().define_variable(name.to_string(), value);
    }

    pub fn call_function(&mut self, name: &str, args: Vec<Value>) -> io::Result<Value> {
        let function_value = self.global.borrow().get_variable(name)?;

        if let Value::RustFunction(function) = function_value {
            let return_value = function(args)?;

            return Ok(return_value);
        }

        let mut return_value = Value::Void;

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
                let arg_type = arg.get_data_type();
                
                if arg_type != parameter.data_type {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Expected argument type {}, instead got {}", parameter.data_type, arg_type)
                    ));
                }
                
                self.local.borrow_mut().define_variable(parameter.name.clone(), arg);
            }

            for statement in function.borrow().body.iter() {
                match self.interpret_statement(statement.clone()) {
                    Ok(Some(value)) => {
                        return_value = value;

                        break;
                    },
                    Err(e) => return Err(e),
                    _ => {}
                } 
            }

            let return_data_type = return_value.get_data_type();
            let expected_data_type = function.borrow().return_type.clone();

            if return_data_type != expected_data_type {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Expected to return {}, but instead got {}", return_data_type, expected_data_type)
                ));
            }

            self.local = previous_enviroment.clone();
        }
        
        Ok(return_value)
    }

    pub fn interpret_statement(&mut self, statement: Statement) -> io::Result<Option<Value>> {
        let mut interpret_statement = InterpretStatement::new(self);

        interpret_statement.interpret_statement(statement)
    }

    pub fn interpret_expression(&mut self, expression: Expression) -> io::Result<Value> {
        let mut interpret_expression = InterpretExpression::new(self);

        return interpret_expression.interpret_expression(expression);
    }
}
