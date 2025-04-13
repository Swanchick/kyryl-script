use super::data_type::DataType;
use super::expression::Expression;


#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    VarableDeclaration {
        name: String,
        data_type: Option<DataType>,
        value: Option<Expression>
    },
    Assigment {
        name: String,
        value: Expression
    },
    AddValue {
        name: String,
        value: Expression
    },
    RemoveValue {
        name: String,
        value: Expression
    },
    // FunctionCall {
    //     name: String,
    //     parameters: Vec<Expression>
    // },
    ReturnStatement {
        value: Option<Expression>
    },
    IfStatement {
        condition: Expression,
        body: Vec<Statement>,
        else_body: Option<Vec<Statement>>
    },
    WhileStatement {
        condition: Expression,
        body: Vec<Statement>
    },
    Expression {
        value: Expression
    }
}
