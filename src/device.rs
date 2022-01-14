use winapi::km::wdm::{DEVICE_OBJECT, IO_STACK_LOCATION, IoDeleteDevice, IRP, IRP_MJ};
use winapi::shared::ntdef::NTSTATUS;
use winapi::shared::ntstatus::{STATUS_NOT_IMPLEMENTED, STATUS_SUCCESS};

use crate::{debug_println, Error, READ_MSR_IO_CONTROL_CODE, WRITE_MSR_IO_CONTROL_CODE};
use crate::io::IoRequest;
use crate::msr::{ioctl_read_msr, ioctl_write_msr};

pub struct Device {
    raw: *mut DEVICE_OBJECT,
}

unsafe impl Send for Device {}

unsafe impl Sync for Device {}

impl Drop for Device {
    fn drop(&mut self) {
        self.close();
    }
}

impl Device {
    pub fn from_raw(raw: *mut DEVICE_OBJECT) -> Self {
        Self {
            raw,
        }
    }

    pub fn close(&self) {
        debug_println!("Dropping device");

        if !self.raw.is_null() {
            unsafe {
                IoDeleteDevice(self.raw);
            }
        }
    }
}

pub unsafe extern "system" fn dispatch_device(_device: &mut DEVICE_OBJECT, irp: &mut IRP) -> NTSTATUS {
    let request: IoRequest = IoRequest::from_raw(irp);
    let stack_location: &IO_STACK_LOCATION = request.stack_location();

    match stack_location.MajorFunction {
        irp_mj if irp_mj as u8 == IRP_MJ::CREATE as u8 => {
            request.complete(Ok(0));
            STATUS_SUCCESS
        }
        irp_mj if irp_mj as u8 == IRP_MJ::CLOSE as u8 => {
            request.complete(Ok(0));
            STATUS_SUCCESS
        }
        irp_mj if irp_mj as u8 == IRP_MJ::DEVICE_CONTROL as u8 => {
            match stack_location.Parameters.DeviceIoControl().IoControlCode as u32 {
                READ_MSR_IO_CONTROL_CODE => {
                    request.complete(Ok(ioctl_read_msr(irp.AssociatedIrp.SystemBuffer())));
                    STATUS_SUCCESS
                }
                WRITE_MSR_IO_CONTROL_CODE => {
                    request.complete(Ok(ioctl_write_msr(irp.AssociatedIrp.SystemBuffer())));
                    STATUS_SUCCESS
                }
                code => {
                    debug_println!("IRP_MJ not implemented: {}", code);
                    request.complete(Err(Error::NOT_IMPLEMENTED));
                    STATUS_NOT_IMPLEMENTED
                }
            }
        }
        code => {
            debug_println!("IRP_MJ not implemented: {}", code);
            request.complete(Err(Error::NOT_IMPLEMENTED));
            STATUS_NOT_IMPLEMENTED
        }
    }
}