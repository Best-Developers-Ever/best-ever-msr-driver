use alloc::vec::Vec;

use winapi::shared::ntdef::{PWCH, UNICODE_STRING};

pub fn create_unicode_string(rust_str: &str) -> UNICODE_STRING {
    let result: Vec<u16> = rust_str.encode_utf16().collect::<Vec<u16>>();

    let len: usize = result.len();

    let n: usize = if len > 0 && result[len - 1] == 0 { len - 1 } else { len };

    UNICODE_STRING {
        Length: (n * 2) as u16,
        MaximumLength: (len * 2) as u16,
        Buffer: result.as_ptr() as PWCH,
    }
}