use crate::{vec3::*, Interval};

pub type Colour = Vec3;

pub fn linear_to_gamma(x: f64) -> f64 {
    x.sqrt()
}

pub fn write_colour(pixel_colour: Colour) {
    let r = linear_to_gamma(pixel_colour.x());
    let g = linear_to_gamma(pixel_colour.y());
    let b = linear_to_gamma(pixel_colour.z());
    let inensity = Interval::new(0.0, 0.999);
    let ir = (256.0 * inensity.clamp(r)) as u32;
    let ig = (256.0 * inensity.clamp(g)) as u32;
    let ib = (256.0 * inensity.clamp(b)) as u32;

    println!("{} {} {}", ir, ig, ib);
}
