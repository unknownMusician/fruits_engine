use std::ops::{Index, IndexMut, Mul};

use crate::Matrix;

use super::{num::Number, Vec2};

#[derive(Copy, Clone, Debug)]
pub struct Matrix2x2<T: Number> {
    data: [[T; 2]; 2],
}

/// Column-major
impl<T: Number> Matrix2x2<T> {
    pub fn from_rotation(angle: T) -> Self {
        let sin = angle.into_f64().sin();
        let cos = angle.into_f64().cos();

        Self::from_array([
            [Number::from_f64(cos), Number::from_f64(-sin)],
            [Number::from_f64(sin), Number::from_f64(cos)],
        ])
    }

    pub const fn from_array(data: [[T; 2]; 2]) -> Self {
        Self {
            data,
        }
    }

    pub const fn into_array(self) -> [[T; 2]; 2] { self.data }

    pub const fn col_0(&self) -> [T; 2] { self.data[0] }
    pub const fn col_1(&self) -> [T; 2] { self.data[1] }

    pub const fn row_0(&self) -> [T; 2] { [self.data[0][0], self.data[1][0]] }
    pub const fn row_1(&self) -> [T; 2] { [self.data[0][1], self.data[1][1]] }

    pub const fn get_0_0(&self) -> &T { &self.data[0][0] }
    pub const fn get_0_1(&self) -> &T { &self.data[0][1] }
    pub const fn get_1_0(&self) -> &T { &self.data[1][0] }
    pub const fn get_1_1(&self) -> &T { &self.data[1][1] }

    pub fn get_0_0_mut(&mut self) -> &T { &mut self.data[0][0] }
    pub fn get_0_1_mut(&mut self) -> &T { &mut self.data[0][1] }
    pub fn get_1_0_mut(&mut self) -> &T { &mut self.data[1][0] }
    pub fn get_1_1_mut(&mut self) -> &T { &mut self.data[1][1] }

    pub const fn col(&self, x: u8) -> Option<[T; 2]> {
        if x > 1 {
            return None;
        }

        Some(self.data[x as usize])
    }

    pub fn row(&self, y: u8) -> Option<[T; 2]> {
        (y <= 1).then(|| [self.data[0][y as usize], self.data[1][y as usize]])
    }

    pub fn get(&self, x: u8, y: u8) -> Option<&T> {
        (x <= 1 && y <= 1).then(|| &self.data[x as usize][y as usize])
    }

    pub fn get_mut(&mut self, x: u8, y: u8) -> Option<&mut T> {
        (x <= 1 && y <= 1).then(|| &mut self.data[x as usize][y as usize])
    }

    pub fn transpose(&mut self) {
        *self = Self::from_array([
            self.row_0(),
            self.row_1(),
        ]);
    }

    pub const fn ignored(&self, x: u8, y: u8) -> T {
        self.ignored_element(x, y, 0, 0)
    }

    const fn ignored_element(&self, ignored_x: u8, ignored_y: u8, index_x: u8, index_y: u8) -> T {
        self.data[index_x as usize + (ignored_x <= index_x) as usize][index_y as usize + (ignored_y <= index_y) as usize]
    }

    pub fn determinant(&self) -> T {
        self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
    }

    pub fn minors(&self) -> Matrix2x2<T> {
        Matrix2x2::from_array([
            [self.ignored(0, 0), self.ignored(0, 1)],
            [self.ignored(1, 0), self.ignored(1, 1)],
        ])
    }

    // todo: only for signed
    pub fn cofactor(&mut self) {
        for x in 0..2 {
            for y in 0..2 {
                if (x + y) % 2 != 0 {
                    self.data[x][y] *= T::ZERO - T::ONE;
                }
            }
        }
    }

    pub fn inverse(&self) -> Option<Matrix2x2<T>> {
        let mut inverse = self.minors();
        inverse.cofactor();
        inverse.transpose();

        let determinant = self.determinant();

        if determinant == T::ZERO {
            return None;
        }

        Some(inverse * (T::ONE / determinant))
    }

    pub fn mul_with_projection(self, rhs: T) -> T {
        let Vec2 { x, y, } = self.mul(Vec2::new(rhs, T::ONE));
        x / y
    }
}

impl<T: Number> Matrix<T> for Matrix2x2<T> {
    const IDENTITY: Self = Self::from_array([
        [T::ONE, T::ZERO],
        [T::ZERO, T::ONE],
    ]);
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

impl<T: Number> Mul for Matrix2x2<T> {
    type Output = Self;

    fn mul(self, rhs: Matrix2x2<T>) -> Self::Output {
        Self::from_array([
            [
                Vec2::from(self.row_0()).dot(&Vec2::from(rhs.col_0())),
                Vec2::from(self.row_1()).dot(&Vec2::from(rhs.col_0())),
            ],
            [
                Vec2::from(self.row_0()).dot(&Vec2::from(rhs.col_1())),
                Vec2::from(self.row_1()).dot(&Vec2::from(rhs.col_1())),
            ],
        ])
    }
}

impl<T: Number> Mul<Vec2<T>> for Matrix2x2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        Vec2::from_array([
            Vec2::from(self.row_0()).dot(&rhs),
            Vec2::from(self.row_1()).dot(&rhs),
        ])
    }
}

impl<T: Number> Mul<T> for Matrix2x2<T> {
    type Output = Matrix2x2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Matrix2x2::from_array([
            [self.data[0][0] * rhs, self.data[0][1] * rhs],
            [self.data[1][0] * rhs, self.data[1][1] * rhs],
        ])
    }
}

// todo MatrixIndex for optimized consecutive lookup.
// todo const fn and inline where possible/needed.