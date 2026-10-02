use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Rayon {
    pub origine: Vec3,
    pub dir: Vec3,
}

impl Rayon {
    pub fn at(self, t: f64) -> Vec3 {
        self.origine + t * self.dir
    }
}
