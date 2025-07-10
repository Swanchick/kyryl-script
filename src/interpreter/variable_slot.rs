use super::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum VariableSlot {
    Variable(Value),
    Reference(u64)
}