use core::mem::size_of;

use winapi::shared::ntdef::PVOID;

#[repr(C)]
pub struct RegisterValue {
    register: u32,
    value: [u32; 2],
}

pub fn read_msr(register: u32) -> u64 {
    let (high, low): (u32, u32);

    unsafe {
        asm!("rdmsr", in("ecx") register, out("eax") low, out("edx") high, options(nomem, nostack, preserves_flags));
    }

    ((high as u64) << 32) | (low as u64)
}

pub fn write_msr(register: u32, value: [u32; 2]) {
    let low: u32 = value[0];
    let high: u32 = value[1];

    unsafe {
        asm!("wrmsr", in("ecx") register, in("eax") low, in("edx") high, options(nostack, preserves_flags));
    }
}

pub unsafe fn ioctl_read_msr(buffer: &PVOID) -> usize {
    let result: u64 = read_msr(((*buffer) as *mut u32).read());
    ((*buffer) as *mut u64).write(result);
    size_of::<u64>()
}

pub unsafe fn ioctl_write_msr(buffer: &PVOID) -> usize {
    let write_request: RegisterValue = ((*buffer) as *mut RegisterValue).read();
    write_msr(write_request.register, write_request.value);
    0
}