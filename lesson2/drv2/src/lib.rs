#![no_std]

use drv_common::Driver;
use mod_common::module;

// It should not be difficult to ellide the requirement to specify the lifetime.
module!(
    type: Driver<'static>,
    name: "gpio-keys",
    compatible: "gpio-keys"
);
