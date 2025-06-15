use std::fmt::Display;

use super::parameter::Parameter;


#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
    Int,
    Float,
    String,
    Bool,
    Void(Option<Box<DataType>>),
    List(Box<DataType>),
    Tuple(Vec<DataType>),
    RustFunction {
        return_type: Box<DataType>
    },
    Function {
        parameters: Vec<DataType>,
        return_type: Box<DataType>
    }
}


impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", DataType::display(self.clone()))
    }
}

impl DataType {
    pub fn display(data_type: DataType) -> String {
        match data_type {
            DataType::Int => format!("int"),
            DataType::Float => format!("float"),
            DataType::String => format!("string"),
            DataType::Bool => format!("boolean"),
            DataType::Void(_) => format!("void"),
            DataType::RustFunction{ return_type } => format!("rust_function( ... ) -> {:?}", return_type),
            DataType::List(data_type) => format!("list {:?}", data_type),
            DataType::Function{ parameters, return_type } => format!("function({:?}) -> {:?}", parameters, return_type),
            DataType::Tuple(types) => {
                let mut out = String::new();
                out.push('(');
                let len = types.len();

                for (i, data_type) in types.iter().enumerate() {
                    let type_string = DataType::display(data_type.clone());

                    out.push_str(type_string.as_str());

                    if i != len - 1 {
                        out.push_str(", ");
                    }
                }

                out
            }
        }
    }
    
    pub fn from_parameters(parameters: &Vec<Parameter>) -> Vec<DataType> {
        let mut out: Vec<DataType> = Vec::new();

        for parameter in parameters {
            out.push(parameter.data_type.clone());
        }

        out
    }

    pub fn void() -> DataType {
        DataType::Void(None)
    }

    pub fn is_void(data_type: &DataType) -> bool {
        match data_type {
            DataType::Void(_) => true,
            _ => false
        }
    }
}
