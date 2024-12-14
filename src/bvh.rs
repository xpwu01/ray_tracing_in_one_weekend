use crate::*;

pub struct BVHNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let mut bbox = AABB::empty();
        for i in start..end {
            let object_bbox = objects[i].bounding_box();
            bbox = AABB::from_boxes(&bbox, &object_bbox);
        }

        let axis = bbox.longest_axis();

        let object_span = end - start;

        let (left, right) = match object_span {
            1 => (objects[start].clone(), objects[start].clone()),
            2 => (objects[start].clone(), objects[start + 1].clone()),
            _ => {
                objects[start..end].sort_by(|a, b| {
                    if axis == 0 {
                        Self::box_x_compare(a, b)
                    } else if axis == 1 {
                        Self::box_y_compare(a, b)
                    } else {
                        Self::box_z_compare(a, b)
                    }
                });
                let mid = start + object_span / 2;
                let left: Rc<dyn Hittable> = Rc::new(BVHNode::new(objects, start, mid));
                let right: Rc<dyn Hittable> = Rc::new(BVHNode::new(objects, mid, end));
                (left, right)
            }
        };

        Self { left, right, bbox }
    }

    pub fn from_list(list: HittableList) -> Self {
        let mut objects = list.objects;
        let end = objects.len();
        Self::new(&mut objects, 0, end)
    }

    fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: usize) -> std::cmp::Ordering {
        let box_a = a.bounding_box();
        let box_b = b.bounding_box();
        let a_axis_interval = &box_a.axis_interval(axis);
        let b_axis_interval = &box_b.axis_interval(axis);
        a_axis_interval
            .min
            .partial_cmp(&b_axis_interval.min)
            .unwrap()
    }

    fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, ray_t) {
            return false;
        }

        let hit_left = self.left.hit(ray, ray_t, rec);
        let hit_right = self.right.hit(
            ray,
            Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max }),
            rec,
        );

        hit_left || hit_right
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
