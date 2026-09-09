use ks_core::kyryl_script::KyrylScript;
use ks_core::parser::data_type::DataType;
use ks_vm::VM;

use crate::ks_len::KsLen;
use crate::ks_print::{KsPrint, KsPrintln};
use crate::ks_range::KsRange;

mod ks_len;
mod ks_print;
mod ks_range;

pub fn ks_register_std(kyryl_script: &mut KyrylScript) {
    kyryl_script.compiler_mut().register_native("print", 0);
    kyryl_script.parser_mut().register_variable(
        "print",
        DataType::RustFunction {
            return_type: Box::new(DataType::void()),
        },
        true,
    );

    kyryl_script.compiler_mut().register_native("println", 1);
    kyryl_script.parser_mut().register_variable(
        "println",
        DataType::RustFunction {
            return_type: Box::new(DataType::void()),
        },
        true,
    );

    kyryl_script.compiler_mut().register_native("len", 2);
    kyryl_script.parser_mut().register_variable(
        "len",
        DataType::RustFunction {
            return_type: Box::new(DataType::Int),
        },
        true,
    );

    kyryl_script.compiler_mut().register_native("range", 3);
    kyryl_script.parser_mut().register_variable(
        "range",
        DataType::RustFunction {
            return_type: Box::new(DataType::List(Box::new(DataType::Int))),
        },
        true,
    );
}

pub fn vm_register_std(vm: &mut VM) {
    vm.add_native(Box::new(KsPrint));
    vm.add_native(Box::new(KsPrintln));
    vm.add_native(Box::new(KsLen));
    vm.add_native(Box::new(KsRange));
}
