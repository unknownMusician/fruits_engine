use std::any::TypeId;

#[derive(Clone, Copy)]
pub struct TypeInfo {
    id: TypeId,
    size: usize,
    dropper: unsafe fn(*mut())
}

impl TypeInfo {
    pub fn new<T: 'static>() -> Self {
        Self {
            id: TypeId::of::<T>(),
            size: std::mem::size_of::<T>(),
            dropper: Self::drop_any::<T>,
        }
    }

    pub fn id(&self) -> &TypeId { &self.id }
    pub fn size(&self) -> usize { self.size }
    pub unsafe fn drop(&self, ptr: *mut()) { (self.dropper)(ptr) }
    pub unsafe fn drop_any<T>(ptr: *mut()) { std::ptr::drop_in_place(ptr as *mut T) }
}
