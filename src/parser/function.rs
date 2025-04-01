use super::data_type::DataType;
use super::parameter::Parameter;
use super::statement::Statement;


pub struct Function {
    pub name: String,
    pub return_type: DataType,
    pub parameters: Vec<Parameter>,
    pub body: Vec<Statement>,
}