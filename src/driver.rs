use winapi::km::wdm::{DEVICE_OBJECT, DEVICE_TYPE, DRIVER_OBJECT, IoCreateDevice};
use winapi::shared::ntdef::PUNICODE_STRING;

use crate::{debug_println, Device, Error};
use crate::error::IntoResult;
use crate::utils::create_unicode_string;

pub struct Driver {
    pub(crate) raw: *mut DRIVER_OBJECT,
}

impl Driver {
    pub fn from_raw(raw: *mut DRIVER_OBJECT) -> Self {
        Self {
            raw,
        }
    }

    pub fn create_device(&self,
                         name: &str,
                         device_type: DEVICE_TYPE,
                         device_flags: u32) -> Result<Device, Error> {
        let name_ptr: PUNICODE_STRING = &mut create_unicode_string(name);

        unsafe {
            debug_println!("Creating device: {:?}", ((*name_ptr).Buffer as *mut u16 as *mut [u16; 32]).read());

            let mut device: *mut DEVICE_OBJECT = core::ptr::null_mut();

            IoCreateDevice(
                self.raw,
                0,
                name_ptr,
                device_type,
                device_flags,
                0,
                &mut device)
                .into_result()?;

            Ok(Device::from_raw(device))
        }
    }
}

