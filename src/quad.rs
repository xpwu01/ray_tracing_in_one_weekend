use crate::*;

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    material: Arc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: Arc<dyn Material>) -> Self {
        let bbox = AABB::empty();
        let n = u.cross(v);
        let normal = n.unit_vector();
        let d = normal.dot(q);
        let w = n / n.dot(n);
        let mut quad = Self {
            q,
            u,
            v,
            w,
            material,
            bbox,
            normal,
            d,
        };
        quad.set_bounding_box();
        quad
    }

    pub fn set_bounding_box(&mut self) {
        let bbox_diagonal1 = AABB::from_points(&self.q, &(self.q + self.u + self.v));
        let bbox_diagonal2 = AABB::from_points(&(self.q + self.u), &(self.q + self.v));
        self.bbox = AABB::from_boxes(&bbox_diagonal1, &bbox_diagonal2);
    }

    pub fn is_interior(alpha: f64, beta: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);

        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
            return false;
        }

        rec.u = alpha;
        rec.v = beta;

        true
    }

    pub fn block(a: &Point3, b: &Point3, material: Arc<dyn Material>) -> Arc<HittableList> {
        let mut sides = HittableList::empty();

        let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
        let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

        let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

        sides.add(Arc::new(Quad::new(
            Point3::new(min.x(), min.y(), max.z()),
            dx,
            dy,
            material.clone(),
        )));
        sides.add(Arc::new(Quad::new(
            Point3::new(max.x(), min.y(), max.z()),
            -dz,
            dy,
            material.clone(),
        )));
        sides.add(Arc::new(Quad::new(
            Point3::new(max.x(), min.y(), min.z()),
            -dx,
            dy,
            material.clone(),
        )));
        sides.add(Arc::new(Quad::new(
            Point3::new(min.x(), min.y(), min.z()),
            dz,
            dy,
            material.clone(),
        )));
        sides.add(Arc::new(Quad::new(
            Point3::new(min.x(), max.y(), max.z()),
            dx,
            -dz,
            material.clone(),
        )));
        sides.add(Arc::new(Quad::new(
            Point3::new(min.x(), min.y(), min.z()),
            dx,
            dz,
            material.clone(),
        )));

        Arc::new(sides)
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = self.normal.dot(ray.direction());
        if denom.abs() < 1e-8 {
            return false;
        }

        let t = (self.d - self.normal.dot(ray.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = ray.at(t);
        let planar_hit_point_vector = intersection - self.q;
        let alpha = self.w.dot(planar_hit_point_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hit_point_vector));

        if !Self::is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.material = self.material.clone();
        rec.set_face_normal(ray, self.normal);

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
