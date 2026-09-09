use crate::{GVS, Runner};

pub struct NativeHelper<'a> {
    pub runner: &'a mut Runner,
    pub gvs: &'a mut GVS,
}

impl<'a> NativeHelper<'a> {
    pub fn new(runner: &'a mut Runner, gvs: &'a mut GVS) -> Self {
        Self { runner, gvs }
    }
}

// For now we give the whole access to the languge
//
// Todo:
// Make defined functions to safely control the language
