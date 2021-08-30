use crate::vec3::Vec3;

pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f32 {
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
}
