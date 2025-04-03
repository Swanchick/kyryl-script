mod lexer;
mod parser;

use lexer::{lexer::Lexer, token::Token};

fn main() {
    let mut lexer = Lexer::load("test.kys").unwrap();
    
    lexer.lexer().unwrap();

    for (i, token) in lexer.get_tokens().iter().enumerate() {
        println!("{} {}", i, token);
    }

    let il = Token::IntegerLiteral(10);

    // println!("{}", )
    
}


#[cfg(test)]
mod tests;

