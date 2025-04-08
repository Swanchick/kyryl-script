use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::io;

use crate::parser::function::Function;
use super::enviroment::Enviroment;
use super::value::Value;

pub struct Interpreter {
    global: Rc<RefCell<Enviroment>>,
    enviroment: Rc<RefCell<Enviroment>>
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let global = Rc::new(RefCell::new(Enviroment::new()));

        Interpreter {
            global: global.clone(),
            enviroment: global
        }
    }
 
    pub fn interpret_program(&self, functions: Vec<Function>) -> io::Result<()> {
        for function in &functions {
            self.global.borrow_mut().define_variable(
                &function.name,
                Value::Function(Rc::new(RefCell::new(function.clone())))
            );
        }
        
        match self.global.borrow().get_variable("main") {
            Ok(Value::Function(_)) => self.call_function("main", Vec::new()),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Function main is not found!"))
        }
    }

    fn call_function(&self, name: &str, args: Vec<Value>) -> io::Result<()> {
        
        
        Ok(())
    }
}