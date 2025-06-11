
use std::io;

use crate::interpreter::value::{Value, ValueType};
use crate::native_registry::native_registry::NativeRegistry;
use crate::native_registry::rust_function::RustFunction;
use crate::parser::data_type::DataType;

fn ks_print(args: Vec<Value>) -> io::Result<Value> {
    for arg in args {
        let value_type = arg.get_type().clone();

        match value_type {
            ValueType::Integer(var) => print!("{}", var),
            ValueType::Float(var) => print!("{}", var),
            ValueType::Boolean(var) => print!("{}", var),
            ValueType::String(var) => print!("{}", var),
            ValueType::List { references, data_type: _ } => {
                print!("[");
                for (i, reference) in references.iter().enumerate() {
                    print!("&{}", reference);
                    // ks_print(vec![Value::new(None, var.clone().get_type().clone())])?;

                    if i != references.len() - 1 {
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

fn ks_println(args: Vec<Value>) -> io::Result<Value> {    
    ks_print(args)?;
    println!("");
    
    Ok(Value::new(None, ValueType::Null))
}

fn ks_len(args: Vec<Value>) -> io::Result<Value> {
    if args.len() > 1 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Too many arguments!"));
    }
    
    match args[0].get_type() {
        ValueType::String(str) => {
            Ok(Value::new(None, ValueType::Integer(str.len() as i32)))
        },
        ValueType::List { references, data_type } => {
            Ok(Value::new(None, ValueType::Integer(references.len() as i32)))
        },
        _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid Type!"))
    }
}

// fn ks_range(args: Vec<Value>) -> io::Result<Value> {
//     if args.len() > 1 {
//         return Err(io::Error::new(io::ErrorKind::InvalidData, "Too many arguments!"));
//     }

//     if let ValueType::Integer(n) = args[0].get_type() {
//         let mut out: Vec<Value> = Vec::new();

//         for i in 0..*n {
//             out.push(Value::new(None, ValueType::Integer(i)));
//         }

//         Ok(Value::new(None, ValueType::List(out)))
//     } else {
//         Err(io::Error::new(io::ErrorKind::InvalidData, "Wrong argument type!"))
//     }
// }

fn ks_ref(args: Vec<Value>) -> io::Result<Value> {
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

pub fn register_standart_library(native_registry: &mut NativeRegistry) {    
    native_registry.register_function("println", RustFunction::from(ks_println, DataType::void()));
    native_registry.register_function("print", RustFunction::from(ks_print, DataType::void()));
    native_registry.register_function("len", RustFunction::from(ks_len, DataType::Int));
    // native_registry.register_function("range", RustFunction::from(ks_range, DataType::List(Box::new(DataType::Int))));
    native_registry.register_function("ref", RustFunction::from(ks_ref, DataType::Int));
    // native_registry.register_function("clone", RustFunction::from(ks_clone, DataType::List(Box::new(DataType::void()))));
}