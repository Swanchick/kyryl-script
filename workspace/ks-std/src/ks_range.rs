use ks_vm::{INT_TYPE, KsCall, NativeHelper, VMError, VMResult, Variable, types::StorageId};

pub struct KsRange;

impl KsCall for KsRange {
    fn call<'a>(&mut self, arguments: usize, helper: NativeHelper<'a>) -> VMResult<()> {
        if arguments != 1 {
            return Err(VMError::from("Invalid arguments!"));
        }

        let gvs = helper.gvs;
        let runner = helper.runner;

        let variable = runner.acc.last(gvs)?.clone();

        if !(variable.is_primitive() && variable.value_type == INT_TYPE) {
            return Err(VMError::from("Invalid varaible type, required int"));
        }

        runner.acc.pop(gvs)?;

        let mut stack = Vec::<StorageId>::new();

        for index in 0..variable.value {
            let variable = Variable::from(index as i64);
            let storage_id = gvs.store(variable);
            stack.push(storage_id);
        }

        let stack_id = gvs.collection_store_stack(stack);
        let stack = Variable::collection(stack_id);

        runner.acc.push(gvs, stack)
    }
}
