use std::f32;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Mat4([[f32; 4]; 4]);

#[derive(Copy, Clone, Debug)]
pub struct Mat3([[f32; 3]; 3]);

#[derive(Copy, Clone, Debug)]
pub struct Vec3([f32; 3]);

#[derive(Copy, Clone, Debug)]
pub struct Vec4([f32; 4]);

#[allow(dead_code)]
impl Mat3 {
    /*
    00 10 20
    01 11 21
    02 12 22
    */
    pub fn as_array(&self) -> [[f32; 3]; 3] {
        self.0
    }
    pub fn identity() -> Mat3 {
        Mat3([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
    }
    pub fn from_array(a: [[f32; 3]; 3]) -> Mat3 {
        Mat3(a)
    }
    pub fn inverse(&self) -> Option<Mat3> {
        let d = self.det();
        if d == 0.0f32 {
            None
        } else {
            let invdet = 1.0 / d;
            let mut result = Mat3::identity();
            result.0[0][0] = (self.0[1][1] * self.0[2][2] - self.0[2][1] * self.0[1][2]) * invdet;
            result.0[0][1] = -(self.0[0][1] * self.0[2][2] - self.0[0][2] * self.0[2][1]) * invdet;
            result.0[0][2] = (self.0[0][1] * self.0[1][2] - self.0[0][2] * self.0[1][1]) * invdet;
            result.0[1][0] = -(self.0[1][0] * self.0[2][2] - self.0[1][2] * self.0[2][0]) * invdet;
            result.0[1][1] = (self.0[0][0] * self.0[2][2] - self.0[0][2] * self.0[2][0]) * invdet;
            result.0[1][2] = -(self.0[0][0] * self.0[1][2] - self.0[1][0] * self.0[0][2]) * invdet;
            result.0[2][0] = (self.0[1][0] * self.0[2][1] - self.0[2][0] * self.0[1][1]) * invdet;
            result.0[2][1] = -(self.0[0][0] * self.0[2][1] - self.0[2][0] * self.0[0][1]) * invdet;
            result.0[2][2] = (self.0[0][0] * self.0[1][1] - self.0[1][0] * self.0[0][1]) * invdet;
            Some(result)
        }
    }
    pub fn det(&self) -> f32 {
        self.0[0][0] * (self.0[1][1] * self.0[2][2] - self.0[1][2] * self.0[2][1])
            - self.0[0][1] * (self.0[1][0] * self.0[2][2] - self.0[1][2] * self.0[2][0])
            + self.0[0][2] * (self.0[1][0] * self.0[2][1] - self.0[2][0] * self.0[1][1])
    }
}

#[allow(dead_code)]
impl Mat4 {
    /*
    00 10 20 30
    01 11 21 31
    02 12 22 32
    03 13 23 33
    */
    pub fn as_array(&self) -> [[f32; 4]; 4] {
        self.0
    }
    pub fn identity() -> Mat4 {
        Mat4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    pub fn from_blocks(a: Mat3, b: Vec3, c: Vec3, d: f32) -> Mat4 {
        Mat4([
            [a.0[0][0], a.0[0][1], a.0[0][2], c.0[0]],
            [a.0[1][0], a.0[1][1], a.0[1][2], c.0[1]],
            [a.0[2][0], a.0[2][1], a.0[2][2], c.0[2]],
            [b.0[0], b.0[1], b.0[2], d],
        ])
    }
    pub fn from_array(a: [[f32; 4]; 4]) -> Mat4 {
        Mat4(a)
    }
    pub fn scale(s: f32) -> Mat4 {
        Mat4([
            [s, 0.0, 0.0, 0.0],
            [0.0, s, 0.0, 0.0],
            [0.0, 0.0, s, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    pub fn diag(x: f32, y: f32, z: f32, w: f32) -> Mat4 {
        Mat4([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, w],
        ])
    }
    pub fn translation(x: f32, y: f32, z: f32) -> Mat4 {
        Mat4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [x, y, z, 1.0],
        ])
    }
    pub fn rotation(a: f32, mut x: f32, mut y: f32, mut z: f32) -> Mat4 {
        // http://fr.wikipedia.org/wiki/Matrice_de_rotation

        let c = a.cos();
        let s = a.sin();
        let ic = 1.0f32 - c;

        let mut len = x * x + y * y + z * z;
        if len != 1.0 && len != 0.0 {
            len = len.sqrt();
            x /= len;
            y /= len;
            z /= len;
        }

        Mat4([
            [x * x * ic + c, x * y * ic - z * s, x * z * ic + y * s, 0.0],
            [y * x * ic + z * s, y * y * ic + c, y * z * ic - x * s, 0.0],
            [x * z * ic - y * s, y * z * ic + x * s, z * z * ic + c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    pub fn rotation_from_normal(mut x: f32, mut y: f32, mut z: f32) -> Mat4 {
        let mut len = x * x + y * y + z * z;
        if len != 1.0 && len != 0.0 {
            len = len.sqrt();
            x /= len;
            y /= len;
            z /= len;
        }
        Mat4::rotation(f32::atan2(y, x), 0.0, 0.0, 1.0) * Mat4::rotation(z.acos(), 0.0, 1.0, 0.0)
    }
    pub fn perspective(aspect_ratio: f32, fov: f32, znear: f32, zfar: f32) -> Mat4 {
        let f = 1.0 / (fov / 2.0).tan();

        Mat4([
            [f / aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, -(zfar + znear) / (zfar - znear), -1.0],
            [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
        ])
    }
    pub fn to_mat3(&self) -> Mat3 {
        Mat3([
            [self.0[0][0], self.0[0][1], self.0[0][2]],
            [self.0[1][0], self.0[1][1], self.0[1][2]],
            [self.0[2][0], self.0[2][1], self.0[2][2]],
        ])
    }
    pub fn normal_matrix(&self) -> Option<Mat3> {
        self.to_mat3().inverse()
    }
    pub fn inverse(&self) -> Option<Mat4> {
        let d = self.0[3][3];
        if d == 0.0 {
            return None;
        }
        if let Some(ia) = self.normal_matrix() {
            let b = Vec3::from_array([self.0[3][0], self.0[3][1], self.0[3][2]]);
            let c = Vec3::from_array([self.0[0][3], self.0[1][3], self.0[2][3]]);
            let rd = 1.0 / (d - Vec3::dot(c, ia * b));
            let ra = ia + Vec3::extern_prod(rd * (ia * b), c * ia);
            let rb = -rd * (ia * b);
            let rc = -rd * c * ia;
            Some(Mat4::from_blocks(ra, rb, rc, rd))
        } else {
            None
        }
    }
}

#[test]
fn inv4() {
    let m = Mat4::rotation(1.0, 1.0, 1.0, 1.0) * Mat4::translation(1.0, 2.0, 3.0);
    println!("{:?}", m);
    println!("{:?}", m.inverse().unwrap());
    println!("{:?}", m * m.inverse().unwrap());
}

impl ops::Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, _rhs: Mat4) -> Mat4 {
        let mut x = [[0.0f32; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    x[j][i] += self.0[k][i] * _rhs.0[j][k];
                }
            }
        }
        Mat4(x)
    }
}

impl ops::Add for Mat3 {
    type Output = Mat3;

    fn add(self, _rhs: Mat3) -> Mat3 {
        let mut x = [[0.0f32; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                x[j][i] += self.0[j][i] + _rhs.0[j][i];
            }
        }
        Mat3(x)
    }
}

impl Vec3 {
    pub fn from_array(a: [f32; 3]) -> Vec3 {
        Vec3(a)
    }

    pub fn dot(a: Vec3, b: Vec3) -> f32 {
        a.0[0] * b.0[0] + a.0[1] * b.0[1] + a.0[2] * b.0[2]
    }

    pub fn extern_prod(a: Vec3, b: Vec3) -> Mat3 {
        let mut x = [[0.0f32; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                x[j][i] = a.0[j] * b.0[i];
            }
        }
        Mat3::from_array(x)
    }
}

impl ops::Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        let mut x = [0.0f32; 3];
        for i in 0..3 {
            for j in 0..3 {
                x[i] += self.0[j][i] * _rhs.0[j];
            }
        }
        Vec3(x)
    }
}

impl ops::Mul<Mat3> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Mat3) -> Vec3 {
        let mut x = [0.0f32; 3];
        for i in 0..3 {
            for j in 0..3 {
                x[i] += self.0[j] * _rhs.0[i][j];
            }
        }
        Vec3(x)
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        let mut x = [0.0f32; 3];
        for i in 0..3 {
            x[i] += self * _rhs.0[i];
        }
        Vec3(x)
    }
}

impl Vec4 {
    pub fn from_array(a: [f32; 4]) -> Vec4 {
        Vec4(a)
    }
    pub fn as_array(&self) -> [f32; 4] {
        self.0
    }
}

impl ops::Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, _rhs: Vec4) -> Vec4 {
        let mut x = [0.0f32; 4];
        for i in 0..4 {
            for j in 0..4 {
                x[i] += self.0[j][i] * _rhs.0[j];
            }
        }
        Vec4(x)
    }
}
