use std::io;

use core::interpreter::value::{Value, ValueType};
use core::native_registry::native_registry::NativeRegistry;
use core::global::data_type::DataType;


pub fn ks_range(args: Vec<Value>) -> io::Result<Value> {
    if args.len() != 1 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Too many arguments!"));
    }
 
    let mut references: Vec<u64> = Vec::new();

    let arg = &args[0];
    if let ValueType::Integer(number) = arg.get_type() {
        let native = NativeRegistry::get();
        {
            let native = native.borrow();
            let local = &native.local;
            if let Some(local) = local {
                let mut local = local.borrow_mut();

                for i in 0..(number.clone()) {
                    let value = Value::new(None, ValueType::Integer(i));
                    let reference = local.create_value_without_name(value);

                    references.push(reference);
                }
            } 
        }
    }

    Ok(Value::new(None, ValueType::List { references, data_type: DataType::Int }))
}