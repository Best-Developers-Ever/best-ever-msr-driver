use winapi::km::wdm::{IO_STACK_LOCATION, IoCompleteRequest, IoGetCurrentIrpStackLocation, IRP};
use winapi::km::wdm::IO_PRIORITY::IO_NO_INCREMENT;
use winapi::shared::ntdef::NTSTATUS;
use winapi::shared::ntstatus::STATUS_SUCCESS;

use crate::Error;

pub struct IoRequest {
    irp: *mut IRP,
}

impl IoRequest {
    pub fn from_raw(irp: *mut IRP) -> Self {
        Self { irp }
    }

    pub fn irp_mut(&self) -> &mut IRP {
        unsafe { &mut *self.irp }
    }

    pub fn stack_location(&self) -> &IO_STACK_LOCATION {
        unsafe { &*IoGetCurrentIrpStackLocation(self.irp_mut()) }
    }

    pub fn complete(&self, value: Result<usize, Error>) {
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