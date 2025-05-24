use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;


#[test]
fn test_variable_declatarion_with_type() {
    let source = String::from("let variable: int = 123;");
    let mut lexer = Lexer::new(source);

    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone());
    parser.parse_block_statement().unwrap();
}


#[test]
fn test_variable_declatarion_with_type_error() {
    let source = String::from("let variable: float = 123;");
    let mut lexer = Lexer::new(source);

    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone());
    let err = parser.parse_block_statement().unwrap_err();


    assert_eq!(err.to_string(), "Differenet data types in expression and actual data type.")
}


#[test]
fn test_function_enviroment_parameters() {
    let source = concat!(
        "function foo(bar: int): int {\n",
        "    let a: int = bar;\n",
        "}\n",
    );

    let mut lexer = Lexer::new(source.to_string());

    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone());
    parser.parse_block_statement().unwrap();
}


#[test]
fn test_function_enviroment_parameters_error() {
    let source = concat!(
        "function foo(bar: float): int {\n",
        "    let a: int = bar;\n",
        "}\n",
    );

    let mut lexer = Lexer::new(source.to_string());

    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone());
    assert_eq!(parser.parse_block_statement().unwrap_err().to_string(), "Differenet data types in expression and actual data type.");
}

#[test]
fn test_function_enviroment_parameters_out_of_function() {
    let source = concat!(
        "function foo(bar: float): int {\n",
        "}\n",
        "let a: int = bar;"
    );

    let mut lexer = Lexer::new(source.to_string());

    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone());
    assert_eq!(parser.parse_block_statement().unwrap_err().to_string(), "Variable bar not found!");
}

#[test]
fn test_function_enviroment_return_mismatch() {
    let source = concat!(
        "function foo(): float {\n",
        "    return 100;\n",
        "}\n",
    );

    let mut lexer = Lexer::new(source.to_string());

    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone());
    assert_eq!(parser.parse_block_statement().unwrap_err().to_string(), "Mismatch return and function return types!");
}
