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

macro_rules! members_count {
    () => (0_usize);
    ($x: ident) => (1_usize);
    ($x: ident, $($xs: ident),*) => (1_usize + members_count!($($xs),*));
}

macro_rules! add_all {
    ($x: tt) => ($x);
    ($x: tt, $($xs: tt),+) => ($x + add_all!($($xs),+));
}

macro_rules! vec_impl {
    ($T: ident, $($I: ident),+) => {
        pub struct $T<T> {
            $(pub $I: T),+
        }

        impl<T> $T<T> {
            pub const fn new($($I: T),+) -> Self {
                Self { $($I),+ }
            }
        }

        impl<T> Into<[T; members_count!($($I),+)]> for $T<T> {
            fn into(self) -> [T; members_count!($($I),+)] {
                [
                    $(self.$I),+
                ]
            }
        }

        impl<T> From<[T; members_count!($($I),+)]> for $T<T> {
            fn from(a: [T; members_count!($($I),+)]) -> Self {
                let [$($I),+] = a;
                Self {
                    $($I),+
                }
            }
        }

        impl<T> $T<T>
        where
            T: Mul<T, Output = T>,
            T: Add<T, Output = T>,
        {
            pub fn dot(self, rhs: Self) -> T {
                add_all!($((self.$I * rhs.$I)),+)
            }
        }
        
        impl<T: Add<O>, O> Add<$T<O>> for $T<T> {
            type Output = $T<<T as Add<O>>::Output>;
        
            fn add(self, rhs: $T<O>) -> Self::Output {
                Self::Output {
                    $($I: self.$I + rhs.$I),+
                }
            }
        }
        
        impl<T: AddAssign<O>, O> AddAssign<$T<O>> for $T<T> {
            fn add_assign(&mut self, rhs: $T<O>) {
                $(self.$I += rhs.$I);+
            }
        }
        
        impl<T: Sub<O>, O> Sub<$T<O>> for $T<T> {
            type Output = $T<<T as Sub<O>>::Output>;
        
            fn sub(self, rhs: $T<O>) -> Self::Output {
                Self::Output {
                    $($I: self.$I - rhs.$I),+
                }
            }
        }
        
        impl<T: SubAssign<O>, O> SubAssign<$T<O>> for $T<T> {
            fn sub_assign(&mut self, rhs: $T<O>) {
                $(self.$I -= rhs.$I);+
            }
        }

        impl<T: Mul<O>, O> Mul<$T<O>> for $T<T> {
            type Output = $T<<T as Mul<O>>::Output>;
        
            fn mul(self, rhs: $T<O>) -> Self::Output {
                Self::Output {
                    $($I: self.$I * rhs.$I),+
                }
            }
        }
        
        impl<T: MulAssign<O>, O> MulAssign<$T<O>> for $T<T> {
            fn mul_assign(&mut self, rhs: $T<O>) {
                $(self.$I *= rhs.$I);+
            }
        }
        
        impl<T: Div<O>, O> Div<$T<O>> for $T<T> {
            type Output = $T<<T as Div<O>>::Output>;
        
            fn div(self, rhs: $T<O>) -> Self::Output {
                Self::Output {
                    $($I: self.$I / rhs.$I),+
                }
            }
        }
        
        impl<T: DivAssign<O>, O> DivAssign<$T<O>> for $T<T> {
            fn div_assign(&mut self, rhs: $T<O>) {
                $(self.$I /= rhs.$I);+
            }
        }
    };
}

vec_impl!{Vec2, x, y}
vec_impl!{Vec3, x, y, z}
vec_impl!{Vec4, x, y, z, w}

trait CrossProduct<Rhs = Self> {
    type Output;

    fn cross(self, rhs: Rhs) -> Self::Output;
}

impl<T, O> CrossProduct<Vec3<O>> for Vec3<T>
where
    T: Mul<O>,
    <T as Mul<O>>::Output: Sub,
    T: Clone,
    O: Clone,
{
    type Output = Vec3<<<T as Mul<O>>::Output as Sub>::Output>;

    fn cross(self, rhs: Vec3<O>) -> Self::Output {
        Self::Output {
            x: self.y.clone() * rhs.z.clone() - self.z.clone() * rhs.y.clone(),
            y: self.z.clone() * rhs.x.clone() - self.x.clone() * rhs.z.clone(),
            z: self.x * rhs.y.clone() - self.y * rhs.x,
        }
    }
}

// todo: swizzling
