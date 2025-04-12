use std::mem::discriminant;
use std::rc::Rc;
use std::cell::RefCell;

use crate::parser::function::Function;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i32),
    Float(f64),
    String(String),
    Boolean(bool),
    Function(Rc<RefCell<Function>>),
    Void
}
