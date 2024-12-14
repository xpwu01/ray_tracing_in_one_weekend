use crate::*;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new(object: Rc<dyn Hittable>) -> Self {
        let bbox = AABB::from_boxes(&AABB::empty(), &object.bounding_box());
        let objects = vec![object];
        Self { objects, bbox }
    }

    pub fn empty() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AABB::empty(),
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.bbox = AABB::from_boxes(&self.bbox, &object.bounding_box());
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::empty()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if object.hit(ray, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
