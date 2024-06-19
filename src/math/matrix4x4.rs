use std::ops::{Index, IndexMut, Mul};

use super::{num::Number, Vec3, Vec4};

#[derive(Copy, Clone)]
pub struct Matrix4x4<T: Number> {
    data: [[T; 4]; 4],
}

/// Column-major
impl<T: Number> Matrix4x4<T> {
    pub const fn from_array(data: [[T; 4]; 4]) -> Self {
        Self {
            data,
        }
    }

    pub const fn into_array(self) -> [[T; 4]; 4] { self.data }

    pub const fn col_0(&self) -> [T; 4] { self.data[0] }
    pub const fn col_1(&self) -> [T; 4] { self.data[1] }
    pub const fn col_2(&self) -> [T; 4] { self.data[2] }
    pub const fn col_3(&self) -> [T; 4] { self.data[3] }

    pub const fn row_0(&self) -> [T; 4] { [self.data[0][0], self.data[1][0], self.data[2][0], self.data[3][0]] }
    pub const fn row_1(&self) -> [T; 4] { [self.data[0][1], self.data[1][1], self.data[2][1], self.data[3][1]] }
    pub const fn row_2(&self) -> [T; 4] { [self.data[0][2], self.data[1][2], self.data[2][2], self.data[3][2]] }
    pub const fn row_3(&self) -> [T; 4] { [self.data[0][3], self.data[1][3], self.data[2][3], self.data[3][3]] }

    pub const fn get_0_0(&self) -> &T { &self.data[0][0] }
    pub const fn get_0_1(&self) -> &T { &self.data[0][1] }
    pub const fn get_0_2(&self) -> &T { &self.data[0][2] }
    pub const fn get_0_3(&self) -> &T { &self.data[0][3] }
    pub const fn get_1_0(&self) -> &T { &self.data[1][0] }
    pub const fn get_1_1(&self) -> &T { &self.data[1][1] }
    pub const fn get_1_2(&self) -> &T { &self.data[1][2] }
    pub const fn get_1_3(&self) -> &T { &self.data[1][3] }
    pub const fn get_2_0(&self) -> &T { &self.data[2][0] }
    pub const fn get_2_1(&self) -> &T { &self.data[2][1] }
    pub const fn get_2_2(&self) -> &T { &self.data[2][2] }
    pub const fn get_2_3(&self) -> &T { &self.data[2][3] }
    pub const fn get_3_0(&self) -> &T { &self.data[3][0] }
    pub const fn get_3_1(&self) -> &T { &self.data[3][1] }
    pub const fn get_3_2(&self) -> &T { &self.data[3][2] }
    pub const fn get_3_3(&self) -> &T { &self.data[3][3] }

    pub const fn get_0_0_mut(&mut self) -> &T { &mut self.data[0][0] }
    pub const fn get_0_1_mut(&mut self) -> &T { &mut self.data[0][1] }
    pub const fn get_0_2_mut(&mut self) -> &T { &mut self.data[0][2] }
    pub const fn get_0_3_mut(&mut self) -> &T { &mut self.data[0][3] }
    pub const fn get_1_0_mut(&mut self) -> &T { &mut self.data[1][0] }
    pub const fn get_1_1_mut(&mut self) -> &T { &mut self.data[1][1] }
    pub const fn get_1_2_mut(&mut self) -> &T { &mut self.data[1][2] }
    pub const fn get_1_3_mut(&mut self) -> &T { &mut self.data[1][3] }
    pub const fn get_2_0_mut(&mut self) -> &T { &mut self.data[2][0] }
    pub const fn get_2_1_mut(&mut self) -> &T { &mut self.data[2][1] }
    pub const fn get_2_2_mut(&mut self) -> &T { &mut self.data[2][2] }
    pub const fn get_2_3_mut(&mut self) -> &T { &mut self.data[2][3] }
    pub const fn get_3_0_mut(&mut self) -> &T { &mut self.data[3][0] }
    pub const fn get_3_1_mut(&mut self) -> &T { &mut self.data[3][1] }
    pub const fn get_3_2_mut(&mut self) -> &T { &mut self.data[3][2] }
    pub const fn get_3_3_mut(&mut self) -> &T { &mut self.data[3][3] }

