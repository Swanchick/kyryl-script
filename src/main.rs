mod lexer;
mod parser;

use lexer::lexer::Lexer; 
use parser::function::Function;
use parser::data_type::DataType;
use parser::parameter::Parameter;

fn main() {
    let mut lexer = Lexer::load("test.kys").unwrap();
    
    lexer.lexer().unwrap();

    for token in lexer.get_tokens() {
        println!("{}", token);
    }
}


#[cfg(test)]
mod tests;

