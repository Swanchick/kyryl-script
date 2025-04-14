use std::fmt::Display;


#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
    Int,
    Float,
    String,
    Bool,
    Void,
    List(Box<DataType>),
    Function
}


impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Int => write!(f, "int"),
            DataType::Float => write!(f, "float"),
            DataType::String => write!(f, "string"),
            DataType::Bool => write!(f, "boolean"),
            DataType::Void => write!(f, "void"),
            DataType::List(data_type) => write!(f, "list {}", *data_type),
            DataType::Function => write!(f, "function")
        }
    }
}
