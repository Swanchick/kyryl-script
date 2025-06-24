use super::native_function::NativeFunction;

#[derive(Debug, Clone)]
pub enum NativeTypes {
    NativeFunction(NativeFunction)
}
