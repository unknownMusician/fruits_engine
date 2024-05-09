use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

trait Unimplementable { }

pub trait Number
    : Sized
    + Unimplementable
    + Copy
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
{
    const ZERO: Self;
    const ONE: Self;

    fn into_f64(self) -> f64;
    fn from_f64(v: f64) -> Self;
}

pub trait FloatNumber : Number {
    fn sin(self) -> Self;
    fn cos(self) -> Self;
}

macro_rules! impl_number_trait {
    ($I: ident, $Z: literal, $O: literal) => {
        impl Unimplementable for $I { }

        impl Number for $I {
            const ZERO: Self = $Z;
            const ONE: Self = $O;
            
            fn into_f64(self) -> f64 { self as f64 }
            fn from_f64(v: f64) -> Self { v as Self }
        }
    };
}

impl_number_trait!(f32, 0_f32, 1_f32);
impl_number_trait!(f64, 0_f64, 1_f64);

impl_number_trait!(i8, 0_i8, 1_i8);
impl_number_trait!(i16, 0_i16, 1_i16);
impl_number_trait!(i32, 0_i32, 1_i32);
impl_number_trait!(i64, 0_i64, 1_i64);
impl_number_trait!(i128, 0_i128, 1_i128);

impl_number_trait!(u8, 0_u8, 1_u8);
impl_number_trait!(u16, 0_u16, 1_u16);
impl_number_trait!(u32, 0_u32, 1_u32);
impl_number_trait!(u64, 0_u64, 1_u64);
impl_number_trait!(u128, 0_u128, 1_u128);
