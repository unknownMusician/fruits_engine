use std::ops::{Index, IndexMut, Mul};

use super::{num::Number, Vec3};

pub struct Matrix3x3<T: Number> {
    data: [[T; 3]; 3],
}

/// Column-major
impl<T: Number> Matrix3x3<T> {
    pub fn from_scale(pos: Vec3<T>) -> Self {
        Self::from_array([
            [pos.x, T::ZERO, T::ZERO],
            [T::ZERO, pos.y, T::ZERO],
            [T::ZERO, T::ZERO, pos.z],
        ])
    }
    pub fn from_euler(euler: Vec3<f64>) -> Self {
        todo!()
    }
    pub fn from_axis_angle(axis: Vec3<f64>, angle: f64) -> Self {
        axis.normalize

        Self::from_array([
            [],
            [],
            []
        ])
        todo!()
    }

    pub const fn from_array(data: [[T; 3]; 3]) -> Self {
        Self {
            data,
        }
    }

    pub const fn into_array(self) -> [[T; 3]; 3] { self.data }

    pub fn col_0(&self) -> [T; 3] { self.data[0] }
    pub fn col_1(&self) -> [T; 3] { self.data[1] }
    pub fn col_2(&self) -> [T; 3] { self.data[2] }

    pub fn row_0(&self) -> [T; 3] { [self.data[0][0], self.data[1][0], self.data[2][0]] }
    pub fn row_1(&self) -> [T; 3] { [self.data[0][1], self.data[1][1], self.data[2][1]] }
    pub fn row_2(&self) -> [T; 3] { [self.data[0][2], self.data[1][2], self.data[2][2]] }

    pub fn get_0_0(&self) -> &T { &self.data[0][0] }
    pub fn get_0_1(&self) -> &T { &self.data[0][1] }
    pub fn get_0_2(&self) -> &T { &self.data[0][2] }
    pub fn get_1_0(&self) -> &T { &self.data[1][0] }
    pub fn get_1_1(&self) -> &T { &self.data[1][1] }
    pub fn get_1_2(&self) -> &T { &self.data[1][2] }
    pub fn get_2_0(&self) -> &T { &self.data[2][0] }
    pub fn get_2_1(&self) -> &T { &self.data[2][1] }
    pub fn get_2_2(&self) -> &T { &self.data[2][2] }

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
        if x > 2 {
            return None;
        }

        Some(self.data[x as usize])
    }

    pub fn row(&self, y: u8) -> Option<[T; 3]> {
        if y > 2 {
            return None;
        }

        Some([self.data[0][y as usize], self.data[1][y as usize], self.data[2][y as usize]])
    }

    pub fn get(&self, x: u8, y: u8) -> Option<&T> {
        if x > 2 || y > 2 {
            return None;
        }

        Some(&self.data[x as usize][y as usize])
    }

    pub fn get_mut(&mut self, x: u8, y: u8) -> Option<&mut T> {
        if x > 2 || y > 2 {
            return None;
        }
        
        Some(&mut self.data[x as usize][y as usize])
    }

    pub fn transpose(&mut self) {
        *self = Self::from_array([
            self.row_0(),
            self.row_1(),
            self.row_2(),
        ])
    }
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

impl<T: Number> Clone for Matrix3x3<T> {
    fn clone(&self) -> Self { Self { data: self.data.clone() } }
}
impl<T: Number> Copy for Matrix3x3<T> { }

impl<T: Number> Mul for Matrix3x3<T> {
    type Output = Self;

    fn mul(self, rhs: Matrix3x3<T>) -> Self::Output {
        Self::from_array([
            [
                Vec3::from(self.row_0()).dot(Vec3::from(rhs.col_0())),
                Vec3::from(self.row_0()).dot(Vec3::from(rhs.col_1())),
                Vec3::from(self.row_0()).dot(Vec3::from(rhs.col_2())),
            ],
            [
                Vec3::from(self.row_1()).dot(Vec3::from(rhs.col_0())),
                Vec3::from(self.row_1()).dot(Vec3::from(rhs.col_1())),
                Vec3::from(self.row_1()).dot(Vec3::from(rhs.col_2())),
            ],
            [
                Vec3::from(self.row_2()).dot(Vec3::from(rhs.col_0())),
                Vec3::from(self.row_2()).dot(Vec3::from(rhs.col_1())),
                Vec3::from(self.row_2()).dot(Vec3::from(rhs.col_2())),
            ],
        ])
    }
}

impl<T: Number> Mul<Vec3<T>> for Matrix3x3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::from_array([
            Vec3::from(self.row_0()).dot(rhs),
            Vec3::from(self.row_1()).dot(rhs),
            Vec3::from(self.row_2()).dot(rhs),
        ])
    }
}

// todo MatrixIndex for optimized consecutive lookup.
// todo const fn and inline where possible/needed.