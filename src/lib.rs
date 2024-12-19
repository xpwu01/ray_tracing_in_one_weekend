pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod colour;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod perlin;
pub mod quad;
pub mod ray;
pub mod rtw_image;
pub mod sphere;
pub mod texture;
pub mod vec3;

pub use aabb::*;
pub use bvh::*;
pub use camera::*;
pub use colour::*;
pub use hittable::*;
pub use hittable_list::*;
pub use interval::*;
pub use material::*;
pub use perlin::*;
pub use quad::*;
pub use ray::*;
pub use rtw_image::*;
pub use sphere::*;
pub use texture::*;
pub use vec3::*;

pub use std::f64::consts::PI;
pub use std::rc::Rc;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    rand::random::<f64>()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn random_int_range(min: i32, max: i32) -> i32 {
    random_double_range(min as f64, (max + 1) as f64) as i32
}
