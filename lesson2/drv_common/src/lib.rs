#![no_std]

pub struct Driver<'a> {
    pub name: &'a str,
    pub compatible: &'a str,
}

impl Driver<'_> {
    pub fn info<'a>(name: &'a str, compatible: &'a str) -> Driver<'a> {
        Driver {
            name,
            compatible,
        }
    }
}

type InitFn = fn() -> Driver<'static>;

pub struct CallEntry {
    pub init_fn: InitFn,
}
