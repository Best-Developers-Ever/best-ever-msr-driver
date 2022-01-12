use winapi::km::wdm::{DEVICE_OBJECT, IO_STACK_LOCATION, IoCompleteRequest, IoDeleteDevice, IoGetCurrentIrpStackLocation, IRP, IRP_MJ};
use winapi::km::wdm::IO_PRIORITY::IO_NO_INCREMENT;
use winapi::shared::ntdef::NTSTATUS;
use winapi::shared::ntstatus::{STATUS_NOT_IMPLEMENTED, STATUS_SUCCESS};

use crate::{debug_println, Error, READ_MSR_IO_CONTROL_CODE, WRITE_MSR_IO_CONTROL_CODE};
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

pub struct IoRequest {
    irp: *mut IRP,
}

impl IoRequest {
    fn from_raw(irp: *mut IRP) -> Self {
        Self { irp }
    }

    fn irp_mut(&self) -> &mut IRP {
        unsafe { &mut *self.irp }
    }

    fn stack_location(&self) -> &IO_STACK_LOCATION {
        unsafe { &*IoGetCurrentIrpStackLocation(self.irp_mut()) }
    }

    fn complete(&self, value: Result<usize, Error>) {
        let irp: &mut IRP = self.irp_mut();

        match value {
            Ok(code) => unsafe {
                irp.IoStatus.Information = code;
                (irp.IoStatus.__bindgen_anon_1.Status_mut() as *mut NTSTATUS).write(STATUS_SUCCESS);
            }
            Err(error) => unsafe {
                irp.IoStatus.Information = 0;
                (irp.IoStatus.__bindgen_anon_1.Status_mut() as *mut NTSTATUS).write(error.to_nt_status());
            }
        }

        unsafe {
            IoCompleteRequest(irp as *mut IRP, IO_NO_INCREMENT);
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