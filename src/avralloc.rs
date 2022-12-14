use avr_device::interrupt::Mutex;
use core::alloc::{GlobalAlloc, Layout,};
use core::cell::RefCell;
use core::ptr::{self, NonNull};
// use cortex_m::interrupt::Mutex;
use linked_list_allocator::Heap;

pub struct AvrHeap {
    heap: Mutex<RefCell<Heap>>,
}

impl AvrHeap {
    /// Crate a new UNINITIALIZED heap allocator
    ///
    /// You must initialize this heap using the
    /// [`init`](struct.AvrHeap.html#method.init) method before using the allocator.
    pub const fn empty() -> AvrHeap {
        AvrHeap {
            heap: Mutex::new(RefCell::new(Heap::empty())),
        }
    }

    /// Initializes the heap
    ///
    /// This function must be called BEFORE you run any code that makes use of the
    /// allocator.
    ///
    /// `start_addr` is the address where the heap will be located.
    ///
    /// `size` is the size of the heap in bytes.
    ///
    /// Note that:
    ///
    /// - The heap grows "upwards", towards larger addresses. Thus `end_addr` must
    ///   be larger than `start_addr`
    ///
    /// - The size of the heap is `(end_addr as usize) - (start_addr as usize)`. The
    ///   allocator won't use the byte at `end_addr`.
    ///
    /// # Safety
    ///
    /// Obey these or Bad Stuff will happen.
    ///
    /// - This function must be called exactly ONCE.
    /// - `size > 0`
    pub unsafe fn init(&self, start_addr: usize, size: usize) {
        avr_device::interrupt::free(
            // cortex_m::interrupt::free(
            |cs| {
                self.heap.borrow(cs).borrow_mut().init(start_addr, size);
            },
        );
    }

    /// Returns an estimate of the amount of bytes in use.
    pub fn used(&self) -> usize {
        avr_device::interrupt::free(
            // cortex_m::interrupt::free(
            |cs| self.heap.borrow(cs).borrow_mut().used(),
        )
    }

    /// Returns an estimate of the amount of bytes available.
    pub fn free(&self) -> usize {
        avr_device::interrupt::free(
            // cortex_m::interrupt::free(
            |cs| self.heap.borrow(cs).borrow_mut().free(),
        )
    }
}

unsafe impl GlobalAlloc for AvrHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        avr_device::interrupt::free(
            // cortex_m::interrupt::free(
            |cs| {
                self.heap
                    .borrow(cs)
                    .borrow_mut()
                    .allocate_first_fit(layout)
                    .ok()
                    .map_or(ptr::null_mut(), |allocation| allocation.as_ptr())
            },
        )
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        avr_device::interrupt::free(
            // cortex_m::interrupt::free(
            |cs| {
                self.heap
                    .borrow(cs)
                    .borrow_mut()
                    .deallocate(NonNull::new_unchecked(ptr), layout)
            },
        );
    }
}
