use crate::shapes::{ Hitable, HitRecord, MaterialAccessor };
use crate::ray::Ray;

#[derive(Clone)]
pub struct HitableList<T: Hitable + MaterialAccessor + Copy> {
    pub list: Vec<T>
}

impl<T: Hitable + MaterialAccessor + Copy> HitableList<T> {
    pub fn from_list(other: Vec<T>) -> HitableList<T> {
        HitableList {
            list: other
        }
    }
}

pub fn hit<T: Hitable + MaterialAccessor + Copy>(
    borrowed_list: &Vec<T>, 
    r: &Ray, 
    t_min: f32, 
    t_max: f32, 
    rec: &mut HitRecord) -> bool {

    let mut temp_rec = HitRecord::default();
    let mut hit_anything = false;
    let mut closest = t_max.clone();

    for current in borrowed_list {
        if (*current).hit(&r, t_min, closest, &mut temp_rec) {
            let mat_info = current.get_material_info();

            hit_anything = true;
            closest = temp_rec.t.clone();
            *rec = temp_rec;
            (*rec).material_type = mat_info.0;
            (*rec).material_index = mat_info.1;
        }
    }

    return hit_anything;
}
