use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::sync::Once;


use crate::interpreter::enviroment::Environment;
use super::native_buffer::NativeBuffer;

use super::native_types::NativeTypes;

static INIT: Once = Once::new();
static mut NATIVE_REGISTRY: Option<Rc<RefCell<NativeRegistry>>> = None;

pub struct NativeRegistry {
    pub global: Option<Rc<RefCell<Environment>>>,
    pub local: Option<Rc<RefCell<Environment>>>,
    natives: HashMap<String, NativeTypes>
}

impl NativeRegistry {
    pub fn get() -> Rc<RefCell<NativeRegistry>> {
        unsafe {
            INIT.call_once(|| {
                NATIVE_REGISTRY = Some(NativeRegistry::new());
            });

            NATIVE_REGISTRY.as_ref().unwrap().clone()
        }
    }

    pub fn new() -> Rc<RefCell<NativeRegistry>> {
        Rc::new(RefCell::new(
            NativeRegistry { 
                global: None, 
                local: None,
                natives: HashMap::new()
            }
        ))
    }

    pub fn add_buffer(&mut self, buffer: NativeBuffer) {
        for (name, native) in buffer.get_table() {
            self.natives.insert(name.to_owned(), native.clone());
        }
    }

    pub fn get_natives(&self) -> &HashMap<String, NativeTypes> {
        &self.natives
    }

    pub fn get_native(&self, name: &str) -> Option<&NativeTypes> {
        self.natives.get(name)
    }

}
