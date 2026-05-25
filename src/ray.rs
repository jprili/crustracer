use crate::vec3::{ Vec3 };

pub struct Ray {
    org: Vec3,
    dir: Vec3
}

impl Ray {
    pub fn new(
        origin: Vec3,
        direction: Vec3
    ) -> Self {
        Self {
            org: origin,
            dir: direction
        }
    }

    pub fn at(self, t: f64) -> Vec3 {
        self.org + (self.dir * t)
    }
}
