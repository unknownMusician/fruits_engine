use std::collections::VecDeque;

struct Events<T> {
    events: VecDeque<T>,
    next_indices: Box<[usize]>,
    offset: usize,
}

impl<T> Events<T> {
    pub fn new(readers_count: usize) -> Self {
        Self {
            events: VecDeque::new(),
            next_indices: vec![0; readers_count].into_boxed_slice(),
            offset: 0,
        }
    }
    
    pub fn write(&mut self, event: T) {
        self.events.push_back(event);
    }
    
    pub fn read(&mut self, id: usize) -> Option<&T> {
        let (start, end) = self.events.as_slices();
        
        let next_index = self.next_indices.get(id)?;
        
        let index = next_index.max(&self.offset) - self.offset;
        
        let event = if index < start.len() {
            &start[index]
        } else if index - start.len() < end.len() {
            &end[index - start.len()]
        } else {
            return None;
        };
        
        self.next_indices[id] = index + self.offset + 1;
        Some(event)
    }
    
    pub fn clear_read_events(&mut self) {
        let min_next_index = self.next_indices.iter().min().unwrap();
        
        let delete_count = min_next_index - self.offset;
        
        for _ in 0..delete_count {
            self.events.pop_front();
            self.offset += 1;
        }
    }
}