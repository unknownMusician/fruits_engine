pub unsafe trait SameType {
    type Same;

    fn same_into_self(v: Self::Same) -> Self;
    fn same_into_generic(self) -> Self::Same;
    fn same_as_ref_self(v: &Self::Same) -> &Self;
    fn same_as_ref_generic(&self) -> &Self::Same;
    fn same_as_mut_self(v: &mut Self::Same) -> &mut Self;
    fn same_as_mut_generic(&mut self) -> &mut Self::Same;
}

unsafe impl<T> SameType for T {
    type Same = T;

    fn same_into_self(v: Self::Same) -> Self { unsafe { std::mem::transmute(v) } }
    fn same_into_generic(self) -> Self::Same { unsafe { std::mem::transmute(self) } }
    fn same_as_ref_self(v: &Self::Same) -> &Self { unsafe { std::mem::transmute(v) } }
    fn same_as_ref_generic(&self) -> &Self::Same { unsafe { std::mem::transmute(self) } }
    fn same_as_mut_self(v: &mut Self::Same) -> &mut Self { unsafe { std::mem::transmute(v) } }
    fn same_as_mut_generic(&mut self) -> &mut Self::Same { unsafe { std::mem::transmute(self) } }
}