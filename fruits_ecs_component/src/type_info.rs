use std::any::TypeId;

#[derive(Clone, Copy)]
pub struct TypeInfo {
    id: TypeId,
    name: &'static str,
    size: usize,
    dropper: unsafe fn(*mut())
}

impl TypeInfo {
    pub fn new<T: 'static>() -> Self {
        Self {
            id: TypeId::of::<T>(),
            name: std::any::type_name::<T>(),
            size: std::mem::size_of::<T>(),
            dropper: Self::drop_any::<T>,
        }
    }

    pub const fn id(&self) -> &TypeId { &self.id }
    pub const fn name(&self) -> &'static str { &self.name }
    pub const fn size(&self) -> usize { self.size }
    pub unsafe fn drop(&self, ptr: *mut()) { (self.dropper)(ptr) }
    unsafe fn drop_any<T>(ptr: *mut()) { std::ptr::drop_in_place(ptr as *mut T) }
}
