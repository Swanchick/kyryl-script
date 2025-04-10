use std::rc::Rc;
use std::cell::RefCell;
use std::io;

use crate::parser::data_type::DataType;
use crate::parser::function::Function;
use crate::parser::parameter::Parameter;
use crate::parser::statement::{self, Statement};

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
}