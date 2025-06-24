
use std::io;

use crate::interpreter::value::{Value, ValueType};
use crate::native_registry::native_buffer::NativeBuffer;
use crate::native_registry::native_registry::NativeRegistry;
use crate::native_registry::native_function::NativeFunction;
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

pub fn register_standart_library() {
    let mut buffer = NativeBuffer::new();

    buffer.add_function("print", NativeFunction::process(ks_print));
    buffer.add_function("println", NativeFunction::process(ks_println));

    let registry = NativeRegistry::get();
    {
        let mut registry = registry.borrow_mut();

        registry.add_buffer(buffer);
    }
}