
use std::io;

use crate::interpreter::value::{Value, ValueType};
use crate::native_registry::native_registry::NativeRegistry;
use crate::native_registry::rust_function::RustFunction;
use crate::parser::data_type::DataType;

fn kys_print(args: Vec<Value>) -> io::Result<Value> {
    for arg in args {
        let value_type = arg.get_type().clone();

        match value_type {
            ValueType::Integer(var) => print!("{}", var),
            ValueType::Float(var) => print!("{}", var),
            ValueType::Boolean(var) => print!("{}", var),
            ValueType::String(var) => print!("{}", var),
            ValueType::List(vars) => {
                print!("[");
                for (i, var) in vars.iter().enumerate() {
                    kys_print(vec![Value::new(None, var.clone().get_type().clone())])?;

                    if i != vars.len() - 1 {
                        print!(", ")
                    }
                }
                print!("]")
            }
            ValueType::Null => print!("null"),
            _ => return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unsupported value to print: {}", value_type.get_data_type())
            ))
        }
    }
    
    Ok(Value::new(None, ValueType::Null))
}

fn kys_println(args: Vec<Value>) -> io::Result<Value> {    
    kys_print(args)?;
    println!("");
    
    Ok(Value::new(None, ValueType::Null))
}

fn kys_len(args: Vec<Value>) -> io::Result<Value> {
    if args.len() > 1 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Too many arguments!"));
    }
    
    match args[0].get_type() {
        ValueType::String(str) => {
            Ok(Value::new(None, ValueType::Integer(str.len() as i32)))
        },
        ValueType::List(list) => {
            Ok(Value::new(None, ValueType::Integer(list.len() as i32)))
        },
        _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid Type!"))
    }
}

fn kys_range(args: Vec<Value>) -> io::Result<Value> {
    if args.len() > 1 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Too many arguments!"));
    }

    if let ValueType::Integer(n) = args[0].get_type() {
        let mut out: Vec<Value> = Vec::new();

        for i in 0..*n {
            out.push(Value::new(None, ValueType::Integer(i)));
        }

        Ok(Value::new(None, ValueType::List(out)))
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidData, "Wrong argument type!"))
    }
}

pub fn register_standart_library(native_registry: &mut NativeRegistry) {    
    native_registry.register_function("println", RustFunction::from(kys_println, DataType::void()));
    native_registry.register_function("print", RustFunction::from(kys_print, DataType::void()));
    native_registry.register_function("len", RustFunction::from(kys_len, DataType::Int));
    native_registry.register_function("range", RustFunction::from(kys_range, DataType::List(Box::new(DataType::Int))));
}