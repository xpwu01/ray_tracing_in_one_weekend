pub mod camera;
pub mod colour;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;

pub use camera::*;
pub use colour::*;
pub use hittable::*;
pub use hittable_list::*;
pub use interval::*;
pub use material::*;
pub use ray::*;
pub use sphere::*;
pub use vec3::*;

pub use std::rc::Rc;
pub use std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    rand::random::<f64>()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
