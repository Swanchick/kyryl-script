#[repr(u8)]
#[derive(Debug)]
pub enum RunnerStatus {
    None,
    OutOfProgram,
    OutOfCallStacks,
}
