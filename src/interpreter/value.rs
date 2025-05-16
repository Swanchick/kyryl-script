use std::rc::Rc;
use std::cell::RefCell;
use std::io;

use crate::parser::data_type::DataType;
use crate::parser::function::Function;
use crate::parser::parameter::Parameter;
use crate::parser::statement::Statement;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Integer(i32),
    Float(f64),
    String(String),
    Boolean(bool),
    RustFunction(fn(args: Vec<Value>) -> io::Result<Value>),
    List(Vec<Value>),
    Void,
    Function {
        name: String,
        return_type: DataType,
        parameters: Vec<Parameter>,
        body: Vec<Statement>
    },
}


#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    reference: Option<u128>,
    value_type: ValueType
}

impl Value {
    pub fn new(reference: Option<u128>, value_type: ValueType) -> Value {
        Value {
            reference,
            value_type
        }
    }

    pub fn set_reference(&mut self, reference: u128) {
        self.reference = Some(reference);
    }

    pub fn get_reference(&self) -> Option<u128> {
        self.reference
    }

    pub fn get_type(&self) -> &ValueType {
        &self.value_type
    }
}

impl ValueType {
    pub fn get_data_type(&self) -> DataType {
        match self {
            ValueType::Integer(_) => DataType::Int,
            ValueType::Float(_) => DataType::Float,
            ValueType::String(_) => DataType::String,
            ValueType::Boolean(_) => DataType::Bool,
            ValueType::Function{ .. } => DataType::Function,
            ValueType::RustFunction(_) => DataType::Function,
            ValueType::List(list) => {
                if list.len() != 0 {
                    DataType::List(Box::new(list[0].get_type().get_data_type()))
                } else {
                    DataType::List(Box::new(DataType::Void))
                }
            }
            ValueType::Void => DataType::Void,
        }
    }
}