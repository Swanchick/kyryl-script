use std::mem::discriminant;
use std::rc::Rc;
use std::cell::RefCell;

use crate::parser::data_type::DataType;
use crate::parser::function::Function;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i32),
    Float(f64),
    String(String),
    Boolean(bool),
    Function(Rc<RefCell<Function>>),
    Void
}


impl Value {
    pub fn get_data_type(&self) -> DataType {
        match self {
            Value::Integer(_) => DataType::Int,
            Value::Float(_) => DataType::Float,
            Value::String(_) => DataType::String,
            Value::Boolean(_) => DataType::Bool,
            Value::Function(_) => DataType::Function,
            Value::Void => DataType::Void,
        }
    }
}