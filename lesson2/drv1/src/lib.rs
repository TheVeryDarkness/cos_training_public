#![no_std]

use drv_common::{drv_entry, Driver};

drv_entry!(drv1_init_fn);

fn drv1_init_fn() -> Driver<'static> {
    Driver::info("uart", "ns16550a")
}
