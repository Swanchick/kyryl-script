
use std::io;

use core::interpreter::value::{Value, ValueType};
use core::native_registry::native_registry::NativeRegistry;

pub fn ks_print(args: Vec<Value>) -> io::Result<Value> {
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
                    let native = NativeRegistry::get();
                    {
                        let native = native.borrow();
                        let env = &native.local;
                        if let Some(env) = env {
                            let env = env.borrow();

                            let value = env.get_by_reference(reference.clone())?;
                            ks_print(vec![value])?;
                        }
                    }
                    
                    if i  < references.len() - 1 {
                        print!(", ");
                    }     
                }

                print!("]");
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

pub fn ks_println(args: Vec<Value>) -> io::Result<Value> {    
    ks_print(args)?;
    println!("");
    
    Ok(Value::new(None, ValueType::Null))
}