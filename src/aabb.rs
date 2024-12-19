use crate::*;

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn new(x: &Interval, y: &Interval, z: &Interval) -> Self {
        let mut aabb = Self {
            x: *x,
            y: *y,
            z: *z,
        };
        aabb.pad_to_minimum();
        aabb
    }

    pub fn empty() -> Self {
        Self {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    pub fn from_points(a: &Point3, b: &Point3) -> Self {
        let mut aabb = Self {
            x: Interval::new(a.x().min(b.x()), a.x().max(b.x())),
            y: Interval::new(a.y().min(b.y()), a.y().max(b.y())),
            z: Interval::new(a.z().min(b.z()), a.z().max(b.z())),
        };
        aabb.pad_to_minimum();
        aabb
    }

    pub fn from_boxes(a: &AABB, b: &AABB) -> Self {
        Self {
            x: Interval::enclosing(&a.x, &b.x),
            y: Interval::enclosing(&a.y, &b.y),
            z: Interval::enclosing(&a.z, &b.z),
        }
    }

    pub fn axis_interval(&self, axis: usize) -> &Interval {
        match axis {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }

    pub fn hit(&self, ray: &Ray, mut ray_t: Interval) -> bool {
        let ray_origin = ray.origin();
        let ray_direction = ray.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let inv_d = 1.0 / ray_direction[axis];
            let t0 = (ax.min - ray_origin[axis]) * inv_d;
            let t1 = (ax.max - ray_origin[axis]) * inv_d;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    pub fn longest_axis(&self) -> usize {
        let x_size = self.x.size();
        let y_size = self.y.size();
        let z_size = self.z.size();

        if x_size > y_size {
            if x_size > z_size {
                0
            } else {
                2
            }
        } else {
            if y_size > z_size {
                1
            } else {
                2
            }
        }
    }

    fn pad_to_minimum(&mut self) {
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z.expand(delta);
        }
    }
}
