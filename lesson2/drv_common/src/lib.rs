#![no_std]

use mod_common::Module;

#[macro_export]
macro_rules! drv_entry {
    ($init_fn:expr) => {
        mod_common::entry! ($init_fn, drv_common::CallEntry);
    };
}

pub struct Driver<'a> {
    pub name: &'a str,
    pub compatible: &'a str,
}

impl Driver<'_> {
    pub fn info<'a>(name: &'a str, compatible: &'a str) -> Driver<'a> {
        Driver { name, compatible }
    }
}

impl Module for Driver<'_> {
    type Entry = CallEntry;
}

type InitFn = fn() -> Driver<'static>;

pub struct CallEntry {
    pub init_fn: InitFn,
}
