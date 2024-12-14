use crate::*;

pub struct Sphere {
    centre: Ray,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(centre1: Point3, centre2: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        let radius = radius.max(0.0);
        Self {
            centre: Ray::new(centre1, centre2 - centre1, 0.0),
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let current_centre = self.centre.at(ray.time());
        let oc = current_centre - ray.origin();
        let a = ray.direction().length_squared();
        let h = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            false
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (h - sqrtd) / a;
            if !ray_t.surrounds(root) {
                root = (h + sqrtd) / a;
                if !ray_t.surrounds(root) {
                    return false;
                }
            }
            rec.t = root;
            rec.p = ray.at(rec.t);
            let outward_normal = (rec.p - current_centre) / self.radius;
            rec.set_face_normal(ray, outward_normal);
            rec.material = self.material.clone();

            true
        }
    }
}
