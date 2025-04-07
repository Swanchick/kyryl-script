use super::data_type::DataType;
use super::expression::Expression;
use super::parameter::Parameter;


#[derive(PartialEq, Debug)]
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
    FunctionDeclaration {
        name: String,
        parameters: Vec<Parameter>,
        return_type: Option<DataType>,
        body: Vec<Statement>
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
    }
}
