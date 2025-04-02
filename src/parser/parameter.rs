use super::data_type::DataType;


#[derive(PartialEq, Debug)]
pub struct Parameter {
    pub name: String,
    pub data_type: DataType,
}