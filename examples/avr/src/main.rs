#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]
use panic_halt as _;

extern crate alloc;

use alloc::vec::Vec;

use avr_allocator::{AvrHeap, };
// use embeddedutil::{arduino_hal, avralloc::AvrHeap};
#[global_allocator]
static ALLOCATOR: AvrHeap = AvrHeap::empty();

#[arduino_hal::entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { ALLOCATOR.init(HEAP.as_ptr() as usize, HEAP_SIZE) }
    }

    let mut xs = Vec::new();
    xs.push(1);

    loop { /* .. */ }
}

// // use `#![feature(default_alloc_error_handler)]` or define custom error handler
// // like below
// #[alloc_error_handler]
// fn oom(_: Layout) -> ! {
//     loop {}
// }
