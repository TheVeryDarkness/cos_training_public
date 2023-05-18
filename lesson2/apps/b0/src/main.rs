#![no_std]
#![no_main]

use core::slice::from_raw_parts;

use drv0 as _;
use drv1 as _;

use drv_common::{CallEntry, Driver};

#[no_mangle]
fn main() {
    libos::init();

    libos::println!("\n[ArceOS Tutorial]: B0\n");
    verify();
}

/* Todo: Implement it */
fn traverse_drivers() {
    extern "C" {
        fn initcalls_start() -> *const CallEntry;
        fn initcalls_end() -> *const CallEntry;
    }
    unsafe {
        let range_start = initcalls_start();
        let range_end = initcalls_end();
        // Parse range of init_calls by calling C function.
        display_initcalls_range(range_start as usize, range_end as usize);

        let entries: &[CallEntry] =
            from_raw_parts(range_start, range_end.offset_from(range_start) as usize);
        // For each driver, display name & compatible
        entries.iter().for_each(|entry: &CallEntry| {
            let drv: Driver = (entry.init_fn)();
            display_drv_info(drv.name, drv.compatible);
        })
    }
}

fn display_initcalls_range(start: usize, end: usize) {
    libos::println!("init calls range: 0x{:X} ~ 0x{:X}\n", start, end);
}

fn display_drv_info(name: &str, compatible: &str) {
    libos::println!("Found driver '{}': compatible '{}'", name, compatible);
}

fn verify() {
    traverse_drivers();

    libos::println!("\nResult: Okay!");
}
