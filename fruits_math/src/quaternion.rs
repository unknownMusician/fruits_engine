use std::ops::Mul;

use crate::{Matrix3x3, Number, Vec3, Vec4};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Quat<N: Number> {
    pub x: N,
    pub y: N,
    pub z: N,
    pub w: N,
}

impl<N: Number> Quat<N> {
    pub const IDENTITY: Self = Self::new(N::ZERO, N::ZERO, N::ZERO, N::ONE);

    // todo: check (maybe need transposing)
    pub fn to_matrix(&self) -> Matrix3x3<N> {
        let Quat { x, y, z, w } = *self;

        let _1 = N::ONE;
        let _2 = _1 + _1;

        let xy2 = x * y * _2;
        let yy2 = y * y * _2;
        let zz2 = z * z * _2;
        let zw2 = z * w * _2;
        let xz2 = x * z * _2;
        let yw2 = y * w * _2;
        let xx2 = x * x * _2;
        let yz2 = y * z * _2;
        let xw2 = x * w * _2;

        Matrix3x3::from_array([
            [_1 - yy2 - zz2, xy2 + zw2, xz2 - yw2],
            [xy2 - zw2, _1 - xx2 - zz2, yz2 + xw2],
            [xz2 + yw2, yz2 - xw2, _1 - xx2 - yy2],
        ])
    }

    // todo: check (maybe need transposing)
    pub fn from_matrix(m: Matrix3x3<N>) -> Self {
        let _1 = N::ONE;
        let _2 = _1 + _1;
        let _4 = _2 + _2;

        let m = m.into_array();

        let tr = m[0][0] + m[1][1] + m[2][2];

        if tr > N::ZERO {
            // S=4*qw
            let s = N::from_f64((tr + _1).into_f64().sqrt()) * _2;

            let w = s / _4;
            let x = (m[1][2] - m[2][1]) / s;
            let y = (m[2][0] - m[0][2]) / s;
            let z = (m[0][1] - m[1][0]) / s;

            Quat { x, y, z, w }
        } else if (m[0][0] > m[1][1]) && (m[0][0] > m[2][2]) {
            // S=4*qx
            let s = N::from_f64((_1 + m[0][0] - m[1][1] - m[2][2]).into_f64().sqrt()) * _2;

            let w = (m[1][2] - m[2][1]) / s;
            let x = s / _4;
            let y = (m[1][0] + m[0][1]) / s;
            let z = (m[2][0] + m[0][2]) / s;

            Quat { x, y, z, w }
        } else if m[1][1] > m[2][2] {
            // S=4*qy
            let s = N::from_f64((_1 + m[1][1] - m[0][0] - m[2][2]).into_f64().sqrt()) * _2;

            let w = (m[2][0] - m[0][2]) / s;
            let x = (m[1][0] + m[0][1]) / s;
            let y = s / _4;
            let z = (m[2][1] + m[1][2]) / s;

            Quat { x, y, z, w }
        } else {
            // S=4*qz
            let s = N::from_f64((_1 + m[2][2] - m[0][0] - m[1][1]).into_f64().sqrt()) * _2;

            let w = (m[0][1] - m[1][0]) / s;
            let x = (m[2][0] + m[0][2]) / s;
            let y = (m[2][1] + m[1][2]) / s;
            let z = s / _4;

            Quat { x, y, z, w }
        }
    }

    pub fn rotation_x(rad: f64) -> Self {
        let half_angle = rad / 2.0;

        Self {
            x: N::from_f64(half_angle.sin()),
            y: N::ZERO,
            z: N::ZERO,
            w: N::from_f64(half_angle.cos()),
        }
    }

    pub fn rotation_y(rad: f64) -> Self {
        let half_angle = rad / 2.0;

        Self {
            x: N::ZERO,
            y: N::from_f64(half_angle.sin()),
            z: N::ZERO,
            w: N::from_f64(half_angle.cos()),
        }
    }

    pub fn rotation_z(rad: f64) -> Self {
        let half_angle = rad / 2.0;

        Self {
            x: N::ZERO,
            y: N::ZERO,
            z: N::from_f64(half_angle.sin()),
            w: N::from_f64(half_angle.cos()),
        }
    }

    pub fn rotation_axis_angle(mut axis: Vec3<N>, rad: f64) -> Self {
        if axis == Vec3::with_all(N::from_f64(0.0)) {
            return Self::IDENTITY;
        }

        axis = axis.normalized();

        let half_angle = rad / 2.0;

        let half_angle_sin = N::from_f64(half_angle.sin());

        Self {
            x: axis.x * half_angle_sin,
            y: axis.y * half_angle_sin,
            z: axis.z * half_angle_sin,
            w: N::from_f64(half_angle.cos()),
        }
    }

    pub const fn new(x: N, y: N, z: N, w: N) -> Self {
        Self { x, y, z, w }
    }

    pub const fn with_all(v: N) -> Self {
        Self::new(v, v, v, v)
    }

    pub const fn as_array(&self) -> &[N; 4] {
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_array_mut(&mut self) -> &mut [N; 4] {
        unsafe { std::mem::transmute(self) }
    }

    pub const fn from_array_ref(a: &[N; 4]) -> &Self {
        unsafe { std::mem::transmute(a) }
    }

    pub fn from_array_mut(a: &mut [N; 4]) -> &mut Self {
        unsafe { std::mem::transmute(a) }
    }

    pub const fn into_array(self) -> [N; 4] {
        let Self {x, y, z, w } = self;
        [x, y, z, w]
    }

    pub const fn from_array(a: [N; 4]) -> Self {
        let [x, y, z, w] = a;
        Self {x, y, z, w }
    }

    pub fn normalized(&self) -> Self {
        Self::from_array(*Vec4::from_array_ref(self.as_array()).normalized().as_array())
    }

    pub fn normalized_or_0(&self) -> Self {
        Self::from_array(*Vec4::from_array_ref(self.as_array()).normalized_or_0().as_array())
    }
}

impl<N: Number> Mul for Quat<N> {
    type Output = Quat<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        // todo: invalid
        Self::Output {
            w: rhs.w * self.w - rhs.x * self.x - rhs.y * self.y - rhs.z * self.z,
            x: rhs.w * self.x + rhs.x * self.w - rhs.y * self.z + rhs.z * self.y,
            y: rhs.w * self.y + rhs.x * self.z + rhs.y * self.w - rhs.z * self.x,
            z: rhs.w * self.z - rhs.x * self.y + rhs.y * self.x + rhs.z * self.w,
        }
    }
}
