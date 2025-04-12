#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
    Int,
    Float,
    String,
    Bool,
    Void,
    Struct,
    Enum,
    List,
    Tuple,
    Function
}