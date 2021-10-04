#![feature(allocator)]
#![feature(const_fn)]
#![allocator]
#![no_std]
#![crate_type="rlib"]
#![crate_name="my_allocator"]

extern crate linked_list_allocator;

use linked_list_allocator::Heap;
use core::intrinsics::copy_nonoverlapping;
use core::ptr::null_mut;
use core::cmp::min;

static mut HEAP: Heap = Heap::empty();

pub fn init() {
    extern {
        static __heap_start__: u8;
        static __heap_end__: u8;
    }
    unsafe {
        HEAP.init(
            &__heap_start__ as *const u8 as usize,
            &__heap_end__ as *const u8 as usize - &__heap_start__ as *const u8 as usize);
    }
}

#[no_mangle]
pub extern fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    unsafe {
        if let Some(ptr) = HEAP.allocate_first_fit(size, align) {
            ptr
        } else {
            null_mut()
        }
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
        if let Some(new_ptr) = HEAP.allocate_first_fit(size, align) {
            copy_nonoverlapping(ptr, new_ptr, min(old_size, size));
            HEAP.deallocate(ptr, old_size, align);
            new_ptr
        } else {
            null_mut()
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
