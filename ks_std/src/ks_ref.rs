use std::io;

use core::interpreter::value::{Value, ValueType};

pub fn ks_ref(args: Vec<Value>) -> io::Result<Value> {
    if args.len() > 1 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Too many arguments!")); 
    }

    let value = args[0].clone();
    let reference = value.get_reference();
    if let Some(reference) = reference {
        Ok(Value::new(None, ValueType::Integer(reference as i32)))
    } else {
        Ok(Value::new(None, ValueType::Null))
    }
}
