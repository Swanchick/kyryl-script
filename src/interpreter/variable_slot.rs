use super::value::Value;

#[derive(Debug, Clone)]
pub enum VariableSlot {
    Variable(Value),
    Reference(u128)
}