use std::io;

use core::interpreter::value::{Value, ValueType};
use core::native_registry::native_registry::NativeRegistry;

pub fn ks_local(args: Vec<Value>) -> io::Result<Value> {
    if args.len() > 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Too many arguments!"));
    }

    let native = NativeRegistry::get();
    {
        let native = native.borrow();
        let local = &native.local;
        if let Some(local) = local {
            let local = local.borrow();

            local.display_references();
        } 
    }

    Ok(Value::new(None, ValueType::Null))
}