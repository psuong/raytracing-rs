use crate::shapes::{ Hitable, HitRecord };
use crate::ray::Ray;

struct HitableList<T> where T : Hitable {
    list: Vec<T>
}

impl<T> HitableList<T> where T : Hitable {
    pub fn with_capacity(size: usize) -> HitableList<T> {
        HitableList {
            list : Vec::with_capacity(size)
        }
    }

    pub fn hit(self, r: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest = t_max.clone();

        for current in self.list {
            if current.hit(&r, t_min, closest, &mut temp_rec) {
                hit_anything = true;
                closest = temp_rec.t.clone();
                *hit_record = temp_rec;
            }
        }

        return hit_anything;
    }
}