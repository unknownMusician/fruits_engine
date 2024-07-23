use std::{alloc::{GlobalAlloc, System}, sync::atomic::{AtomicUsize, Ordering}};

#[global_allocator]
static ALLOC: AllocMonitor = AllocMonitor;
static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

pub fn allocated() -> usize {
    ALLOCATED.load(Ordering::Relaxed)
}

struct AllocMonitor;

unsafe impl GlobalAlloc for AllocMonitor {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        {
            ALLOCATED.fetch_add(layout.size(), Ordering::Relaxed);
        }

        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        {
            ALLOCATED.fetch_sub(layout.size(), Ordering::Relaxed);
        }

        System.dealloc(ptr, layout)
    }
}
