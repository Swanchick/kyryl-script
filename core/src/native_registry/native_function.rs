use std::io;

use crate::interpreter::value::Value;
use crate::parser::data_type::DataType;


#[derive(Debug, Clone)]
pub struct NativeFunction {
    pub function: fn(args: Vec<Value>) -> io::Result<Value>,
    pub return_type: DataType
}

impl NativeFunction {
    pub fn from(function: fn(args: Vec<Value>) -> io::Result<Value>, return_type: DataType) -> NativeFunction {
        NativeFunction { 
            function: function, 
            return_type: return_type 
        }
    }
    
    pub fn process(function: fn(args: Vec<Value>) -> io::Result<Value>) -> NativeFunction {
        NativeFunction {
            function,
            return_type: DataType::void()
        }
    }
}
