use crate::vec3::Vec3;

pub fn dot(lhs: Vec3, rhs: Vec3) -> f32 {
    return lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
}

pub fn sqrt(value: Vec3) -> Vec3 {
    return Vec3::new(value.x.sqrt(), value.y.sqrt(), value.z.sqrt());
}
