use core::alloc::{GlobalAlloc, Layout};

use winapi::km::wdm::POOL_TYPE;

use crate::{debug_println, version_info};

const WINDOWS_2004_BUILD: u32 = 19041u32;

const POOL_FLAG_NON_PAGED: u64 = 0x40;
const POOL_FLAG_USE_QUOTA: u64 = 0x01;

pub const POOL_CONFIGURATION: u64 = POOL_FLAG_NON_PAGED | POOL_FLAG_USE_QUOTA;

#[link(name = "ntoskrnl")]
extern "system" {
    pub fn ExAllocatePool2(flags: u64, number_of_bytes: usize, tag: u32) -> *mut u8;

    pub fn ExAllocatePoolWithTag(flags: POOL_TYPE, number_of_bytes: usize, tag: u32) -> *mut u8;

    pub fn ExFreePoolWithTag(pointer: *mut u8, tag: u32);
}

pub const KMRS_TAG: u32 = 0x4B4D5253; // 'KMRS'

pub struct MutableAllocator {
    allocation_function: fn(usize) -> *mut u8,
}

impl MutableAllocator {
    pub fn change_allocator(mut self, new_allocation_function: fn(usize) -> *mut u8) {
        self.allocation_function = new_allocation_function;
    }

    pub fn allocate(&self, layout_size: usize) -> *mut u8 {
        (self.allocation_function)(layout_size)
    }
}

pub struct KernelAllocator {
    allocator_provider: fn() -> MutableAllocator,
}

impl KernelAllocator {
    pub const fn new() -> Self {
        Self { allocator_provider: || MutableAllocator { allocation_function: |layout_size: usize| -> *mut u8 { unsafe { ExAllocatePoolWithTag(POOL_TYPE::NonPagedPool, layout_size, KMRS_TAG) } } } }
    }

    pub fn change_allocator(&self, new_allocation_function: fn(usize) -> *mut u8) {
        (self.allocator_provider)().change_allocator(new_allocation_function);
    }
}

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pool: *mut u8 = (self.allocator_provider)().allocate(layout.size());

        if pool.is_null() {
            panic!("[kernel-allocator] Failed to allocate pool");
        }

        pool
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        ExFreePoolWithTag(ptr, KMRS_TAG);
    }
}

#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    panic!("[kernel-allocator] Memory allocation error: {:?}", layout);
}

pub fn auto_select_allocator(allocator: &KernelAllocator) {
    unsafe {
        match version_info() {
            Some(version_info) if version_info.dwBuildNumber as u32 >= WINDOWS_2004_BUILD => {
                debug_println!("Windows build is 2004 or newer, using ExAllocatePool2");
                allocator.change_allocator(|layout_size: usize| ExAllocatePool2(POOL_CONFIGURATION, layout_size, KMRS_TAG))
            }
            _ => debug_println!("Windows build is older than 2004, using ExAllocatePoolWithTag")
        }
    }
}