use crate::shapes::{ Hitable, HitRecord };
use crate::ray::Ray;

#[derive(Clone)]
pub struct HitableList<T: Hitable + Copy> {
    pub list: Vec<T>
}

impl<T: Hitable + Copy> HitableList<T> {
    pub fn from_list(other: Vec<T>) -> HitableList<T> {
        HitableList {
            list: other
        }
    }
}

pub fn hit<T: Hitable + Copy>(borrowed_list: &Vec<T>, r: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
    let mut temp_rec = HitRecord::default();
    let mut hit_anything = false;
    let mut closest = t_max.clone();

    for current in borrowed_list {
        if (*current).hit(&r, t_min, closest, &mut temp_rec) {
            hit_anything = true;
            closest = temp_rec.t.clone();
            *hit_record = temp_rec;
        }
    }

    return hit_anything;
}