    pub const fn col(&self, x: u8) -> Option<[T; 4]> {
        (x <= 3).then(|| self.data[x as usize])
    }

    pub const fn row(&self, y: u8) -> Option<[T; 4]> {
        (y <= 3).then(|| [self.data[0][y as usize], self.data[1][y as usize], self.data[2][y as usize], self.data[3][y as usize]])
    }

    pub const fn get(&self, x: u8, y: u8) -> Option<&T> {
        (x <= 3 && y <= 3).then(|| &self.data[x as usize][y as usize])
    }

    pub const fn get_mut(&mut self, x: u8, y: u8) -> Option<&mut T> {
        (x <= 3 && y <= 3).then(|| &mut self.data[x as usize][y as usize])
    }

    pub const fn transpose(&mut self) {
        *self = Self::from_array([
            self.row_0(),
            self.row_1(),
            self.row_2(),
            self.row_3(),
        ]);
    }

    pub const fn identity() -> Self {
        Self::from_array([
            [T::ONE, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, T::ONE, T::ZERO, T::ZERO],
            [T::ZERO, T::ZERO, T::ONE, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ONE],
        ])
    }
}

impl<T: Number> Into<[[T; 4]; 4]> for Matrix4x4<T> {
    fn into(self) -> [[T; 4]; 4] {
        self.into_array()
    }
}

impl<T: Number> From<[[T; 4]; 4]> for Matrix4x4<T> {
    fn from(data: [[T; 4]; 4]) -> Self {
        Self::from_array(data)
    }
}

//

impl<T: Number> Index<(usize, usize)> for Matrix4x4<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[x][y]
    }
}

impl<T: Number> IndexMut<(usize, usize)> for Matrix4x4<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.data[x][y]
    }
}

impl<T: Number> Mul for Matrix4x4<T> {
    type Output = Self;

    fn mul(self, rhs: Matrix4x4<T>) -> Self::Output {
        Self::from_array([
            [
                Vec4::from(self.row_0()).dot(&Vec4::from(rhs.col_0())),
                Vec4::from(self.row_0()).dot(&Vec4::from(rhs.col_1())),
                Vec4::from(self.row_0()).dot(&Vec4::from(rhs.col_2())),
                Vec4::from(self.row_0()).dot(&Vec4::from(rhs.col_3())),
            ],
            [
                Vec4::from(self.row_1()).dot(&Vec4::from(rhs.col_0())),
                Vec4::from(self.row_1()).dot(&Vec4::from(rhs.col_1())),
                Vec4::from(self.row_1()).dot(&Vec4::from(rhs.col_2())),
                Vec4::from(self.row_1()).dot(&Vec4::from(rhs.col_3())),
            ],
            [
                Vec4::from(self.row_2()).dot(&Vec4::from(rhs.col_0())),
                Vec4::from(self.row_2()).dot(&Vec4::from(rhs.col_1())),
                Vec4::from(self.row_2()).dot(&Vec4::from(rhs.col_2())),
                Vec4::from(self.row_2()).dot(&Vec4::from(rhs.col_3())),
            ],
            [
                Vec4::from(self.row_3()).dot(&Vec4::from(rhs.col_0())),
                Vec4::from(self.row_3()).dot(&Vec4::from(rhs.col_1())),
                Vec4::from(self.row_3()).dot(&Vec4::from(rhs.col_2())),
                Vec4::from(self.row_3()).dot(&Vec4::from(rhs.col_3())),
            ],
        ])
    }
}

impl<T: Number> Mul<Vec4<T>> for Matrix4x4<T> {
    type Output = Vec4<T>;

    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        Vec4::from_array([
            Vec4::from(self.row_0()).dot(&rhs),
            Vec4::from(self.row_1()).dot(&rhs),
            Vec4::from(self.row_2()).dot(&rhs),
            Vec4::from(self.row_3()).dot(&rhs),
        ])
    }
}

impl<T: Number> Mul<Vec3<T>> for Matrix4x4<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        let Vec4 { x, y, z, w } = self.mul(Vec4::new(rhs.x, rhs.y, rhs.z, T::ONE));

        Vec3::new(x, y, z) / w
    }
}

// todo MatrixIndex for optimized consecutive lookup.
// todo const fn and inline where possible/needed.