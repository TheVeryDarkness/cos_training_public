use core::fmt;
use core::fmt::Write;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::stdio::print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: fmt::Arguments) {
    HeaplessSBIWrite.write_fmt(args).unwrap();
}

pub struct HeaplessSBIWrite;

impl core::fmt::Write for HeaplessSBIWrite {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.as_bytes() {
            putchar(*ch as usize);
        }
        Ok(())
    }
}

/// Writes a byte to the console.
pub fn putchar(c: usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}

pub fn puts(s: &str) {
    for c in s.chars() {
        putchar(c as usize);
    }
}
