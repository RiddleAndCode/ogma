use crate::error::Fallible;
use alloc::alloc::{GlobalAlloc, Layout};
use core::panic::PanicInfo;
use core::ptr::null_mut;

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static A: MyAllocator = MyAllocator;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
fn handle_alloc_error(_: Layout) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn rust_eh_personality() {}

pub fn test_runner(_: &[&dyn Fn() -> Fallible<()>]) {
    // just compile for now
}
