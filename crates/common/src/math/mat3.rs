/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use super::*;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Mat3([Vec3; 3]);

impl Mat3 {
    /// Matrix with all elements set to 0
    pub const ZERO: Mat3 = Mat3([
        Vec3{x:  0.0, y:  0.0, w:  0.0},
        Vec3{x:  0.0, y:  0.0, w:  0.0},
        Vec3{x:  0.0, y:  0.0, w:  0.0},
    ]);

    /// Diagonal matrix of 1.0, applies no transformations
    pub const IDENTITY: Mat3 = Mat3([
        Vec3{x: 1.0, y:  0.0, w:  0.0},
        Vec3{x: 0.0, y:  1.0, w:  0.0},
        Vec3{x: 0.0, y:  0.0, w:  1.0},
    ]);

    /// Rotation matrix to rotate 90 degrees ccw
    pub const ROT_CCW_090: Mat3 = Mat3([
        Vec3{x:  0.0, y: -1.0, w:  0.0},
        Vec3{x:  1.0, y:  0.0, w:  0.0},
        Vec3{x:  0.0, y:  0.0, w:  1.0},
    ]);

    /// Rotation matrix to rotate 180 degrees ccw
    pub const ROT_CCW_180: Mat3 = Mat3([
        Vec3{x: -1.0, y:  0.0, w:  0.0},
        Vec3{x:  0.0, y: -1.0, w:  0.0},
        Vec3{x:  0.0, y:  0.0, w:  1.0},
    ]);

    /// Rotation matrix to rotate 270 degrees ccw
    pub const ROT_CCW_270: Mat3 = Mat3([
        Vec3{x:  0.0, y:  1.0, w:  0.0},
        Vec3{x: -1.0, y:  0.0, w:  0.0},
        Vec3{x:  0.0, y:  0.0, w:  1.0},
    ]);

}

impl Mat3 {
    /// Constructs a matrix with the given value down the l -> r diagonal of the matrix
    #[inline] pub fn new_diagonal(v: real) -> Mat3 {
        Mat3([
            Vec3::new(v,   0.0, 0.0),
            Vec3::new(0.0, v,   0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ])
    }

    /// Constructs a scale matrix
    #[inline] pub fn new_scale(v: Vec2) -> Mat3 {
        Mat3([
            Vec3::new(v[0],  0.0, 0.0),
            Vec3::new( 0.0, v[1], 0.0),
            Vec3::new( 0.0,  0.0, 1.0),
        ])
    }

    /// Constructs a skew matrix
    #[inline] pub fn new_skew(v: Vec2) -> Mat3 {
        Mat3([
            Vec3::new( 1.0, v[0], 0.0),
            Vec3::new(v[1],  1.0, 0.0),
            Vec3::new( 0.0,  0.0, 1.0),
        ])
    }

    /// Constructs a rotation matrix from the assumed unit-length normal
    #[inline] pub fn new_rotation_from_norm(v: Vec2) -> Mat3 {
        Mat3([
            Vec3::new(v[0], -v[1], 0.0),
            Vec3::new(v[1],  v[0], 0.0),
            Vec3::new( 0.0,   0.0, 1.0),
        ])
    }

    /// Constructs a reflection matrix from the assumed unit-length normal of 2*angle
    #[inline] pub fn new_reflection_from_dnorm(v: Vec2) -> Mat3 {
        Mat3([
            Vec3::new(v[0],  v[1], 0.0),
            Vec3::new(v[1], -v[0], 0.0),
            Vec3::new( 0.0,   0.0, 1.0),
        ])
    }
}

impl Mat3 {

