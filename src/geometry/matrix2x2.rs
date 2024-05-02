use std::ops::{Add, Index, IndexMut, Mul};

pub struct Matrix2x2<T> {
    data: [[T; 2]; 2],
}

impl<T> Matrix2x2<T> {
    pub fn get_safe_ref(&self, index: Matrix2x2Index) -> &T {
        &self.data[index.y() as usize][index.x() as usize]
    }
    
    pub fn get_safe_mut(&mut self, index: Matrix2x2Index) -> &mut T {
        &mut self.data[index.y() as usize][index.x() as usize]
    }

    pub fn get_ref(&self, x: u8, y: u8) -> Option<&T> {
        Some(self.get_safe_ref(Matrix2x2Index::new(x, y)?))
    }

    pub fn get_mut(&mut self, x: u8, y: u8) -> Option<&mut T> {
        Some(self.get_safe_mut(Matrix2x2Index::new(x, y)?))
    }
}

impl<T> Index<(usize, usize)> for Matrix2x2<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.1][index.0]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix2x2<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.1][index.0]
    }
}

pub struct Matrix2x2Index {
    x: u8,
    y: u8,
}

impl Matrix2x2Index {
    pub const fn new(x: u8, y: u8) -> Option<Self> {
        match x < 2 && y < 2 {
            true => Some(Self { x, y, }),
            false => None,
        }
    }

    pub const fn x(&self) -> u8 { self.x }
    pub const fn y(&self) -> u8 { self.y }
    pub const fn tuple(&self) -> (u8, u8) { (self.x, self.y) }
}

impl TryFrom<(u8, u8)> for Matrix2x2Index {
    type Error = ();

    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        Self::new(value.0, value.1).ok_or(())
    }
}

// todo
// impl<T: Mul> Mul<Matrix2x2<T>> for Matrix2x2<T> {
//     type Output = Matrix2x2<<<T as Add>::Output> as Mul>;

//     fn mul(self, rhs: Matrix2x2<T>) -> Self::Output {
//         todo!()
//     }
// }