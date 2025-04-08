use std::{cell::RefCell, rc::Rc};

use crate::parser::function::Function;

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i32),
    Float(f64),
    String(String),
    Boolean(bool),
    Function(Rc<RefCell<Function>>),
    Void
}