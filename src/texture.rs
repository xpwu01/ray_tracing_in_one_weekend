use crate::*;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Colour;
}
#[derive(Debug, Clone, PartialEq)]
pub struct SolidColour {
    albedo: Colour,
}

impl SolidColour {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColour {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Colour {
        self.albedo
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    odd: Rc<dyn Texture>,
    even: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            odd,
            even,
        }
    }

    pub fn from_colours(scale: f64, c1: Colour, c2: Colour) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            odd: Rc::new(SolidColour::new(c1)),
            even: Rc::new(SolidColour::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Colour {
        let x_int = (self.inv_scale * p.x()).floor() as i32;
        let y_int = (self.inv_scale * p.y()).floor() as i32;
        let z_int = (self.inv_scale * p.z()).floor() as i32;

        let is_even = (x_int + y_int + z_int) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
