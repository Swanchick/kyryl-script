use std::io;

use core::interpreter::value::{Value, ValueType};


pub fn ks_len(args: Vec<Value>) -> io::Result<Value> {
    if args.len() > 1 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Too many arguments!"));
    }
    
    match args[0].get_type() {
        ValueType::String(str) => {
            Ok(Value::new(None, ValueType::Integer(str.len() as i32)))
        },
        ValueType::List { references, data_type: _ } => {
            Ok(Value::new(None, ValueType::Integer(references.len() as i32)))
        },
        _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid Type!"))
    }
}