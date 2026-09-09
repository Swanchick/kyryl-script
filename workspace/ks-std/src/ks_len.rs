use ks_vm::{Collection, KsCall, NativeHelper, VMError, VMResult, Variable};

pub struct KsLen;

impl KsCall for KsLen {
    fn call<'a>(&mut self, arguments: usize, helper: NativeHelper<'a>) -> VMResult<()> {
        if arguments != 1 {
            return Err(VMError::from("Invalid arguments!"));
        }

        let gvs = helper.gvs;
        let runner = helper.runner;

        let variable = runner.acc.last(gvs)?.clone();

        if !(variable.is_stack() || variable.is_string()) {
            return Err(VMError::from(
                "Invalid varaible type, required string or stack",
            ));
        }

        let collection = gvs
            .collections
            .get(variable.value as usize)
            .ok_or("Cannot find collection")?;

        let len = match collection {
            Collection::String(string) => Ok(string.len()),
            Collection::Stack(stack) => Ok(stack.len()),
            Collection::Free => Err(VMError::from("The collection is freed")),
        }? as i64;

        runner.acc.pop(gvs)?;

        let len = Variable::from(len);
        runner.acc.push(gvs, len)
    }
}
