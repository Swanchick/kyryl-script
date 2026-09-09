use std::env::args;

use ks_core::kyryl_script::KyrylScript;
use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;
use ks_std::{ks_register_std, vm_register_std};
use ks_vm::VM;

fn main() -> KsResult<()> {
    let args: Vec<String> = args().collect();
    let path = args.get(1);

    if let Some(path) = path {
        let mut kyryl_script = KyrylScript::new();
        ks_register_std(&mut kyryl_script);

        let program = kyryl_script.compile_from_file_new(path)?;

        let mut vm = VM::from(program.as_bytes());
        vm_register_std(&mut vm);

        vm.init();
        while !vm.is_empty() {
            vm.step().or_else(|e| Err(KsError::runtime(&e.message)))?;
        }
    }

    Ok(())
}
