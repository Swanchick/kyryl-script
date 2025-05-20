use std::io;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use super::value::Value;
use super::variable_slot::VariableSlot;

#[derive(Debug, Clone)]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, u128>,
    references: HashMap<u128, VariableSlot>,
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

    fn create_value(&mut self, name: String, mut value: Value) {
        value.set_reference(self.last_reference);
        self.references.insert(self.last_reference, VariableSlot::Variable(value));
        self.values.insert(name, self.last_reference);

        self.last_reference += 1;
    }

    pub fn create_value_reference(&mut self, name: String, reference: u128) {
        self.values.insert(name, self.last_reference);
        self.references.insert(self.last_reference, VariableSlot::Reference(reference));

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

    pub fn assign_variable_by_reference(&mut self, reference: u128, mut value: Value) -> io::Result<()> {
        if let Some(slot) = self.references.get(&reference) {
            match slot.clone() {
                VariableSlot::Variable(_) => {                    
                    value.set_reference(reference);
                    self.references.insert(reference, VariableSlot::Variable(value.clone()));
                }

                VariableSlot::Reference(parent_reference) => {
                    self.assign_variable_by_reference(parent_reference, value)?;
                }
            }
        }
        
        
        Ok(())
    }

    pub fn assign_variable(&mut self, name: &str, value: Value) -> io::Result<()> {
        let expected = self.get_variable(name)?;

        if expected.get_type().get_data_type() != value.get_type().get_data_type() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid type for assignment!"));
        }

        if let Some(&reference) = self.values.get(name) {
            if let Some(slot) = self.references.get(&reference) {
                if let VariableSlot::Reference(parent_reference) = slot {
                    if let Some(parent) = &self.parent {
                        return parent.borrow_mut().assign_variable_by_reference(parent_reference.clone(), value);
                    } else {
                        return Err(io::Error::new(io::ErrorKind::InvalidData, format!("")))
                    }
                }
            }
            
            self.references.insert(reference, VariableSlot::Variable(Value::new(Some(reference), value.get_type().clone())));
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, format!("Variable {name} does not exist!")))
        }
    }

    pub fn get_by_reference(&self, reference: u128) -> io::Result<Value> {
        if let Some(slot) = self.references.get(&reference) {
            match slot {
                VariableSlot::Variable(value) => {
                    return Ok(value.clone());
                }
                VariableSlot::Reference(parent_reference) => {
                    let parent_reference = parent_reference.clone();
                    
                    if let Some(parent) = &self.parent {
                        return parent.borrow().get_by_reference(parent_reference);
                    }
                }
            }
        }

        return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Reference not found {}!", reference)));
    }

    pub fn get_variable(&self, name: &str) -> io::Result<Value> {
        if let Some(reference) = self.values.get(name) {
            if let Some(slot) = self.references.get(reference) {
                match slot {
                    VariableSlot::Variable(value) => {
                        return Ok(value.clone());
                    }

                    VariableSlot::Reference(parent_reference) => {
                        let parent_reference = parent_reference.clone();
                        
                        if let Some(parent) = &self.parent {
                            return parent.borrow().get_by_reference(parent_reference);
                        } 

                        return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Variable {} does not exist!", name)));
                    }
                }
            }
        }
        
        if let Some(parent) = &self.parent {
            return parent.borrow().get_variable(name)
        }

        Err(io::Error::new(io::ErrorKind::InvalidData, format!("Variable {} does not exist!", name)))
    }
}

