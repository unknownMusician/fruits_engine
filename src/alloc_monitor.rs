use std::{alloc::{GlobalAlloc, System}, sync::Mutex};

#[global_allocator]
static ALLOC: LoggingAlloc = LoggingAlloc;
static ALLOCATED: Mutex<usize> = Mutex::new(0);

struct LoggingAlloc;

pub fn allocated() -> usize {
    *ALLOCATED.lock().unwrap()
}

unsafe impl GlobalAlloc for LoggingAlloc {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        {
            *ALLOCATED.lock().unwrap() += layout.size();
        }

        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        {
            *ALLOCATED.lock().unwrap() -= layout.size();
        }

        System.dealloc(ptr, layout)
    }
}
