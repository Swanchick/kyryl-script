use std::io;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use super::value::Value;

#[derive(Debug, Clone)]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Value>
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            parent: None,
            values: HashMap::new()
        }
    }

    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            parent: Some(parent),
            values: HashMap::new()
        }
    }

    pub fn get_values(&self) -> &HashMap<String, Value> {
        &self.values
    }

    pub fn define_variable(&mut self, name: String, value: Value) {
        self.values.insert(name.to_string(), value);
    }

    ///
    /// SO MUCH RECURSION (:O)-<--<
    ///                    ^- literally me right now
    pub fn assign_variable(&mut self, name: &str, value: Value) -> io::Result<()> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().assign_variable(name, value)
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown value!"))
        }
    }

    pub fn get_variable(&self, name: &str) -> io::Result<Value> {
        if let Some(value) = self.values.get(name) {
            Ok(value.to_owned())
        } else if let Some(parent) = &self.parent {
            parent.borrow().get_variable(name)
        } else {
            todo!()
        }
    }
}

