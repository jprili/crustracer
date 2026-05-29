use std::ops;
use crate::constants::{ rand_range, rand_unit };

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    e: [f64; 3]
}

impl ops::Neg for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
} 

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.e[idx]
    } 
}

impl ops::Add for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn add(self, vec3: Self) -> Self::Output {
        Self {
            e: [ 
                self.x() + vec3.x(), 
                self.y() + vec3.y(), 
                self.z() + vec3.z()
            ]
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, vec3: Self) {
        *self = Self {
            e: [ 
                self.x() + vec3.x(), 
                self.y() + vec3.y(), 
                self.z() + vec3.z()
            ]
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, vec3: Self) -> Self::Output {
        Self {
            e: [ 
                self.x() - vec3.x(), 
                self.y() - vec3.y(), 
                self.z() - vec3.z()
            ]
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, vec3: Self) {
        *self = Self {
            e: [ 
                self.x() - vec3.x(), 
                self.y() - vec3.y(), 
                self.z() - vec3.z()
            ]
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, scale: f64) -> Self::Output {
        Self {
            e: [ 
                self.x() * scale, 
                self.y() * scale, 
                self.z() * scale
            ]
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, scale: f64) {
        *self = Self {
            e: [ 
                self.x() * scale, 
                self.y() * scale, 
                self.z() * scale
            ]
        }
    }
}


impl ops::Div<f64> for Vec3 {
    type Output = Self;
    #[inline(always)]
    fn div(self, scale: f64) -> Self::Output {
        Self {
            e: [ 
                self.x() / scale, 
                self.y() / scale, 
                self.z() / scale
            ]
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, scale: f64) {
        *self = Self {
            e: [ 
                self.x() / scale, 
                self.y() / scale, 
                self.z() / scale
            ]
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, vec3: Vec3) -> Self::Output {
        Vec3::new(
            self.e[0] * *vec3.x(),
            self.e[1] * *vec3.y(),
            self.e[2] * *vec3.z()
        )
    }
}


impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, vec3: Vec3) -> Self::Output {
        vec3 * self
    }
}

impl ops::Div<Vec3> for f64 {
    type Output = Vec3;

    #[inline(always)]
    fn div(self, vec3: Vec3) -> Self::Output {
        vec3 / self
    }
}

impl Vec3 {
    pub fn new(
        e1: f64,
        e2: f64,
        e3: f64
    ) -> Self {
        Self {
            e: [e1, e2, e3]
        }
    }

    pub fn x(&self) -> &f64 { &self.e[0] }
    pub fn y(&self) -> &f64 { &self.e[1] }
    pub fn z(&self) -> &f64 { &self.e[2] }

    pub fn mag_sq(&self) -> f64 {
              &self.e[0] * &self.e[0]
            + &self.e[1] * &self.e[1]
            + &self.e[2] * &self.e[2]
    }

    pub fn mag(&self) -> f64 {
        *(&self.mag_sq().sqrt())
    }

    #[inline(always)]
    pub fn dot(&self, vec3: Self) -> f64 {
              (&self.e[0] * vec3[0])
            + (&self.e[1] * vec3[1])
            + (&self.e[2] * vec3[2])
    }

    #[inline(always)]
    pub fn cross(&self, vec3: Self) -> Vec3 {
        Self {
            e: [
                (&self.e[1] * vec3.e[2]) - (&self.e[2] * vec3.e[1]),
                (&self.e[2] * vec3.e[0]) - (&self.e[0] * vec3.e[2]),
                (&self.e[0] * vec3.e[1]) - (&self.e[1] * vec3.e[0]),
            ]
        }
    }

    #[inline(always)]
    pub fn unit_vec(&self) -> Self {
        let mag: f64 = self.mag();
        Self {
            e: [ 
                self.x() / mag, 
                self.y() / mag, 
                self.z() / mag 
            ]
        }
    }

    #[inline]
    pub fn rand_vec3() -> Vec3 {
        Vec3::new(
            rand_unit(),
            rand_unit(),
            rand_unit()
        )
    }

    #[inline]
    pub fn rand_vec3_range(lo: f64, hi: f64) -> Vec3 {
        Vec3::new(
            rand_range(lo, hi),
            rand_range(lo, hi),
            rand_range(lo, hi)
        )
    }

    #[inline]
    pub fn rand_vec3_unit() -> Vec3 {
        loop {
            let p = Self::rand_vec3_range(-1., 1.);
            let mag_sq: f64 = p.mag_sq();
            if 1e-160 <= mag_sq && mag_sq <= 1. {
                return p / mag_sq.sqrt()
            }
        }
    }

    #[inline]
    pub fn rand_on_hemisphere(normal: Vec3) -> Vec3 {
        let unit_vec = Vec3::rand_vec3_unit();
        if unit_vec.dot(normal) > 0. {
            unit_vec
        } else { -unit_vec }
    }

    pub fn is_near_zero(&self) -> bool {
        let s = 1e-8;
        let check = |x: f64| x.abs() < s; 
        check(self.e[0]) && check(self.e[1]) && check(self.e[2])
    }

    #[inline]
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - (2. * v.dot(n) * n)
    }

    #[inline]
    pub fn refract(v: Vec3, n: Vec3, ratio: f64) -> Vec3 {
        let cos_th: f64 = f64::min(
            Vec3::dot(&-v, n),
             1.
        );
        let r_out_perp: Vec3 = ratio * (v + cos_th * n);
        let r_out_para: Vec3 = 
            -f64::sqrt(f64::abs(1. - r_out_perp.mag_sq())) * n;
        r_out_perp + r_out_para
    }

    pub fn set_x(&mut self, x: f64) {
        self.e[0] = x
    }

    pub fn set_y(&mut self, y: f64) {
        self.e[1] = y
    }

    pub fn set_z(&mut self, z: f64) {
        self.e[2] = z
    }

    pub fn set(&mut self, i: f64, j: f64, k: f64) {
        self.set_x(i);
        self.set_y(j);
        self.set_z(k);
    }

    pub fn set_vec(&mut self, v: Vec3) {
        self.set(*v.x(), *v.y(), *v.z())
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            e: [0., 0., 0.]
        }
    }
}

