use std::collections::HashMap;
use std::io;

use crate::interpreter;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::lexer::lexer::Lexer;
use crate::parser::expression::Expression;
use crate::parser::operator::Operator;
use crate::parser::parser::Parser;
use crate::parser::statement::Statement;


fn get_expression(expression_str: &str) -> Expression {
    let expression_str = expression_str.to_string();
    
    let mut lexer = Lexer::new(expression_str);
    lexer.lexer().unwrap();
    
    let mut parser = Parser::new(lexer.get_tokens().clone());
    parser.parse_expression().unwrap()
}

fn get_statement(statement_str: &str) -> Statement {
    let statement_str = String::from(statement_str);
    let mut lexer = Lexer::new(statement_str);
    lexer.lexer().unwrap();
    
    let mut parser = Parser::new(lexer.get_tokens().clone());
    parser.parse_statement().unwrap()
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

#[test]
fn test_interpreter_boolean_false_1() {
    let expression = get_expression("22 == 33");
    let test_value = Value::Boolean(false);

    let interpreter = Interpreter::new();
    let value = interpreter.interpret_expression(expression).unwrap();

    assert_eq!(value, test_value)
}

#[test]
fn test_interpreter_boolean_true_1() {
    let expression = get_expression("\"Hello World\" == \"Hello World\"");
    let test_value = Value::Boolean(true);

    let interpreter = Interpreter::new();
    let value = interpreter.interpret_expression(expression).unwrap();

    assert_eq!(value, test_value)
}

#[test]
fn test_interpreter_boolean_true_2() {
    let expression = get_expression("\"Hello World\" == \"Hello World\" && 22 == 22 || 90 == 10");
    let test_value = Value::Boolean(true);

    let interpreter = Interpreter::new();
    let value = interpreter.interpret_expression(expression).unwrap();

    assert_eq!(value, test_value)
}

#[test]
fn test_interpreter_boolean_true_3() {
    let expression = get_expression("\"Hello World\" == \"Hello World\" && 22 == 22 && 90 == 10");
    let test_value = Value::Boolean(false);

    let interpreter = Interpreter::new();
    let value = interpreter.interpret_expression(expression).unwrap();

    assert_eq!(value, test_value)
}


#[test]
fn test_interpreter_var_dec_statement() {
    let test_statement = get_statement("let a = 10 + 20;");
    let test_hash = HashMap::from([
        (String::from("a"), Value::Integer(30))
    ]);

    let interpreter = Interpreter::new();
    interpreter.interpret_statement(test_statement).unwrap();

    let env = interpreter.get_local();

    assert_eq!(env.borrow().get_values(), &test_hash);
}

#[test]
fn test_interpreter_assigment_statement() {
    let test_statement1 = get_statement("let a = 10 + 20;");
    let test_statement2 = get_statement("a = 50;");
    
    
    let test_hash = HashMap::from([
        (String::from("a"), Value::Integer(50))
    ]);

    let interpreter = Interpreter::new();
    interpreter.interpret_statement(test_statement1).unwrap();
    interpreter.interpret_statement(test_statement2).unwrap();
    let env = interpreter.get_local();

    assert_eq!(env.borrow().get_values(), &test_hash);
}

#[test]
fn test_interpreter_return_statement() {
    let test_statement = get_statement("return \"Hello\" + \" World\";");
    let test_value = Some(Value::String(String::from("Hello World")));

    let interpreter = Interpreter::new();
    let value = interpreter.interpret_statement(test_statement).unwrap();

    assert_eq!(value, test_value);
}

