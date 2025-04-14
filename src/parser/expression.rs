use super::operator::Operator;

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    IntegerLiteral(i32),
    FloatLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    Identifier(String),
    FunctionCall(String, Vec<Expression>),
    ListLiteral(Vec<Expression>),

    IdentifierIndex {
        left: Box<Expression>,
        index: Box<Expression>
    },
    
    BinaryOp {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>
    },

    UnaryOp {
        expression: Box<Expression>,
        operator: Operator
    },
    FrontUnaryOp {
        expression: Box<Expression>,
        operator: Operator
    },
}