    /// Calculates the determinant of the matrix
    #[inline] pub fn determinant(self) -> real {
        let m00 = self[1].y*self[2].y - self[1].w*self[2].w;
        let m01 = self[1].x*self[2].x - self[1].w*self[2].w;
        let m02 = self[1].x*self[2].x - self[1].y*self[2].y;
        self[0].x*m00 + self[0].y*m01 + self[0].w*m02
    }

    /// Calculates the transposition of the matrix
    #[inline] pub fn transpose(self) -> Mat3 {
        Mat3([self.col(0), self.col(1), self.col(2)])
    }

    /// Attempts to calculate the inverse of the matrix
    /// If `self` has a determinant of 0/NaN/Inf, then `None` will be returned
    #[inline] pub fn inverse(self) -> Option<Mat3> {
        let m00 =   self[1].y*self[2].y - self[1].w*self[2].w;
        let m01 = -(self[1].x*self[2].x - self[1].w*self[2].w);
        let m02 =   self[1].x*self[2].x - self[1].y*self[2].y;

        let m10 = -(self[0].y*self[2].y - self[0].w*self[2].w);
        let m11 =   self[0].x*self[2].x - self[0].w*self[2].w;
        let m12 = -(self[0].x*self[2].x - self[0].y*self[2].y);

        let m20 =   self[0].y*self[1].y - self[0].w*self[1].w;
        let m21 = -(self[0].x*self[1].x - self[0].w*self[1].w);
        let m22 =   self[0].x*self[1].x - self[0].y*self[1].y;

        let inv_det = 1.0/(self[0].x*m00 - self[0].y*m01 + self[0].w*m02);
        match inv_det.is_finite() {
            false => None,
            true  => Some(Mat3([
                Vec3::new(inv_det*m00, inv_det*m10, inv_det*m20),
                Vec3::new(inv_det*m01, inv_det*m11, inv_det*m21),
                Vec3::new(inv_det*m02, inv_det*m12, inv_det*m22)
            ]))
        }
    }

    /// Calculates the inverse of a pure rotation and translation matrix
    /// by transposing the rotation component, and setting the translation
    /// component to the negation of the translation multiplied by the
    /// inverse rotation matrix
    #[inline] pub fn inverse_fast_tr(self) -> Mat3 {
        let pos = Vec3::new(-self[0].w, -self[1].w, 0.0);
        Mat3([
            Vec3::new(self[0].x, self[1].x, self.col(0).dot(pos)),
            Vec3::new(self[0].y, self[1].y, self.col(1).dot(pos)),
            Vec3::new(      0.0,       0.0,                  1.0),
        ])
    }
}

impl Mat3 {
    #[inline] pub fn mul_vec(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self[0].dot(other),
            self[1].dot(other),
            self[2].dot(other)
        )
    }

    pub fn mul_mat(self, other: Mat3) -> Mat3 {
        Mat3([
            Vec3::new(self[0].dot(other.col(0)), self[0].dot(other.col(1)), self[0].dot(other.col(2))),
            Vec3::new(self[1].dot(other.col(0)), self[1].dot(other.col(1)), self[1].dot(other.col(2))),
            Vec3::new(self[2].dot(other.col(0)), self[2].dot(other.col(1)), self[2].dot(other.col(2)))
        ])
    }

    #[inline] pub fn add_comp(self, other: Mat3) -> Mat3 {
        Mat3([self[0]+other[0], self[1]+other[1], self[2]+other[2]])
    }

    #[inline] pub fn sub_comp(self, other: Mat3) -> Mat3 {
        Mat3([self[0]-other[0], self[1]-other[1], self[2]-other[2]])
    }

    #[inline] pub fn mul_comp(self, other: Mat3) -> Mat3 {
        Mat3([self[0].mul_comp(other[0]), self[1].mul_comp(other[1]), self[2].mul_comp(other[2])])
    }

    #[inline] pub fn div_comp(self, other: Mat3) -> Mat3 {
        Mat3([self[0].div_comp(other[0]), self[1].div_comp(other[1]), self[2].div_comp(other[2])])
    }

    #[inline] pub fn mul_scalar(self, other: real) -> Mat3 {
        Mat3([self[0]*other, self[1]*other, self[2]*other])
    }

    #[inline] pub fn div_scalar(self, other: real) -> Mat3 {
        Mat3([self[0]/other, self[1]/other, self[2]/other])
    }

    #[inline] pub fn recip_scalar(self, other: real) -> Mat3 {
        Mat3([other/self[0], other/self[1], other/self[2]])
    }
}

impl_mat2d!(Mat3, Vec3);
const_assert!(core::mem::size_of::<Mat3>() == 9*core::mem::size_of::<real>());