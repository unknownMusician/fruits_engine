use std::ops::{
    Add,
    AddAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Sub,
    SubAssign,
};

use super::num::Number;

macro_rules! members_count {
    () => (0_usize);
    ($x: ident) => (1_usize);
    ($x: ident, $($xs: ident),*) => (1_usize + members_count!($($xs),*));
}

macro_rules! add_all {
    ($x: tt) => ($x);
    ($x: tt, $($xs: tt),+) => ($x + add_all!($($xs),+));
}

macro_rules! and_all {
    ($x: tt) => (true);
    ($x: tt, $($xs: tt),+) => ($x & and_all!($($xs),+));
}

macro_rules! vec_impl {
    ($V: ident, $($I: ident),+) => {
        #[derive(Copy, Clone, Debug)]
        #[repr(C)]
        pub struct $V<T: Number> {
            $(pub $I: T),+
        }

        impl<T: Number> $V<T> {
            pub const fn new($($I: T),+) -> Self {
                Self { $($I),+ }
            }

            pub const fn with_all(v: T) -> Self {
                Self { $($I: v),+ }
            }

            pub const fn as_array(&self) -> &[T; members_count!($($I),+)] {
                unsafe { std::mem::transmute(self) }
            }

            pub fn as_array_mut(&mut self) -> &mut [T; members_count!($($I),+)] {
                unsafe { std::mem::transmute(self) }
            }

            pub const fn from_array_ref(a: &[T; members_count!($($I),+)]) -> &Self {
                unsafe { std::mem::transmute(a) }
            }

            pub fn from_array_mut(a: &mut [T; members_count!($($I),+)]) -> &mut Self {
                unsafe { std::mem::transmute(a) }
            }

            pub const fn into_array(self) -> [T; members_count!($($I),+)] {
                [
                    $(self.$I),+
                ]
            }

            pub const fn from_array(a: [T; members_count!($($I),+)]) -> Self {
                let [$($I),+] = a;
                Self {
                    $($I),+
                }
            }
            
            pub fn dot(&self, rhs: &Self) -> T {
                add_all!($((self.$I * rhs.$I)),+)
            }

            pub fn length_sq(&self) -> T {
                self.dot(self)
            }

            pub fn length(&self) -> f64 {
                self.dot(self).into_f64().sqrt()
            }

            pub fn normalized(&self) -> Self {
                *self / T::from_f64(self.length())
            }

            pub fn normalized_or_0(&self) -> Self {
                if self == &Self::with_all(T::ZERO) {
                    *self
                } else {
                    self.normalized()
                }
            }
        }

        impl<T: Number> PartialEq for $V<T> {
            fn eq(&self, rhs: &Self) -> bool {
                and_all!($((self.$I == rhs.$I)),+)
            }
        }

        impl<T: Number> Into<[T; members_count!($($I),+)]> for $V<T> {
            fn into(self) -> [T; members_count!($($I),+)] {
                self.into_array()
            }
        }

        impl<T: Number> From<[T; members_count!($($I),+)]> for $V<T> {
            fn from(a: [T; members_count!($($I),+)]) -> Self {
                Self::from_array(a)
            }
        }

        impl<T: Number> Add for $V<T> {
            type Output = Self;
        
            fn add(self, rhs: Self) -> Self::Output {
                Self::Output {
                    $($I: self.$I + rhs.$I),+
                }
            }
        }
        
        impl<T: Number> AddAssign for $V<T> {
            fn add_assign(&mut self, rhs: Self) {
                $(self.$I += rhs.$I);+
            }
        }
        
        impl<T: Number> Sub for $V<T> {
            type Output = Self;
        
            fn sub(self, rhs: Self) -> Self::Output {
                Self::Output {
                    $($I: self.$I - rhs.$I),+
                }
            }
        }
        
        impl<T: Number> SubAssign for $V<T> {
            fn sub_assign(&mut self, rhs: Self) {
                $(self.$I -= rhs.$I);+
            }
        }

        impl<T: Number> Mul for $V<T> {
            type Output = Self;
        
            fn mul(self, rhs: Self) -> Self::Output {
                Self::Output {
                    $($I: self.$I * rhs.$I),+
                }
            }
        }

        impl<T: Number> Mul<T> for $V<T> {
            type Output = Self;
        
            fn mul(self, rhs: T) -> Self::Output {
                Self::Output {
                    $($I: self.$I * rhs),+
                }
            }
        }
        
        impl<T: Number> MulAssign for $V<T> {
            fn mul_assign(&mut self, rhs: Self) {
                $(self.$I *= rhs.$I);+
            }
        }
        
        impl<T: Number> MulAssign<T> for $V<T> {
            fn mul_assign(&mut self, rhs: T) {
                $(self.$I *= rhs);+
            }
        }
        
        impl<T: Number> Div for $V<T> {
            type Output = Self;
        
            fn div(self, rhs: Self) -> Self::Output {
                Self::Output {
                    $($I: self.$I / rhs.$I),+
                }
            }
        }
        
        impl<T: Number> Div<T> for $V<T> {
            type Output = Self;
        
            fn div(self, rhs: T) -> Self::Output {
                Self::Output {
                    $($I: self.$I / rhs),+
                }
            }
        }
        
        impl<T: Number> DivAssign for $V<T> {
            fn div_assign(&mut self, rhs: Self) {
                $(self.$I /= rhs.$I);+
            }
        }
        
        impl<T: Number> DivAssign<T> for $V<T> {
            fn div_assign(&mut self, rhs: T) {
                $(self.$I /= rhs);+
            }
        }
    };
}

vec_impl!{Vec2, x, y}
vec_impl!{Vec3, x, y, z}
vec_impl!{Vec4, x, y, z, w}

impl<T: Number> Vec3<T> {
    fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

// todo: swizzling
// todo: maybe unconstraint from Number trait
// todo: VectorIndex for optimized consecutive lookup.
// todo: const fn and inline where possible/needed.