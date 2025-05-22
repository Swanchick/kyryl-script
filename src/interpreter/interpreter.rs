use std::rc::Rc;
use std::cell::RefCell;
use std::io;

use crate::parser::expression::Expression;
use crate::parser::statement::Statement;

use super::enviroment::Environment;
use super::interpret_expression::InterpretExpression;
use super::interpret_statement::InterpretStatement;
use super::return_value::Return;
use super::value::{Value, ValueType};

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

    pub fn get_variable(&self, name: &str) -> io::Result<Value> {
        let local = self.local.borrow();
        
        let value = local.get_variable(name)?;

        Ok(value.clone())
    }

    pub fn define_variable(&mut self, name: &str, value: Value) -> io::Result<()> {
        let mut local = self.local.borrow_mut();

        local.define_variable(name.to_string(), value)?;
        
        Ok(())
    }

    pub fn define_variable_by_reference(&mut self, name: &str, value: &Value) -> io::Result<()> {
        let mut local = self.local.borrow_mut();

        if let Some(reference) = value.get_reference() {
            local.create_value_reference(name.to_string(), reference);
        }
        
        Ok(())
    }

    pub fn assign_variable(&mut self, name: &str, value: Value) -> io::Result<()> {
        let mut local = self.local.borrow_mut();

        local.assign_variable(name, value)?;
        
        Ok(())
    }

    pub fn interpret_statements(&mut self, statements: Vec<Statement>) -> io::Result<Return> {
        for statement in statements {
            let result = self.interpret_statement(statement)?;

            if let Return::Success(_) = &result {
                return Ok(result);
            }
        }

        Ok(Return::Nothing)
    }

    pub fn interpret_statement(&mut self, statement: Statement) -> io::Result<Return> {
        let mut interpret_statement = InterpretStatement::new(self);

        interpret_statement.interpret_statement(statement)
    }

    pub fn interpret_expression(&mut self, expression: Expression) -> io::Result<Value> {
        let mut interpret_expression = InterpretExpression::new(self);

        interpret_expression.interpret_expression(expression)
    }

    pub fn register_rust_function(&mut self, name: &str, function: fn(args: Vec<Value>) -> io::Result<Value>) {
        let value_type = ValueType::RustFunction(function);
        let value = Value::new(None, value_type);

        let _ = self.global.borrow_mut().define_variable(name.to_string(), value);
    }

    pub fn call_function(&mut self, name: &str, args: Vec<Value>) -> io::Result<Value> {        
        let value = self.get_variable(name)?;

        match value.get_type() {
            ValueType::RustFunction(function) => {
                function(args)
            },

            ValueType::Function { name, return_type: _, parameters, body } => {
                if args.len() != parameters.len() {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Missmatch in function's singature \"{}\"!", name)));
                }

                let previous = self.local.clone();
                self.local = Rc::new(RefCell::new(Environment::with_parent(previous.clone())));

                for (arg, parameter) in args.iter().zip(parameters) {
                    if arg.get_type().get_data_type() != parameter.data_type {
                        return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Missmatch in function's singature \"{}\"!", name)));
                    }

                    if let Some(_) = arg.get_reference() {
                        self.define_variable_by_reference(parameter.name.as_str(), arg)?;
                    } else {
                        self.define_variable(parameter.name.as_str(), arg.clone())?;
                    }
                }

                let result = self.interpret_statements(body.to_vec())?;

                self.local = previous.clone();

                match result {
                    Return::Success(value) => Ok(value),
                    Return::Nothing => Ok(Value::new(None, ValueType::Null))
                }
            }

            _ => Err(io::Error::new(io::ErrorKind::InvalidData, format!("Variable {} is not a function!", name)))
        }
    }
}
