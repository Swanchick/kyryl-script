mod lexer;
mod parser;

use lexer::lexer::Lexer;
use lexer::token::Token;

fn check(token1: Token, token2: Token) -> bool {
    matches!(token1, token2)
}

fn test_function(num: &mut i32, a: i32) -> bool {
    println!("{}: {}", a, num);

    *num += 1 * a;

    let a2 = num.clone() + a;

    a2 <= 10
}

fn main() {
    let mut lexer = Lexer::load("test.kys").unwrap();
    
    lexer.lexer().unwrap();

    for (i, token) in lexer.get_tokens().iter().enumerate() {
        println!("{} {}", i, token);
    }

    let mut a = 1;

    while test_function(&mut a, 1) && test_function(&mut a, 2) {
        
    }
}


#[cfg(test)]
mod tests;

