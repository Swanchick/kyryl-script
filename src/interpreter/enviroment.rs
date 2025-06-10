use std::io;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::atomic::{AtomicU64, Ordering};

use super::value::Value;
use super::variable_slot::VariableSlot;


static GLOBAL_REFERENCE_COUNT: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone)]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, u64>,
    references: HashMap<u64, VariableSlot>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            parent: None,
            values: HashMap::new(),
            references: HashMap::new(),
        }
    }

    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            parent: Some(parent),
            values: HashMap::new(),
            references: HashMap::new(),
        }
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<Environment>>> {
        match &self.parent {
            Some(parent) => Some(parent.clone()),
            None => None
        }
    }

    fn next_reference(&self) -> u64 {
        GLOBAL_REFERENCE_COUNT.fetch_add(1, Ordering::SeqCst)
    }

    fn create_value(&mut self, name: String, mut value: Value) {
        let reference = self.next_reference();
        
        value.set_reference(reference);
        self.references.insert(reference, VariableSlot::Variable(value));
        self.values.insert(name, reference);
    }

    pub fn create_value_reference(&mut self, name: String, reference: u64) {        
        self.values.insert(name, reference);
        self.references.insert(reference, VariableSlot::Reference(reference));
    }

    pub fn variable_exists(&self, reference: u64) -> bool {
        if self.references.contains_key(&reference) {
            return true;
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().variable_exists(reference);
        }

        false
    }

    pub fn variable_is_used(&self, reference: u64) -> bool {
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
                let same_scope = self.same_scope_reference(reference);
                let is_existing = self.variable_exists(reference);
                let is_used = self.variable_is_used(reference);

                if same_scope {
                    if is_existing && is_used {
                        self.values.insert(name, reference);
                    } else {
                        self.create_value(name, value);
                    }
                } else {
                    self.create_value_reference(name, reference);
                }
            }
            None => {
                self.create_value(name, value);
            }
        }

        Ok(())
    }

    pub fn assign_variable_by_reference(&mut self, reference: u64, mut value: Value) -> io::Result<()> {
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
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().assign_variable(name, value)
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, format!("Variable {} does not exist!", name)))
        }
    }

    pub fn get_by_reference(&self, reference: u64) -> io::Result<Value> {
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
        } else if let Some(parent) = &self.parent {
            return parent.borrow().get_by_reference(reference);
        }

        return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Reference not found {}!", reference)));
    }

    pub fn display_references(&self) {
        for name in self.values.keys() {
            let reference = self.values.get(name).unwrap();
            let slot = self.references.get(reference).unwrap();
            println!("{}({}) = {:?}", name, reference, slot);
        }
    }

    fn same_scope_reference(&self, reference: u64) -> bool {
        if let Some(_) = self.references.get(&reference) {
            true
        } else {
            false
        }
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

        Err(io::Error::new(io::ErrorKind::InvalidData, format!("Variable asdwasdw {} does not exist!", name)))
    }
}

