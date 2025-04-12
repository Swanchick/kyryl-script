use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::rc::Rc;

use crate::interpreter;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::lexer::lexer::Lexer;
use crate::parser::expression::Expression;
use crate::parser::function::{self, Function};
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

fn get_function(function_str: &str) -> Function {
    let function_str = function_str.to_string();

    let mut lexer = Lexer::new(function_str);
    lexer.lexer().unwrap();
    
    let mut parser = Parser::new(lexer.get_tokens().clone());
    parser.parse_function().unwrap()
}

#[test]
fn test_interpreter_plus() {
    let expression = get_expression("5 + 2");

    let mut interpreter = Interpreter::new();

    let test_value = Value::Integer(7);
    let value = interpreter.interpret_expression(expression).unwrap();

    assert_eq!(value, test_value);
}


#[test]
fn test_interpreter_minus() {
    let mut interpreter = Interpreter::new();

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

    let mut interpreter = Interpreter::new();
    let value = interpreter.interpret_expression(expression).unwrap();

    assert_eq!(value, test_value);
}


#[test]
fn test_interpreter_even_more_complex() {
    let test_value = Value::Float(117.5);
    
    let expression = get_expression("(22 + 3) / 10 + 5 * 25 - 10");

    let mut interpreter = Interpreter::new();
    let value = interpreter.interpret_expression(expression).unwrap();

    assert_eq!(value, test_value);
}

#[test]
fn test_interpreter_even_more_complex_2() {
    let test_value = Value::Float(-235.0);
    
    let expression = get_expression("((22 + 3) / 10 + 5 * 25 - 10) * -2");

    let mut interpreter = Interpreter::new();
    let value = interpreter.interpret_expression(expression).unwrap();

    assert_eq!(value, test_value);
}

#[test]
fn test_interpreter_string_error() {
    let err = io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!");
    
    let expression = get_expression("((22 + 3) / 10 + 5 * 25 - \"Hello World\") * -2");

    let mut interpreter = Interpreter::new();
    let value = interpreter.interpret_expression(expression).unwrap_err();

    assert_eq!(value.to_string(), err.to_string());
}

#[test]
fn test_interpreter_add_strings() {
    let expression = get_expression("\"Hello\" + \" World\"");
    let test_value = Value::String(String::from("Hello World"));

    let mut interpreter = Interpreter::new();

    let value = interpreter.interpret_expression(expression).unwrap();

    assert_eq!(value, test_value);

}

#[test]
fn test_interpreter_boolean_false_1() {
    let expression = get_expression("22 == 33");
    let test_value = Value::Boolean(false);

    let mut interpreter = Interpreter::new();
    let value = interpreter.interpret_expression(expression).unwrap();

    assert_eq!(value, test_value)
}

#[test]
fn test_interpreter_boolean_true_1() {
    let expression = get_expression("\"Hello World\" == \"Hello World\"");
    let test_value = Value::Boolean(true);

    let mut interpreter = Interpreter::new();
    let value = interpreter.interpret_expression(expression).unwrap();

    assert_eq!(value, test_value)
}

#[test]
fn test_interpreter_boolean_true_2() {
    let expression = get_expression("\"Hello World\" == \"Hello World\" && 22 == 22 || 90 == 10");
    let test_value = Value::Boolean(true);

    let mut interpreter = Interpreter::new();
    let value = interpreter.interpret_expression(expression).unwrap();

    assert_eq!(value, test_value)
}

#[test]
fn test_interpreter_boolean_true_3() {
    let expression = get_expression("\"Hello World\" == \"Hello World\" && 22 == 22 && 90 == 10");
    let test_value = Value::Boolean(false);

    let mut interpreter = Interpreter::new();
    let value = interpreter.interpret_expression(expression).unwrap();

    assert_eq!(value, test_value)
}


#[test]
fn test_interpreter_var_dec_statement() {
    let test_statement = get_statement("let a = 10 + 20;");
    let test_hash = HashMap::from([
        (String::from("a"), Value::Integer(30))
    ]);

    let mut interpreter = Interpreter::new();
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

    let mut interpreter = Interpreter::new();
    interpreter.interpret_statement(test_statement1).unwrap();
    interpreter.interpret_statement(test_statement2).unwrap();
    let env = interpreter.get_local();

    assert_eq!(env.borrow().get_values(), &test_hash);
}

#[test]
fn test_interpreter_return_statement() {
    let test_statement = get_statement("return \"Hello\" + \" World\";");
    let test_value = Some(Value::String(String::from("Hello World")));

    let mut interpreter = Interpreter::new();
    let value = interpreter.interpret_statement(test_statement).unwrap();

    assert_eq!(value, test_value);
}

#[test]
fn test_interpreter_var_dec_identefier_statement() {
    let test_statement1 = get_statement("let a = 10;");
    let test_statement2 = get_statement("let b = 20;");
    let test_statement3 = get_statement("let c = a + b;");
    let test_value = Value::Integer(30);

    let mut interpreter = Interpreter::new();
    interpreter.interpret_statement(test_statement1).unwrap();
    interpreter.interpret_statement(test_statement2).unwrap();
    interpreter.interpret_statement(test_statement3).unwrap();
    
    let value = interpreter.get_local().borrow().get_variable("c").unwrap();

    assert_eq!(value, test_value);
}


#[test]
fn test_interpreter_function() {
    let source1 = concat!(
        "function add(a: int, b: int): int {\n",
        "   return a + b;\n",
        "}\n"
    );

    let test_value = Value::Integer(100);
    
    let function = Rc::new(RefCell::new(get_function(source1)));


    let expression = get_expression("add(20, 30) + 50");

    let mut interpreter = Interpreter::new();
    interpreter.get_local().borrow_mut().define_variable(String::from("add"), Value::Function(function));

    let value = interpreter.interpret_expression(expression).unwrap();

    assert_eq!(value, test_value);
}

#[test]
fn test_interpreter_rust_function() {
    let test_value = Value::Integer(100);

    let add_function: fn(args: Vec<Value>) -> io::Result<Value> = |args: Vec<Value>| {
        if args.len() != 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Huh?"));
        }

        let a = args[0].clone();
        let b = args[1].clone();

        match (a, b) {
            (Value::Integer(n1), Value::Integer(n2)) => {
                Ok(Value::Integer(n1 + n2))
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Huh? 2"))
        }
    };

    let expression = get_expression("add(20, 30) + 50");
    let mut interpreter = Interpreter::new();
    interpreter.register_rust_function("add", add_function);
    let value = interpreter.interpret_expression(expression).unwrap();

    assert_eq!(value, test_value);
}

