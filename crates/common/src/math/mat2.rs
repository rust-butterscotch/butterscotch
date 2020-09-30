/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use super::{Vec2, real};

// Row major
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Mat2([Vec2; 2]);

impl Mat2 {
    /// Matrix with all elements set to 0
    pub const ZERO: Mat2 = Mat2([
        vec2!(0.0, 0.0),
        vec2!(0.0, 0.0),
    ]);

    /// Diagonal matrix of 1.0, applies no transformations
    pub const IDENTITY: Mat2 = Mat2([
        vec2!(1.0, 0.0),
        vec2!(0.0, 1.0),
    ]);

    /// Rotation matrix to rotate 90 degrees ccw
    pub const ROT_CCW_090: Mat2 = Mat2([
        vec2!( 0.0, -1.0),
        vec2!( 1.0,  0.0),
    ]);

    /// Rotation matrix to rotate 180 degrees ccw
    pub const ROT_CCW_180: Mat2 = Mat2([
        vec2!(-1.0,  0.0),
        vec2!( 0.0, -1.0),
    ]);

    /// Rotation matrix to rotate 270 degrees ccw
    pub const ROT_CCW_270: Mat2 = Mat2([
        vec2!( 0.0,  1.0),
        vec2!(-1.0,  0.0),
    ]);

}

impl Mat2 {
    /// Constructs a matrix with the given value down the l -> r diagonal of the matrix
    pub fn new_diagonal(v: real) -> Mat2 {
        Mat2([
            vec2!(v,   0.0),
            vec2!(0.0, v  ),
        ])
    }

    /// Constructs a scale matrix
    pub fn new_scale(v: Vec2) -> Mat2 {
        Mat2([
            vec2!(v[0],  0.0),
            vec2!( 0.0, v[1])
        ])
    }

    /// Constructs a skew matrix
    pub fn new_skew(v: Vec2) -> Mat2 {
        Mat2([
            vec2!( 1.0, v[0]),
            vec2!(v[1],  1.0)
        ])
    }

    /// Constructs a rotation matrix from the assumed unit-length normal
    pub fn new_rotation_from_norm(v: Vec2) -> Mat2 {
        Mat2([
            vec2!(v[0], -v[1]),
            vec2!(v[1],  v[0]),
        ])
    }

    /// Constructs a reflection matrix from the assumed unit-length normal of 2*angle
    pub fn new_reflection_from_dnorm(v: Vec2) -> Mat2 {
        Mat2([
            vec2!(v[0],  v[1]),
            vec2!(v[1], -v[0]),
        ])
    }
}

impl Mat2 {

    /// Calculates the determinant of the matrix
    pub fn determinant(self) -> real {
        self[0][0]*self[1][1] - self[0][1]*self[1][0]
    }

    /// Calculates the transposition of the matrix
    pub fn transpose(self) -> Mat2 {
        Mat2([self.col(0), self.col(1)])
    }

    /// Attempts to calculate the inverse of the matrix
    /// If `self` has a determinant of 0/NaN/Inf, then `None` will be returned
    pub fn inverse(self) -> Option<Mat2> {
        let inv_det = 1.0/self.determinant();
        match inv_det.is_finite() {
            false => None,
            true  => Some(Mat2([
                vec2!( self[1][1]*inv_det, -self[0][1]*inv_det),
                vec2!(-self[1][0]*inv_det,  self[0][0]*inv_det),
            ]))
        }
    }
}

impl Mat2 {
    /// Multiplies the given vector by this matrix, applying the transformation
    pub fn mul_vec(self, other: Vec2) -> Vec2 {
        vec2!(
            self[0].dot(other),
            self[1].dot(other)
        )
    }

    /// Multiplies the given matrix by this matrix, applying the transformation
    pub fn mul_mat(self, other: Mat2) -> Mat2 {
        Mat2([
            vec2!(self[0].dot(other.col(0)), self[0].dot(other.col(1))),
            vec2!(self[1].dot(other.col(0)), self[1].dot(other.col(1))),
        ])
    }

    /// Calculates the sums of two matrices component-wise
    pub fn add_comp(self, other: Mat2) -> Mat2 {
        Mat2([self[0]+other[0], self[1]+other[1]])
    }

    /// Calculates the difference of two matrices component-wise
    pub fn sub_comp(self, other: Mat2) -> Mat2 {
        Mat2([self[0]-other[0], self[1]-other[1]])
    }

    /// Multiplies each component of the matrix by a fixed value
    pub fn mul_scalar(self, other: real) -> Mat2 {
        Mat2([self[0]*other, self[1]*other])
    }

    /// Divides each component of the matrix by a fixed value
    pub fn div_scalar(self, other: real) -> Mat2 {
        Mat2([self[0]/other, self[1]/other])
    }

    /// Divides a fixed value by each component of the matrix
    pub fn recip_scalar(self, other: real) -> Mat2 {
        Mat2([other/self[0], other/self[1]])
    }
}

impl_mat2d!(Mat2, Vec2);
const_assert!(core::mem::size_of::<Mat2>() == 4*core::mem::size_of::<real>());