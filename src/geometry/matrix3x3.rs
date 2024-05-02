use std::ops::{Index, IndexMut};

pub struct Matrix<const X: usize, const Y: usize, T> {
    data: [[T; X]; Y],
}

impl<const X: usize, const Y: usize, T> Matrix<X, Y, T> {
    pub fn get_ref(&self, i: usize, j: usize) -> Option<&T> {
        match i < X || j < Y {
            true => Some(&self[(i, j)]),
            false => None,
        }
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
        match i < X || j < Y {
            true => Some(&mut self[(i, j)]),
            false => None,
        }
    }
}

impl<const X: usize, const Y: usize, T> Index<(usize, usize)> for Matrix<X, Y, T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.1][index.0]
    }
}

impl<const X: usize, const Y: usize, T> IndexMut<(usize, usize)> for Matrix<X, Y, T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.1][index.0]
    }
}

pub struct Matrix3x3<T> {
    data: [T; 3 * 3],
}

impl<T> Matrix3x3<T> {
    pub fn get_ref(&self, i: usize, j: usize) -> Option<&T> {
        match i < 3 || j < 3 {
            true => Some(&self[(i, j)]),
            false => None,
        }
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
        match i < 3 || j < 3 {
            true => Some(&mut self[(i, j)]),
            false => None,
        }
    }
}

impl<T> Index<(usize, usize)> for Matrix3x3<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.1 * 3 + index.0]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix3x3<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.1 * 3 + index.0]
    }
}