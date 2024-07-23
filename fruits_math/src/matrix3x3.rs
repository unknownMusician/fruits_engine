use std::ops::{Index, IndexMut, Mul};

use crate::{Matrix, Matrix2x2};

use super::{num::Number, Vec2, Vec3};

#[derive(Copy, Clone)]
pub struct Matrix3x3<T: Number> {
    data: [[T; 3]; 3],
}

/// Column-major
impl<T: Number> Matrix3x3<T> {
    // todo
    // pub const fn from_euler(euler: Vec3<f64>) -> Self {
    //     // todo: check axis order
    //     let [c, b, a] = euler.into_array();

    //     let sin_a = a.sin();
    //     let sin_b = b.sin();
    //     let sin_c = c.sin();

    //     let cos_a = a.cos();
    //     let cos_b = b.cos();
    //     let cos_c = c.cos();

    //     Self::from_array([
    //         [],
    //         [],
    //         [],
    //     ])
    // }

    pub const fn from_array(data: [[T; 3]; 3]) -> Self {
        Self {
            data,
        }
    }

    pub const fn into_array(self) -> [[T; 3]; 3] { self.data }

    pub const fn col_0(&self) -> [T; 3] { self.data[0] }
    pub const fn col_1(&self) -> [T; 3] { self.data[1] }
    pub const fn col_2(&self) -> [T; 3] { self.data[2] }

    pub const fn row_0(&self) -> [T; 3] { [self.data[0][0], self.data[1][0], self.data[2][0]] }
    pub const fn row_1(&self) -> [T; 3] { [self.data[0][1], self.data[1][1], self.data[2][1]] }
    pub const fn row_2(&self) -> [T; 3] { [self.data[0][2], self.data[1][2], self.data[2][2]] }

    pub const fn get_0_0(&self) -> &T { &self.data[0][0] }
    pub const fn get_0_1(&self) -> &T { &self.data[0][1] }
    pub const fn get_0_2(&self) -> &T { &self.data[0][2] }
    pub const fn get_1_0(&self) -> &T { &self.data[1][0] }
    pub const fn get_1_1(&self) -> &T { &self.data[1][1] }
    pub const fn get_1_2(&self) -> &T { &self.data[1][2] }
    pub const fn get_2_0(&self) -> &T { &self.data[2][0] }
    pub const fn get_2_1(&self) -> &T { &self.data[2][1] }
    pub const fn get_2_2(&self) -> &T { &self.data[2][2] }

    pub fn get_0_0_mut(&mut self) -> &T { &mut self.data[0][0] }
    pub fn get_0_1_mut(&mut self) -> &T { &mut self.data[0][1] }
    pub fn get_0_2_mut(&mut self) -> &T { &mut self.data[0][2] }
    pub fn get_1_0_mut(&mut self) -> &T { &mut self.data[1][0] }
    pub fn get_1_1_mut(&mut self) -> &T { &mut self.data[1][1] }
    pub fn get_1_2_mut(&mut self) -> &T { &mut self.data[1][2] }
    pub fn get_2_0_mut(&mut self) -> &T { &mut self.data[2][0] }
    pub fn get_2_1_mut(&mut self) -> &T { &mut self.data[2][1] }
    pub fn get_2_2_mut(&mut self) -> &T { &mut self.data[2][2] }

    pub fn col(&self, x: u8) -> Option<[T; 3]> {
        (x <= 2).then(|| self.data[x as usize])
    }

    pub fn row(&self, y: u8) -> Option<[T; 3]> {
        (y <= 2).then(|| [self.data[0][y as usize], self.data[1][y as usize], self.data[2][y as usize]])
    }

    pub fn get(&self, x: u8, y: u8) -> Option<&T> {
        (x <= 2 && y <= 2).then(|| &self.data[x as usize][y as usize])
    }

    pub fn get_mut(&mut self, x: u8, y: u8) -> Option<&mut T> {
        (x <= 2 && y <= 2).then(|| &mut self.data[x as usize][y as usize])
    }

    pub fn transpose(&mut self) {
        *self = Self::from_array([
            self.row_0(),
            self.row_1(),
            self.row_2(),
        ]);
    }

    pub const fn ignored(&self, x: u8, y: u8) -> Matrix2x2<T> {
        Matrix2x2::from_array([
            [self.ignored_element(x, y, 0, 0), self.ignored_element(x, y, 0, 1)],
            [self.ignored_element(x, y, 1, 0), self.ignored_element(x, y, 1, 1)],
        ])
    }

