use std::io;

use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::lexer::lexer::Lexer;
use crate::parser::expression::Expression;
use crate::parser::operator::Operator;
use crate::parser::parser::Parser;


fn get_expression(expression_str: &str) -> Expression {
    let expression_str = expression_str.to_string();
    
    let mut lexer = Lexer::new(expression_str);
    lexer.lexer().unwrap();
    
    let mut parser = Parser::new(lexer.get_tokens().clone());
    parser.parse_expression().unwrap()
}


#[test]
fn test_interpreter_plus() {
    let expression = get_expression("5 + 2");

    let interpreter = Interpreter::new();

    let test_value = Value::Integer(7);
    let value = interpreter.interpret_expression(expression).unwrap();

    assert_eq!(value, test_value);
}


#[test]
fn test_interpreter_minus() {
    let interpreter = Interpreter::new();

    let expression = Expression::BinaryOp {
        left: Box::new(Expression::IntegerLiteral(10)),
        operator: Operator::Minus,
        right: Box::new(Expression::IntegerLiteral(7))
    };

    let test_value = Value::Integer(3);
    let value = interpreter.interpret_expression(expression).unwrap();

    assert_eq!(value, test_value);
}


#[test]
fn test_interpreter_complex() {
    let test_value = Value::Integer(15);
    
    let expression = get_expression("-10 + 22 + 3");

    let interpreter = Interpreter::new();
    let value = interpreter.interpret_expression(expression).unwrap();

    assert_eq!(value, test_value);
}


#[test]
fn test_interpreter_even_more_complex() {
    let test_value = Value::Float(117.5);
    
    let expression = get_expression("(22 + 3) / 10 + 5 * 25 - 10");

    let interpreter = Interpreter::new();
    let value = interpreter.interpret_expression(expression).unwrap();

    assert_eq!(value, test_value);
}

#[test]
fn test_interpreter_even_more_complex_2() {
    let test_value = Value::Float(-235.0);
    
    let expression = get_expression("((22 + 3) / 10 + 5 * 25 - 10) * -2");

    let interpreter = Interpreter::new();
    let value = interpreter.interpret_expression(expression).unwrap();

    assert_eq!(value, test_value);
}

#[test]
fn test_interpreter_string_error() {
    let err = io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!");
    
    let expression = get_expression("((22 + 3) / 10 + 5 * 25 - \"Hello World\") * -2");

    let interpreter = Interpreter::new();
    let value = interpreter.interpret_expression(expression).unwrap_err();

    assert_eq!(value.to_string(), err.to_string());
}

#[test]
fn test_interpreter_add_strings() {
    let expression = get_expression("\"Hello\" + \" World\"");
    let test_value = Value::String(String::from("Hello World"));

    let interpreter = Interpreter::new();

    let value = interpreter.interpret_expression(expression).unwrap();

    assert_eq!(value, test_value);

}
