use crate::vec3::Vec3;

pub fn dot(lhs: Vec3, rhs: Vec3) -> f32 {
    return lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
}

pub fn sqrt(value: Vec3) -> Vec3 {
    return Vec3::new(value.x.sqrt(), value.y.sqrt(), value.z.sqrt());
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    return v - 2.0 * dot(v, n) * n;
}

pub fn refract(v: Vec3, n: Vec3, ni_over_t: f32, refracted: &mut Vec3) -> bool {
    let uv = v.unit_vector();
    let dt = dot(uv, n);

    let discriminant = 1.0 - ni_over_t * ni_over_t * (1.0 - dt * dt);
    if discriminant > 0.0 {
        *refracted = ni_over_t * (uv - n * dt) - n * discriminant.sqrt();
        return true;
    }

    return false;
}

pub fn schlick(cos: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cos).powf(5.0);
}