    const fn ignored_element(&self, ignored_x: u8, ignored_y: u8, index_x: u8, index_y: u8) -> T {
        self.data[index_x as usize + (ignored_x <= index_x) as usize][index_y as usize + (ignored_y <= index_y) as usize]
    }

    pub fn determinant(&self) -> T {
        T::ZERO
        + self.data[0][0] * self.ignored(0, 0).determinant()
        - self.data[1][0] * self.ignored(1, 0).determinant()
        + self.data[2][0] * self.ignored(2, 0).determinant()
    }

    pub fn minors(&self) -> Matrix3x3<T> {
        Matrix3x3::from_array([
            [self.ignored(0, 0).determinant(), self.ignored(0, 1).determinant(), self.ignored(0, 2).determinant()],
            [self.ignored(1, 0).determinant(), self.ignored(1, 1).determinant(), self.ignored(1, 2).determinant()],
            [self.ignored(2, 0).determinant(), self.ignored(2, 1).determinant(), self.ignored(2, 2).determinant()],
        ])
    }

    // todo: only for signed
    pub fn cofactor(&mut self) {
        for x in 0..3 {
            for y in 0..3 {
                if (x + y) % 2 != 0 {
                    self.data[x][y] *= T::ZERO - T::ONE;
                }
            }
        }
    }

    pub fn inverse(&self) -> Option<Matrix3x3<T>> {
        let mut inverse = self.minors();
        inverse.cofactor();
        inverse.transpose();

        let determinant = self.determinant();

        if determinant == T::ZERO {
            return None;
        }

        Some(inverse * (T::ONE / determinant))
    }

    pub fn mul_with_projection(self, rhs: Vec2<T>) -> Vec2<T> {
        let Vec3 { x, y, z, } = self.mul(Vec3::new(rhs.x, rhs.y, T::ONE));

        Vec2::new(x, y) / z
    }
}

impl<T: Number> Matrix<T> for Matrix3x3<T> {
    const IDENTITY: Self = Self::from_array([
        [T::ONE, T::ZERO, T::ZERO],
        [T::ZERO, T::ONE, T::ZERO],
        [T::ZERO, T::ZERO, T::ONE],
    ]);
}

impl<T: Number> Into<[[T; 3]; 3]> for Matrix3x3<T> {
    fn into(self) -> [[T; 3]; 3] {
        self.into_array()
    }
}

impl<T: Number> From<[[T; 3]; 3]> for Matrix3x3<T> {
    fn from(data: [[T; 3]; 3]) -> Self {
        Self::from_array(data)
    }
}

//

impl<T: Number> Index<(usize, usize)> for Matrix3x3<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[x][y]
    }
}

impl<T: Number> IndexMut<(usize, usize)> for Matrix3x3<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.data[x][y]
    }
}

impl<T: Number> Mul for Matrix3x3<T> {
    type Output = Self;

    fn mul(self, rhs: Matrix3x3<T>) -> Self::Output {
        Self::from_array([
            [
                Vec3::from(self.row_0()).dot(&Vec3::from(rhs.col_0())),
                Vec3::from(self.row_1()).dot(&Vec3::from(rhs.col_0())),
                Vec3::from(self.row_2()).dot(&Vec3::from(rhs.col_0())),
            ],
            [
                Vec3::from(self.row_0()).dot(&Vec3::from(rhs.col_1())),
                Vec3::from(self.row_1()).dot(&Vec3::from(rhs.col_1())),
                Vec3::from(self.row_2()).dot(&Vec3::from(rhs.col_1())),
            ],
            [
                Vec3::from(self.row_0()).dot(&Vec3::from(rhs.col_2())),
                Vec3::from(self.row_1()).dot(&Vec3::from(rhs.col_2())),
                Vec3::from(self.row_2()).dot(&Vec3::from(rhs.col_2())),
            ],
        ])
    }
}

impl<T: Number> Mul<Vec3<T>> for Matrix3x3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::from_array([
            Vec3::from(self.row_0()).dot(&rhs),
            Vec3::from(self.row_1()).dot(&rhs),
            Vec3::from(self.row_2()).dot(&rhs),
        ])
    }
}

impl<T: Number> Mul<T> for Matrix3x3<T> {
    type Output = Matrix3x3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Matrix3x3::from_array([
            [self.data[0][0] * rhs, self.data[0][1] * rhs, self.data[0][2] * rhs],
            [self.data[1][0] * rhs, self.data[1][1] * rhs, self.data[1][2] * rhs],
            [self.data[2][0] * rhs, self.data[2][1] * rhs, self.data[2][2] * rhs],
        ])
    }
}

// todo MatrixIndex for optimized consecutive lookup.
// todo const fn and inline where possible/needed.