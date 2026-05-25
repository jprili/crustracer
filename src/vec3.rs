use std::ops;

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
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            e: [0., 0., 0.]
        }
    }
}

