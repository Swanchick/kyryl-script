use std::rc::Rc;
use std::cell::RefCell;
use std::io;

use crate::native_registry::native_registry::NativeRegistry;
use crate::native_registry::native_types::NativeTypes;
use crate::parser::data_type::DataType;
use crate::parser::expression::Expression;
use crate::parser::statement::Statement;

use super::enviroment::Environment;
use super::interpret_expression::InterpretExpression;
use super::interpret_statement::InterpretStatement;
use super::return_value::Return;
use super::value::{Value, ValueType};

#[derive(Debug)]
pub struct Interpreter {
    global: Rc<RefCell<Environment>>,
    local: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new(global: Rc<RefCell<Environment>>) -> Interpreter {
        let local = Rc::new(RefCell::new(Environment::with_parent(global.clone())));
        
        let native = NativeRegistry::get();
        {
            let mut native = native.borrow_mut();
            if let None = native.global {
                native.global = Some(global.clone());
            }

            native.local = Some(local.clone());
        }

        Interpreter {
            global: global.clone(),
            local: local,
        }
    }

    pub fn empty() -> Interpreter {
        let local = Rc::new(RefCell::new(Environment::new()));
        
        Interpreter {
            global: local.clone(),
            local: local,
        }
    }

    pub fn create_reference(&mut self, reference: u64) {
        let mut local = self.local.borrow_mut();
        local.create_reference(reference);
    }

    pub fn create_value(&mut self, value: Value) -> u64 {
        let mut local = self.local.borrow_mut();
        local.create_value_without_name(value)
    }

    pub fn get_variable(&self, name: &str) -> io::Result<Value> {
        let local = self.local.borrow();
        local.get_variable(name)
    }

    pub fn get_variable_reference(&self, reference: u64) -> io::Result<Value> {
        let local = self.local.borrow();

        local.get_by_reference(reference)
    }

    pub fn define_variable(&mut self, name: &str, value: Value) -> io::Result<()> {
        let mut local = self.local.borrow_mut();

        local.define_variable(name.to_string(), value)?;
        
        Ok(())
    }

    pub fn define_variable_by_reference(&mut self, name: &str, reference: u64) -> io::Result<()> {
        let mut local = self.local.borrow_mut();

        local.create_value_reference(name.to_string(), reference);
        
        Ok(())
    }

    pub fn assign_variable(&mut self, name: &str, value: Value) -> io::Result<()> {
        let mut local = self.local.borrow_mut();

        local.assign_variable(name, value)?;

        Ok(())
    }

    pub fn assign_variable_by_reference(&mut self, reference: u64, value: Value) -> io::Result<()> {
        let mut local = self.local.borrow_mut();

        local.assign_variable_by_reference(reference, value)?;
        Ok(())
    }

    pub fn same_scope(&self, reference: u64) -> bool {
        let local = self.local.borrow();

        local.same_scope_reference(reference)
    } 

    pub fn variable_exists(&self, reference: u64) -> bool {
        let local = self.local.borrow();

        local.variable_exists(reference)
    }

    pub fn enter_enviroment(&mut self) {
        let previous = self.local.clone();
        let new_local = Rc::new(RefCell::new(Environment::with_parent(previous)));
        
        let native = NativeRegistry::get();
        {
            let mut native = native.borrow_mut();

            native.local = Some(new_local.clone());
        }

        self.local = new_local;
    }

    pub fn exit_enviroment(&mut self) -> io::Result<()> {
        let new_env = {
            let local = self.local.clone();
            let local_borrow = local.borrow();

            if let Some(parent) = local_borrow.get_parent() {
                Some(parent.clone())
            } else {
                None
            }
        };

        if let Some(env) = new_env {            
            let native = NativeRegistry::get();
            {
                let mut native = native.borrow_mut();

                native.local = Some(env.clone());
            }
            
            self.local = env;
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "No parent enviroment!"))
        }
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

    fn move_to_parent(&mut self, value: Value) {
        let mut local = self.local.borrow_mut();

        local.move_to_parent(value);
    }

    pub fn call_function(&mut self, name: &str, args: Vec<Value>) -> io::Result<Value> {        
        let registry = NativeRegistry::get();
        {
            let registry = registry.borrow();
            let native = registry.get_native(name);

            match native {
                Some(NativeTypes::NativeFunction(native_function)) => {
                    let value = (native_function.function)(args.clone())?;

                    return Ok(value);
                },
                _ => {}
            }
        }

        let value = self.get_variable(name)?;

        if let ValueType::Function { return_type: _, parameters, body } = value.get_type() {
            if args.len() != parameters.len() {
                return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Missmatch in function's singature \"{}\"!", name)));
            }

            self.enter_enviroment();

            for (arg, parameter) in args.iter().zip(parameters) {
                if arg.get_type().get_data_type() != parameter.data_type && !DataType::is_void(&arg.get_data_type()) {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Missmatch in function's singature \"{}\"!", name)));
                }

                if let Some(reference) = arg.get_reference() {
                    self.define_variable_by_reference(parameter.name.as_str(), reference)?;
                } else {
                    self.define_variable(parameter.name.as_str(), arg.clone())?;
                }
            }

            let result = self.interpret_statements(body.to_vec())?;
            
            match result {
                Return::Success(mut value) => {
                    match value.get_type() {
                        ValueType::List { references, data_type: _ } | ValueType::Tuple { references, data_types: _ } => {
                            for reference in references {
                                let list_value = self.get_variable_reference(*reference)?;
                                self.move_to_parent(list_value);
                            }
                        },
                        _ => {}
                    }
                    
                    if let Some(reference) = value.get_reference() {
                        if self.same_scope(reference) {
                            value.clear_reference();
                        }
                    }
                    
                    self.exit_enviroment()?;
                    
                    return Ok(value);
                },
                Return::Nothing => {

                    self.exit_enviroment()?;
                    return Ok(Value::new(None, ValueType::Null));
                }
            }
        }

        Err(io::Error::new(io::ErrorKind::InvalidData, format!("Variable {} is not a function!", name)))
    }
}
