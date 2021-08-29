use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    
    pub fn new(a: &Vec3, b: &Vec3) -> Ray {
        Ray {
            origin: a.clone(),
            direction: b.clone()
        }
    }

    #[allow(dead_code)]
    pub fn point_at_parameter(self, t: f32) -> Vec3 {
        self.origin + (self.direction * t)
    }
}

