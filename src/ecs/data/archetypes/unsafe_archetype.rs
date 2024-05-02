use std::cell::UnsafeCell;

pub const CHUNK_SIZE: usize = 1024 * 12;

pub struct ArchetypeItemPhysicalLocation {
    pub memory_offset: usize,
    pub memory_size: usize,
    pub chunk_index: usize
}

pub struct UnsafeArchetype {
    chunks: UnsafeCell<Vec<Box<[u8; CHUNK_SIZE]>>>,
}

// todo: temp
unsafe impl Send for UnsafeArchetype { }
// todo: temp
unsafe impl Sync for UnsafeArchetype { }

impl UnsafeArchetype {
    pub fn new() -> Self {
        Self {
            chunks: UnsafeCell::new(Vec::new()),
        }
    }

    pub unsafe fn get_memory(&self, location: &ArchetypeItemPhysicalLocation) -> (*mut (), usize) {
        let chunks = &mut *self.chunks.get();

        let chunk_ptr = chunks[location.chunk_index].as_mut_ptr();

        let memory_ptr = (chunk_ptr as usize + location.memory_offset) as *mut ();

        (memory_ptr, location.memory_size)
    }

    pub fn chunks_count(&self) -> usize {
        let chunks = unsafe { &*self.chunks.get() };

        chunks.len()
    }

    pub unsafe fn push_chunk(&mut self) {
        let chunks = &mut *self.chunks.get();

        chunks.push(std::iter::repeat(0_u8).take(CHUNK_SIZE).collect::<Box<_>>().try_into().unwrap());
    }

    pub unsafe fn pop_chunk(&mut self) {
        let chunks = &mut *self.chunks.get();

        chunks.pop();
    }
}