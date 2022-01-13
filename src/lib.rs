#![no_std]
#![feature(alloc_error_handler)]
#![feature(asm)]
#![feature(set_ptr_value)]
#![feature(const_fn_fn_ptr_basics)]

extern crate alloc;

use core::panic::PanicInfo;

use winapi::km::wdm::{DEVICE_TYPE, DRIVER_OBJECT, IRP_MJ};
use winapi::km::wdm::DEVICE_TYPE::FILE_DEVICE_UNKNOWN;
use winapi::shared::ntdef::NTSTATUS;
use winapi::shared::ntstatus::STATUS_SUCCESS;

use kernel_allocator::KernelAllocator;

use crate::device::Device;
use crate::driver::Driver;
use crate::error::Error;
use crate::kernel_allocator::auto_select_allocator;
use crate::kernel_module::KernelModule;
use crate::symbolic_link::SymbolicLink;
use crate::utils::{create_unicode_string, version_info};

mod kernel_allocator;
mod utils;
mod error;
mod driver;
mod msr;
mod kernel_module;
mod device;
mod symbolic_link;
mod debug;

#[no_mangle]
pub extern "system" fn __CxxFrameHandler3() -> i32 {
    0
}

#[export_name = "_fltused"]
static _FLOAT_USED: i32 = 0;

#[global_allocator]
static GLOBAL_ALLOCATOR: KernelAllocator = KernelAllocator::new();

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const FILE_DEVICE_SECURE_OPEN: u32 = 0x0100;

const DEVICE_NAME: &'static str = "\\Device\\BestEverMsrDriver\0";
const DOS_DEVICE_NAME: &'static str = "\\DosDevices\\BestEverMsrDriver\0";

const IO_CONTROL_DEVICE_TYPE: DEVICE_TYPE = FILE_DEVICE_UNKNOWN;
// const IO_CONTROL_DEVICE_TYPE: u32 = 40001;

const READ_MSR_IO_CONTROL_CODE: u32 = ((IO_CONTROL_DEVICE_TYPE as i32 as u32) << 16) | (0u32 << 14) | (0x821 << 2) | 0u32;
const WRITE_MSR_IO_CONTROL_CODE: u32 = ((IO_CONTROL_DEVICE_TYPE as i32 as u32) << 16) | (0u32 << 14) | (0x822 << 2) | 0u32;

struct Module<'a> {
    _device: Device,
    _symbolic_link: SymbolicLink<'a>,
}

impl<'a> KernelModule for Module<'a> {
    fn init(driver: Driver, _registry_path: &str) -> Result<Self, Error> {
        auto_select_allocator(&GLOBAL_ALLOCATOR);

        debug_println!("Init device");

        let device: Device = driver.create_device(
            DEVICE_NAME,
            IO_CONTROL_DEVICE_TYPE,
            FILE_DEVICE_SECURE_OPEN,
        )?;

        let symbolic_link: SymbolicLink = SymbolicLink::new(DOS_DEVICE_NAME, DEVICE_NAME)?;

        Ok(Module {
            _device: device,
            _symbolic_link: symbolic_link,
        })
    }

    fn cleanup(&self, _driver: Driver) {
        debug_println!("Bye bye!");
    }
}

kernel_module!(Module);
