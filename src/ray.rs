use crate::vec3::{ Vec3 };

#[derive(Clone, Copy)]
pub struct Ray {
    pub org: Vec3,
    pub dir: Vec3
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

    pub fn set_org(&mut self, origin: Vec3) {
        self.org = origin
    }

    pub fn set_dir(&mut self, direction: Vec3) {
        self.dir = direction
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            org: Vec3::default(),
            dir: Vec3::default()
        }
    }
}