use std::fmt::Display;

use super::parameter::Parameter;


#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
    Int,
    Float,
    String,
    Bool,
    Void,
    RustFunction,
    List(Box<DataType>),
    Function {
        parameters: Vec<DataType>,
        return_type: Box<DataType>
    }
}


impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Int => write!(f, "int"),
            DataType::Float => write!(f, "float"),
            DataType::String => write!(f, "string"),
            DataType::Bool => write!(f, "boolean"),
            DataType::Void => write!(f, "void"),
            DataType::RustFunction => write!(f, "rust_function"),
            DataType::List(data_type) => write!(f, "list {:?}", data_type),
            DataType::Function{ parameters, return_type } => write!(f, "function({:?}) -> {:?}", parameters, return_type)
        }
    }
}

impl DataType {
    pub fn from_parameters(parameters: &Vec<Parameter>) -> Vec<DataType> {
        let mut out: Vec<DataType> = Vec::new();

        for parameter in parameters {
            out.push(parameter.data_type.clone());
        }

        out
    }
}
