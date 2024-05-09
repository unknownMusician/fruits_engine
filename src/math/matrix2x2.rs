use std::ops::{Index, IndexMut, Mul};

use super::{num::Number, Vec2};

pub struct Matrix2x2<T: Number> {
    data: [[T; 2]; 2],
}

/// Column-major
impl<T: Number> Matrix2x2<T> {
    pub fn from_scale(pos: Vec2<T>) -> Self {
        Self::from_array([
            [pos.x, T::ZERO],
            [T::ZERO, pos.y],
        ])
    }

    pub fn from_rotation(rad: f64) -> Self {
        let sin = rad.sin();
        let cos = rad.cos();

        Self::from_array([
            [Number::from_f64(cos), Number::from_f64(sin)],
            [Number::from_f64(-sin), Number::from_f64(cos)],
        ])
    }

    pub const fn from_array(data: [[T; 2]; 2]) -> Self {
        Self {
            data,
        }
    }

    pub const fn into_array(self) -> [[T; 2]; 2] { self.data }

    pub fn col_0(&self) -> [T; 2] { self.data[0] }
    pub fn col_1(&self) -> [T; 2] { self.data[1] }

    pub fn row_0(&self) -> [T; 2] { [self.data[0][0], self.data[1][0]] }
    pub fn row_1(&self) -> [T; 2] { [self.data[0][1], self.data[1][1]] }

    pub fn get_0_0(&self) -> &T { &self.data[0][0] }
    pub fn get_0_1(&self) -> &T { &self.data[0][1] }
    pub fn get_1_0(&self) -> &T { &self.data[1][0] }
    pub fn get_1_1(&self) -> &T { &self.data[1][1] }

    pub fn get_0_0_mut(&mut self) -> &T { &mut self.data[0][0] }
    pub fn get_0_1_mut(&mut self) -> &T { &mut self.data[0][1] }
    pub fn get_1_0_mut(&mut self) -> &T { &mut self.data[1][0] }
    pub fn get_1_1_mut(&mut self) -> &T { &mut self.data[1][1] }

    pub fn col(&self, x: u8) -> Option<[T; 2]> {
        if x > 1 {
            return None;
        }

        Some(self.data[x as usize])
    }

    pub fn row(&self, y: u8) -> Option<[T; 2]> {
        if y > 1 {
            return None;
        }

        Some([self.data[0][y as usize], self.data[1][y as usize]])
    }

    pub fn get(&self, x: u8, y: u8) -> Option<&T> {
        if x > 1 || y > 1 {
            return None;
        }

        Some(&self.data[x as usize][y as usize])
    }

    pub fn get_mut(&mut self, x: u8, y: u8) -> Option<&mut T> {
        if x > 1 || y > 1 {
            return None;
        }
        
        Some(&mut self.data[x as usize][y as usize])
    }

    pub fn transpose(&mut self) {
        (self.data[0][1], self.data[1][0]) = (self.data[1][0], self.data[0][1]);
    }
}

impl<T: Number> Into<[[T; 2]; 2]> for Matrix2x2<T> {
    fn into(self) -> [[T; 2]; 2] {
        self.into_array()
    }
}

impl<T: Number> From<[[T; 2]; 2]> for Matrix2x2<T> {
    fn from(data: [[T; 2]; 2]) -> Self {
        Self::from_array(data)
    }
}

impl<T: Number> Index<(usize, usize)> for Matrix2x2<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[x][y]
    }
}

impl<T: Number> IndexMut<(usize, usize)> for Matrix2x2<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.data[x][y]
    }
}

impl<T: Number> Clone for Matrix2x2<T> {
    fn clone(&self) -> Self { Self { data: self.data.clone() } }
}
impl<T: Number> Copy for Matrix2x2<T> { }

impl<T: Number> Mul for Matrix2x2<T> {
    type Output = Self;

    fn mul(self, rhs: Matrix2x2<T>) -> Self::Output {
        Self::from_array([
            [
                Vec2::from(self.row_0()).dot(Vec2::from(rhs.col_0())),
                Vec2::from(self.row_0()).dot(Vec2::from(rhs.col_1())),
            ],
            [
                Vec2::from(self.row_1()).dot(Vec2::from(rhs.col_0())),
                Vec2::from(self.row_1()).dot(Vec2::from(rhs.col_1())),
            ],
        ])
    }
}

impl<T: Number> Mul<Vec2<T>> for Matrix2x2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        Vec2::from_array([
            Vec2::from(self.row_0()).dot(rhs),
            Vec2::from(self.row_1()).dot(rhs),
        ])
    }
}

// todo MatrixIndex for optimized consecutive lookup.
// todo const fn and inline where possible/needed.