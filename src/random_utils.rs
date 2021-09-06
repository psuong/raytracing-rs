use rand::prelude::*;
use crate::vec3::Vec3;

/// Helper function to generate a random value between 0 and 1.
///
/// # Parameters
///
/// - `range` : i32
/// The maximum exclusive value that rand will generate from
///
/// # Returns
///
/// A normalized random value between [0..1)
pub fn generate_normalized_ran(range: i32) -> f32 {
    let random_value = rand::thread_rng().gen_range(0..range);
    return random_value as f32 / range as f32;
}

/// Picks a random point inside a unit sphere
///
/// # Returns
///
/// A random unit inside a unit sphere where all x, y, z
/// values are between [0..1).
pub fn random_in_unit_sphere() -> Vec3 {
    let mut p = 2.0 * Vec3::new(
        generate_normalized_ran(100), 
        generate_normalized_ran(100), 
        generate_normalized_ran(100)) - Vec3::from_uniform_value(1.0);

    while p.length_sq() >= 1.0 {
        p = 2.0 * Vec3::new(
            generate_normalized_ran(100), 
            generate_normalized_ran(100), 
            generate_normalized_ran(100)) - Vec3::from_uniform_value(1.0);
    }

    return p;
}
