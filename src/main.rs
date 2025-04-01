mod lexer;
mod parser;

use lexer::lexer::Lexer; 

fn main() {
    let mut lexer = Lexer::load("test.kys").unwrap();
    
    lexer.lexer().unwrap();

    for token in lexer.get_tokens() {
        println!("{}", token);
    }
}


#[cfg(test)]
mod tests;

