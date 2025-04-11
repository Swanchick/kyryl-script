use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::parser::expression::Expression;
use crate::parser::operator::Operator;


#[test]
fn test_interpreter_plus() {
    let interpreter = Interpreter::new();

    let expression = Expression::BinaryOp {
        left: Box::new(Expression::IntegerLiteral(5)),
        operator: Operator::Plus,
        right: Box::new(Expression::IntegerLiteral(2))
    };

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
