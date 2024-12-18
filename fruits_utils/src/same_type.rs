
pub trait SameType {
    type Same;

    fn same_to_self(v: Self::Same) -> Self;
    fn self_to_same(v: Self) -> Self::Same;
    fn same_to_self_ref(v: &Self::Same) -> &Self;
    fn self_to_same_ref(v: &Self) -> &Self::Same;
    fn same_to_self_mut(v: &mut Self::Same) -> &mut Self;
    fn self_to_same_mut(v: &mut Self) -> &mut Self::Same;
}

impl<T> SameType for T {
    type Same = T;

    fn same_to_self(v: Self::Same) -> Self { v }
    fn self_to_same(v: Self) -> Self::Same { v }
    fn same_to_self_ref(v: &Self::Same) -> &Self { v }
    fn self_to_same_ref(v: &Self) -> &Self::Same { v }
    fn same_to_self_mut(v: &mut Self::Same) -> &mut Self { v }
    fn self_to_same_mut(v: &mut Self) -> &mut Self::Same { v }
} 