use crate::global::data_type::DataType;

use super::expression::Expression;
use super::parameter::Parameter;


#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    VariableDeclaration {
        name: String,
        public: bool,
        data_type: Option<DataType>,
        value: Option<Expression>
    },
    Assigment {
        name: String,
        value: Expression
    },
    AssigmentIndex {
        name: String,
        index: Vec<Expression>,
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
    ForLoopStatement {
        name: String,
        list: Expression,
        body: Vec<Statement>
    },
    Expression {
        value: Expression
    },
    Function {
        name: String,
        public: bool,
        return_type: DataType,
        parameters: Vec<Parameter>,
        body: Vec<Statement>
    },
    EarlyReturn {
        name: String,
        body: Option<Vec<Statement>>
    },
    Use {
        file_name: String,
        body: Vec<Statement>
    }
}
