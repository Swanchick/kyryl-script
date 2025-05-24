use std::io;

use crate::interpreter::value::Value;
use crate::parser::data_type::DataType;


pub struct RustFunction {
    pub function: fn(args: Vec<Value>) -> io::Result<Value>,
    pub return_type: DataType
}

impl RustFunction {
    pub fn from(function: fn(args: Vec<Value>) -> io::Result<Value>, return_type: DataType) -> RustFunction {
        RustFunction { 
            function: function, 
            return_type: return_type 
        }
    }
}
