use crate::*;

pub struct Sphere {
    centre: Ray,
    radius: f64,
    material: Rc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(centre1: Point3, centre2: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        let radius = radius.max(0.0);
        let rvec = Vec3::new(radius, radius, radius);
        let centre = Ray::new(centre1, centre2 - centre1, 0.0);
        let box1 = AABB::from_points(&(centre.at(0.0) - rvec), &(centre.at(0.0) + rvec));
        let box2 = AABB::from_points(&(centre.at(1.0) - rvec), &(centre.at(1.0) + rvec));
        Self {
            centre,
            radius,
            material,
            bbox: AABB::from_boxes(&box1, &box2),
        }
    }

    fn get_sphere_uv(p: &Point3, u: &mut f64, v: &mut f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;
        *u = phi / (2.0 * PI);
        *v = theta / PI;
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
            Self::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
            rec.material = self.material.clone();

            true
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
