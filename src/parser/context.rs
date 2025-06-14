use crate::parser::data_type::DataType;

pub enum Context {
    Function {
        return_data: DataType
    },
    None
}