mod lexer;
mod parser;
mod interpreter;
mod kys_std;

use std::io;
use std::env;

use interpreter::interpreter::Interpreter;
use lexer::lexer::Lexer;
use parser::parser::Parser;
use kys_std::register_standart_library;



fn run_script(script_path: &str) -> io::Result<()> {
    let mut lexer = Lexer::load(script_path)?;
    lexer.lexer()?;

    let mut parser = Parser::new(lexer.get_tokens().clone());
    let statements = parser.parse_block_statement()?;

    let mut interpreter = Interpreter::new();
    register_standart_library(&mut interpreter);

    interpreter.interpret_statements(statements)?;

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

