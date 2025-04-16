
use std::io;
use crate::interpreter::{interpreter::Interpreter, value::Value};

fn kys_print(args: Vec<Value>) -> io::Result<Value> {
    for arg in args {
        match arg {
            Value::Integer(var) => print!("{}", var),
            Value::Float(var) => print!("{}", var),
            Value::Boolean(var) => print!("{}", var),
            Value::String(var) => print!("{}", var),
            Value::List(vars) => {
                print!("[");
                for (i, var) in vars.iter().enumerate() {
                    kys_print(vec![var.clone()])?;

                    if i != vars.len() - 1 {
                        print!(", ")
                    }
                }
                print!("]")
            }
            Value::Void => print!("void :)"),
            _ => return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unsupported value to print: {}", arg.get_data_type())
            ))
        }
    }
    
    Ok(Value::Void)
}

fn kys_println(args: Vec<Value>) -> io::Result<Value> {
    kys_print(args)?;
    println!("");
    
    Ok(Value::Void)
}

fn kys_len(args: Vec<Value>) -> io::Result<Value> {
    if args.len() > 1 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Too many arguments!"));
    }
    
    match &args[0] {
        Value::String(str) => {
            Ok(Value::Integer(str.len() as i32))
        },
        Value::List(list) => {
            Ok(Value::Integer(list.len() as i32))
        },
        _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid Type!"))
    }
}

fn kys_range(args: Vec<Value>) -> io::Result<Value> {
    if args.len() > 1 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Too many arguments!"));
    }

    if let Value::Integer(n) = &args[0] {
        let mut out: Vec<Value> = Vec::new();

        for i in 0..*n {
            out.push(Value::Integer(i));
        }

        Ok(Value::List(out))
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidData, "Wrong argument type!"))
    }
}

pub fn register_standart_library(interpreter: &mut Interpreter) {    
    interpreter.register_rust_function("println", kys_println);
    interpreter.register_rust_function("print", kys_print);
    interpreter.register_rust_function("len", kys_len);
    interpreter.register_rust_function("range", kys_range);
}