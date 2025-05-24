use std::collections::HashMap;

use super::rust_function::RustFunction;


pub struct NativeRegistry {
    rust_functions: HashMap<String, RustFunction>
}

impl NativeRegistry {
    pub fn new() -> NativeRegistry {
        NativeRegistry { rust_functions: HashMap::new() }
    }

    pub fn register_function(&mut self, name: &str, rust_function: RustFunction) {
        self.rust_functions.insert(name.to_string(), rust_function);
    }

    pub fn get_rust_function(&self, name: &str) -> Option<&RustFunction> {
        self.rust_functions.get(name)
    }

    pub fn get(&self) -> &HashMap<String, RustFunction> {
        &self.rust_functions
    }
}
