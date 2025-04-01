pub enum Expression {
    IntegerLiteral(i32),
    FloatLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    Identifier(String),
    FunctionCall(String, Vec<Expression>)
}