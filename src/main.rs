mod lexer;
mod parser;
mod interpreter;

use lexer::lexer::Lexer;


fn main() {
    let mut lexer = Lexer::load("test.kys").unwrap();
    
    lexer.lexer().unwrap();

    for (i, token) in lexer.get_tokens().iter().enumerate() {
        println!("{} {}", i, token);
    }
}


#[cfg(test)]
mod tests;

