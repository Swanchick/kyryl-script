use crate::parser::data_type::DataType;
use crate::parser::parameter::Parameter;
use crate::parser::statement::Statement;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Integer(i32),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    List {
        references: Vec<u64>,
        data_type: DataType
    },
    Tuple {
        references: Vec<u64>,
        data_types: DataType
    },
    Function {
        name: String,
        return_type: DataType,
        parameters: Vec<Parameter>,
        body: Vec<Statement>
    },
}


#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    reference: Option<u64>,
    value_type: ValueType
}

impl Value {
    pub fn new(reference: Option<u64>, value_type: ValueType) -> Value {
        Value {
            reference,
            value_type
        }
    }

    pub fn set_reference(&mut self, reference: u64) {
        self.reference = Some(reference);
    }

    pub fn get_reference(&self) -> Option<u64> {
        self.reference
    }

    pub fn get_type(&self) -> &ValueType {
        &self.value_type
    }

    pub fn get_type_mut(&mut self) -> &mut ValueType {
        &mut self.value_type
    }

    pub fn clear_reference(&mut self) {
        self.reference = None;
    }

    pub fn get_data_type(&self) -> DataType {
        self.value_type.get_data_type()
    }
}

impl ValueType {
    pub fn get_data_type(&self) -> DataType {
        match self {
            ValueType::Integer(_) => DataType::Int,
            ValueType::Float(_) => DataType::Float,
            ValueType::String(_) => DataType::String,
            ValueType::Boolean(_) => DataType::Bool,
            ValueType::Function { name: _, return_type, parameters, body: _ } => {
                let mut parameter_types: Vec<DataType> = Vec::new();

                for parameter in parameters {
                    parameter_types.push(parameter.data_type.clone());
                }

                DataType::Function { parameters: parameter_types, return_type: Box::new(return_type.clone()) }
            },
            ValueType::List { references: _, data_type } => DataType::List(Box::new(data_type.clone())),
            ValueType::Null => DataType::void(),
            ValueType::Tuple { references: _, data_types } => data_types.clone()
        }
    }
}