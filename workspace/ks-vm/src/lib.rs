#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod environment;
mod native;
pub mod runner;
pub mod types;
mod utils;
mod vm;
mod vm_helper;

pub use environment::variable::{
    BOOLEAN_TYPE, FLOAT_TYPE, FUNCTION_TYPE, INT_TYPE, NULL_TYPE, STACK_TYPE, STRING_TYPE,
};
pub use environment::{Collection, Function, GVS, Stack, Variable};
pub use native::{KsCall, NativeCall, NativeHelper, NativeRegistry};
pub use runner::Runner;
pub use runner::assign::Assign;
pub use runner::call_stack::CallStack;
pub use runner::data_size::DataSize32;
pub use vm::VM;
pub use vm_helper::VMHelper;

pub use types::VMResult;
pub use utils::VMError;
