use std::io;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;


use super::value::Value;

#[derive(Debug, Clone)]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, u128>,
    references: HashMap<u128, Value>,
    last_reference: u128
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            parent: None,
            values: HashMap::new(),
            references: HashMap::new(),
            last_reference: 0
        }
    }

    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            parent: Some(parent),
            values: HashMap::new(),
            references: HashMap::new(),
            last_reference: 0
        }
    }

    pub fn get_values(&self) -> &HashMap<String, u128> {
        &self.values
    }

    pub fn get_references(&self) -> &HashMap<u128, Value> {
        &self.references
    }

    fn create_value(&mut self, name: String, value: Value) {
        let mut value = value;
        value.set_reference(self.last_reference);

        self.references.insert(self.last_reference, value);
        self.values.insert(name, self.last_reference);

        self.last_reference += 1;
    }

    pub fn variable_exists(&self, reference: u128) -> bool {
        if self.references.contains_key(&reference) {
            return true;
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().variable_exists(reference);
        }

        false
    }

    pub fn variable_is_used(&self, reference: u128) -> bool {
        if self.values.values().any(|&x| x == reference) {
            return true;
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().variable_is_used(reference);
        }
        
        false
    }

    fn check_variable(&self, name: &str, reference: &u128) -> bool {
        let reference = self.values.iter().find(|(_, &y)| y == reference.clone());

        if let Some((value_name, reference)) = reference {
            return value_name == name;
        }

        false
    }

    pub fn define_variable(&mut self, name: String, value: Value) -> io::Result<()> {
        match value.get_reference() {
            Some(reference) => {
                let is_existing = self.variable_exists(reference);
                let is_used = self.variable_is_used(reference);
                
                if !(is_existing && is_used) {
                    self.create_value(name, value);
                    return Ok(());
                }

                self.values.insert(name, reference);
            }
            None => {
                self.create_value(name, value);
            }
        }

        Ok(())
    }

    pub fn assign_variable(&mut self, name: &str, value: Value) -> io::Result<()> {
        let value = value;
        
        let original_value = self.get_variable(name)?;
        if original_value.get_type().get_data_type() != value.get_type().get_data_type() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid type for assignment!"))
        }

        if let Some(reference) = value.get_reference() {
            self.references.insert(reference, value);
            self.values.insert(name.to_string(), reference);

            return Ok(());
        } 
        if let Some(&reference) = self.values.get(name) {
            self.references.insert(reference, value);

            return Ok(());
        }

        Err(io::Error::new(io::ErrorKind::InvalidData, format!("Variable {name} does not exists!")))
    }

    pub fn free(&mut self, name: &str) {
        if let Some(&reference) = self.values.get(name) {
            self.values.remove(name);

            if !self.values.values().any(|&x| x == reference) {
                self.references.remove(&reference);
            }
        }
    }

    pub fn get_variable(&self, name: &str) -> io::Result<Value> {
        if let Some(reference) = self.values.get(name) {
            if let Some(value) = self.references.get(reference) {
                Ok(value.clone())
            } else if let Some(parent) = &self.parent {
                parent.borrow_mut().get_variable(name)
            } else {
                Err(io::Error::new(io::ErrorKind::InvalidData, format!("Variable {} does not exist!", name)))
            }
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().get_variable(name)
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, format!("Variable {} does not exist!", name)))
        }
    }
}

