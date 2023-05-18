#![no_std]

use drv_common::{drv_entry, Driver};

drv_entry!(drv0_init_fn);

fn drv0_init_fn() -> Driver<'static> {
    Driver::info("rtc", "google,goldfish-rtc")
}
