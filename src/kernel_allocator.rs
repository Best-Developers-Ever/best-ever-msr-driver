use core::alloc::{GlobalAlloc, Layout};

use winapi::km::wdm::POOL_TYPE;
use winapi::km::wdm::POOL_TYPE::NonPagedPool;

// const POOL_FLAG_NON_PAGED: u64 = 0x40;
// const POOL_FLAG_USE_QUOTA: u64 = 0x01;
//
// const POOL_CONFIGURATION: u64 = POOL_FLAG_NON_PAGED | POOL_FLAG_USE_QUOTA;

#[link(name = "ntoskrnl")]
extern "system" {
    // pub fn ExAllocatePool2(flags: u64, number_of_bytes: usize, tag: u32) -> *mut u8;

    #[deprecated(since = "Windows 10 version 2004", note = "There is no acceptable replacement")]
    pub fn ExAllocatePoolWithTag(flags: POOL_TYPE, number_of_bytes: usize, tag: u32) -> *mut u8;

    pub fn ExFreePoolWithTag(pointer: *mut u8, tag: u32);
}

const KMRS_TAG: u32 = 0x4B4D5253; // 'KMRS'

pub struct KernelAllocator;

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pool: *mut u8 = ExAllocatePoolWithTag(NonPagedPool, layout.size(), KMRS_TAG);

        // WARNING: DON'T USE THIS, IT CAUSES OVERLAPPING ON POINTERS
        // let pool: *mut u8 = ExAllocatePool2(POOL_CONFIGURATION, layout.size(), KMRS_TAG);

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