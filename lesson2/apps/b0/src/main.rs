#![no_std]
#![no_main]

use drv0 as _;
use drv1 as _;

use drv_common::CallEntry;

#[no_mangle]
fn main() {
    libos::init();

    libos::println!("\n[ArceOS Tutorial]: B0\n");
    verify();
}

/* Todo: Implement it */
fn traverse_drivers() {
    libos::println!("\n!!! Fix it !!!\n");
    // Parse range of init_calls by calling C function.
    // display_initcalls_range(range_start, range_end);

    // For each driver, display name & compatible
    // display_drv_info(drv.name, drv.compatible);
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
