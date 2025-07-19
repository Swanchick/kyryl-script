use std::io;

use crate::global::ks_path::KsPath;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::ValueType;

use crate::lexer::lexer::Lexer;
use crate::parser::expression::Expression;
use crate::parser::operator::Operator;
use crate::parser::parser::Parser;


fn get_expression(expression_str: &str) -> Expression {
    let expression_str = expression_str.to_string();
    
    let mut lexer = Lexer::new(expression_str);
    lexer.lexer().unwrap();
    
    let mut parser = Parser::new(lexer.get_tokens().clone(), Vec::new(), KsPath::new(), KsPath::new());
    parser.parse_expression().unwrap()
}


#[test]
fn test_interpreter_plus() {
    let expression = get_expression("5 + 2");

    let mut interpreter = Interpreter::empty();

    let test_value = ValueType::Integer(7);
    let value = interpreter.interpret_expression(expression).unwrap().get_type().clone();

    assert_eq!(value, test_value);
}


#[test]
fn test_interpreter_minus() {
    let mut interpreter = Interpreter::empty();

    let expression = Expression::BinaryOp {
        left: Box::new(Expression::IntegerLiteral(10)),
        operator: Operator::Minus,
        right: Box::new(Expression::IntegerLiteral(7))
    };

    let test_value = ValueType::Integer(3);
    let value = interpreter.interpret_expression(expression).unwrap().get_type().clone();

    assert_eq!(value, test_value);
}


#[test]
fn test_interpreter_complex() {
    let test_value = ValueType::Integer(15);
    
    let expression = get_expression("-10 + 22 + 3");

    let mut interpreter = Interpreter::empty();
    let value = interpreter.interpret_expression(expression).unwrap().get_type().clone();

    assert_eq!(value, test_value);
}


#[test]
fn test_interpreter_even_more_complex() {
    let test_value = ValueType::Float(117.5);
    
    let expression = get_expression("(22 + 3) / 10 + 5 * 25 - 10");

    let mut interpreter = Interpreter::empty();
    let value = interpreter.interpret_expression(expression).unwrap().get_type().clone();

    assert_eq!(value, test_value);
}

#[test]
fn test_interpreter_even_more_complex_2() {
    let test_value = ValueType::Float(-235.0);
    
    let expression = get_expression("((22 + 3) / 10 + 5 * 25 - 10) * -2");

    let mut interpreter = Interpreter::empty();
    let value = interpreter.interpret_expression(expression).unwrap().get_type().clone();

    assert_eq!(value, test_value);
}

#[test]
fn test_interpreter_string_error() {
    let err = io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!");
    
    let expression = get_expression("((22 + 3) / 10 + 5 * 25 - \"Hello World\") * -2");

    let mut interpreter = Interpreter::empty();
    let value = interpreter.interpret_expression(expression).unwrap_err();

    assert_eq!(value.to_string(), err.to_string());
}

#[test]
fn test_interpreter_add_strings() {
    let expression = get_expression("\"Hello\" + \" World\"");
    let test_value = ValueType::String(String::from("Hello World"));

    let mut interpreter = Interpreter::empty();

    let value = interpreter.interpret_expression(expression).unwrap().get_type().clone();

    assert_eq!(value, test_value);

}

#[test]
fn test_interpreter_boolean_false_1() {
    let expression = get_expression("22 == 33");
    let test_value = ValueType::Boolean(false);

    let mut interpreter = Interpreter::empty();
    let value = interpreter.interpret_expression(expression).unwrap().get_type().clone();

    assert_eq!(value, test_value)
}

#[test]
fn test_interpreter_boolean_true_1() {
    let expression = get_expression("\"Hello World\" == \"Hello World\"");
    let test_value = ValueType::Boolean(true);

    let mut interpreter = Interpreter::empty();
    let value = interpreter.interpret_expression(expression).unwrap().get_type().clone();

    assert_eq!(value, test_value)
}

#[test]
fn test_interpreter_boolean_true_2() {
    let expression = get_expression("\"Hello World\" == \"Hello World\" && 22 == 22 || 90 == 10");
    let test_value = ValueType::Boolean(true);

    let mut interpreter = Interpreter::empty();
    let value = interpreter.interpret_expression(expression).unwrap().get_type().clone();

    assert_eq!(value, test_value)
}

#[test]
fn test_interpreter_boolean_true_3() {
    let expression = get_expression("\"Hello World\" == \"Hello World\" && 22 == 22 && 90 == 10");
    let test_value = ValueType::Boolean(false);

    let mut interpreter = Interpreter::empty();
    let value = interpreter.interpret_expression(expression).unwrap().get_type().clone();

    assert_eq!(value, test_value)
}


// #[test]
// fn test_interpreter_var_dec_statement() {
//     let test_statement = get_statement("let a = 10 + 20;");
//     let test_hash = HashMap::from([
//         (String::from("a"), ValueType::Integer(30))
//     ]);

//     let mut interpreter = Interpreter::new(&NativeRegistry::new());
//     interpreter.interpret_statement(test_statement).unwrap();

//     let env = interpreter.get_local();
//     let env = env.borrow();
//     let values = env.get_values();
//     let references = env.get_references();

//     let reference = values.get("a").unwrap();
//     let value = references.get(reference).unwrap().clone();

//     assert_eq!(HashMap::from([(String::from("a"), value.get_type().clone())]), test_hash);
// }


