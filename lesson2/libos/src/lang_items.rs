use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    sbi_rt::system_reset(sbi_rt::Shutdown, sbi_rt::NoReason);
    loop {}
}

