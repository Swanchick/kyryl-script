mod lexer;
mod parser;
mod interpreter;

use std::io;

use interpreter::{interpreter::Interpreter, value::Value};
use lexer::lexer::Lexer;
use parser::parser::Parser;


fn main() {
    let mut lexer = Lexer::load("test.kys").unwrap();
    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone());
    let functions = parser.parse_functions().unwrap();

    let mut interpreter = Interpreter::new();
    interpreter.register_rust_function("print", |args: Vec<Value>| {
        let arg = args.get(0);

        if let Some(arg) = arg {
            match arg {
                Value::Integer(var) => println!("{}", var),
                Value::Float(var) => println!("{}", var),
                Value::Boolean(var) => println!("{}", var),
                Value::String(var) => println!("{}", var),
                Value::Void => println!("void :)"),
                _ => return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Unsupported value to print: {}", arg.get_data_type())
                ))
            }
        }
        
        Ok(Value::Void)
    });

    interpreter.register_rust_function("unprint", |args: Vec<Value>| {
        let arg = args.get(0);

        if let Some(arg) = arg {
            match arg {
                Value::Integer(var) => println!("Delete {}", var),
                Value::Float(var) => println!("Delete {}", var),
                Value::Boolean(var) => println!("Delete {}", var),
                Value::String(var) => println!("Delete {}", var),
                Value::Void => println!("void :)"),
                _ => return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Unsupported value to print: {}", arg.get_data_type())
                ))
            }
        }

        Ok(Value::Void)
    });

    interpreter.interpret_program(functions).unwrap();
}


#[cfg(test)]
mod tests;

