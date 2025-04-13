#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    EqualEqual,
    GreaterEqual,
    Greater,
    LessEqual,
    Less,
    NotEqual,
    And,
    Or,
    Tilde,
    PlusPlus,
    MinusMinus
}