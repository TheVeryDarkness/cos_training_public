#![no_std]
#![no_main]

use core::ops::Range;

use drv_common::{CallEntry, Driver};
use mod_common::{entries, one};

// I've tried linkme, but it failed.
// Though I tried a lot of solutions online, all of them led to some errors.
// I've comsumed so much time.
// And I'm actually a green hand to Rust and OS!
// Please tell me how to implement it automatically the next time!
entries!(INIT_CALLS, Driver, drv0, drv1, drv2);
// static INIT_CALLS: [CallEntry; 3] = [drv0::ENTRY, drv1::ENTRY, drv2::ENTRY];

#[no_mangle]
fn main() {
    libos::init();

    libos::println!("\n[ArceOS Tutorial]: B1\n");
    verify();
}

fn traverse_drivers() {
    let range: Range<*const CallEntry> = INIT_CALLS.as_ptr_range();
    // Parse range of init_calls by calling C function.
    display_initcalls_range(range.start as usize, range.end as usize);

    // For each driver, display name & compatible
    INIT_CALLS.iter().for_each(|entry: &CallEntry| {
        let drv: Driver = (entry.init_fn)();
        display_drv_info(drv.name, drv.compatible);
    })
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
