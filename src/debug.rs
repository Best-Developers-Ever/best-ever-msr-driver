use alloc::string::String;

use winapi::km::wdm::DbgPrint;

#[doc(hidden)]
pub fn _debug_print(args: core::fmt::Arguments) {
    let s: String = alloc::format!("[msr_driver] {}\0", args);
    unsafe { DbgPrint(s.as_ptr()) };
}

#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => ($crate::debug::_debug_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! debug_println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::debug_print!("{}\n", format_args!($($arg)*)));
}