#![feature(allocator)]
#![feature(const_fn)]
#![allocator]
#![no_std]
#![crate_type="rlib"]
#![crate_name="my_allocator"]

extern crate linked_list_allocator;

use linked_list_allocator::Heap;
use core::ptr;
use core::cmp::min;

static mut HEAP: Heap = Heap::empty();

pub unsafe fn init() {
    extern {
        static __heap_start__: u32;
        static __heap_end__: u32;
    }
    HEAP = Heap::new(
        &__heap_start__ as *const u32 as usize,
        (&__heap_end__ as *const u32 as usize) - (&__heap_start__ as *const u32 as usize));
}

#[no_mangle]
pub extern fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    unsafe {
        HEAP.allocate_first_fit(size, align).unwrap_or(ptr::null_mut())
    }
}

#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, old_size: usize, align: usize) {
    unsafe {
        HEAP.deallocate(ptr, old_size, align);
    }
}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, old_size: usize, size: usize, align: usize) -> *mut u8 {
    unsafe {
        if let Some(dst) = HEAP.allocate_first_fit(size, align) {
            ptr::copy_nonoverlapping(ptr, dst, min(old_size, size));
            HEAP.deallocate(ptr, old_size, align);
            dst
        } else {
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize, _size: usize, _align: usize) -> usize {
    old_size
}

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
}
