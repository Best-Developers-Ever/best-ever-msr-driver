use alloc::vec::Vec;
use core::mem;

use winapi::km::wdm::{OSVERSIONINFOW, RtlGetVersion};
use winapi::shared::ntdef::{PWCH, UNICODE_STRING};
use winapi::shared::ntstatus::STATUS_SUCCESS;

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

pub unsafe fn version_info() -> Option<OSVERSIONINFOW> {
    let mut rtl_os_version: OSVERSIONINFOW = mem::zeroed();

    let get_version_status = RtlGetVersion(&mut rtl_os_version);

    if get_version_status == STATUS_SUCCESS {
        Some(rtl_os_version)
    } else {
        None
    }
}