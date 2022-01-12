pub use crate::driver::Driver;
pub use crate::error::Error;

#[macro_export]
macro_rules! kernel_module {
    ($module:ty) => {

        use widestring::U16CString;
        use winapi::shared::ntdef::UNICODE_STRING;
        use crate::device::dispatch_device;

        static mut __MOD: Option<$module> = None;

        #[no_mangle]
        pub extern "system" fn driver_entry(driver: &mut DRIVER_OBJECT, registry_path: &UNICODE_STRING) -> NTSTATUS {
            let irp_functions: [usize; 3] = [
                IRP_MJ::CREATE as usize,
                IRP_MJ::CLOSE as usize,
                IRP_MJ::DEVICE_CONTROL as usize
            ];

            for function_code in irp_functions {
                driver.MajorFunction[function_code] = Some(dispatch_device);
            }

            driver.DriverUnload = Some(driver_exit);

            let driver = Driver::from_raw(driver);

            let registry_path = unsafe { U16CString::from_ptr_str(registry_path.Buffer) };
            let registry_path = registry_path.to_string_lossy();

            match <$module as KernelModule>::init(driver, registry_path.as_str()) {
                Ok(m) => {
                    unsafe {
                        __MOD = Some(m);
                    }

                    STATUS_SUCCESS
                }
                Err(e) => {
                    e.to_nt_status()
                }
            }
        }

        pub extern "system" fn driver_exit(driver_object: &mut DRIVER_OBJECT) {
            let driver = Driver::from_raw(driver_object);

            unsafe {
                match __MOD.take() {
                    Some(m) => m.cleanup(driver),
                    _ => (),
                }
            }
        }
    };
}

pub trait KernelModule: Sized + Sync {
    fn init(driver: Driver, registry_path: &str) -> Result<Self, Error>;
    fn cleanup(&self, _driver: Driver);
}
