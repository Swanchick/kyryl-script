#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::{GVS, NativeCall};

pub struct VMHelper<'a> {
    pub instruction: u8,
    pub gvs: &'a mut GVS,
    pub native_call: &'a mut Option<NativeCall>,
    pub instructions: &'a [u8],
    pub runner_id: usize,
}
