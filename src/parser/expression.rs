use super::operator::Operator;

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    IntegerLiteral(i32),
    FloatLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    Identifier(String),
    FunctionCall(String, Vec<Expression>),
    
    BinaryOp {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>
    }
}