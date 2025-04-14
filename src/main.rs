mod lexer;
mod parser;
mod interpreter;

use std::io;
use std::env;

use interpreter::{interpreter::Interpreter, value::Value};
use lexer::lexer::Lexer;
use parser::parser::Parser;

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

fn register_standart_library(interpreter: &mut Interpreter) {    
    interpreter.register_rust_function("println", kys_println);
    interpreter.register_rust_function("print", kys_print);
}

fn run_script(script_path: &str) -> io::Result<()> {
    let mut lexer = Lexer::load(script_path)?;
    lexer.lexer()?;

    let mut parser = Parser::new(lexer.get_tokens().clone());
    let functions = parser.parse_functions()?;

    let mut interpreter = Interpreter::new();
    register_standart_library(&mut interpreter);

    interpreter.interpret_program(functions)?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let script_path = args.get(1);

    if let Some(script_path) = script_path {
        let result = run_script(script_path);

        if let Err(e) = result {
            println!("========== KyrylScript Error!!!");
            println!("{}", e);
            println!("===============================");
        } 
    }
}


#[cfg(test)]
mod tests;

