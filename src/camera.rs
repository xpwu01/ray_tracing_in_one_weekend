use crate::*;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    image_height: u32,
    pub max_depth: u32,
    pub samples_per_pixel: u32,
    pixel_samples_scale: f64,
    camera_centre: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pub vfov: f64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
    // u: Vec3,
    // v: Vec3,
    // w: Vec3,
    pub defocus_angle: f64,
    pub focus_distance: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        max_depth: u32,
        samples_per_pixel: u32,
        vfov: f64,
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_distance: f64,
    ) -> Self {
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        let mut image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
        image_height = if image_height < 1 { 1 } else { image_height };

        let camera_centre = look_from;

        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_distance;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the camera's orthonormal basis.
        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            camera_centre - (w * focus_distance) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;

        let defocus_radius = focus_distance * (degrees_to_radians(defocus_angle) / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            aspect_ratio,
            image_width,
            image_height,
            max_depth,
            samples_per_pixel,
            pixel_samples_scale,
            camera_centre,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            vfov,
            look_from,
            look_at,
            vup,
            // u,
            // v,
            // w,
            defocus_angle,
            focus_distance,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + self.pixel_delta_u * (i as f64 + offset.x())
            + self.pixel_delta_v * (j as f64 + offset.y());

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.camera_centre
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_double();

        Ray::new(ray_origin, ray_direction, ray_time)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.camera_centre + (self.defocus_disk_u * p[0]) + (self.defocus_disk_v * p[1])
    }

    pub fn ray_colour(ray: Ray, depth: u32, world: &dyn Hittable) -> Colour {
        if depth == 0 {
            return Colour::zero();
        }

        let mut rec = HitRecord::new();

        if world.hit(&ray, Interval::new(0.001, f64::INFINITY), &mut rec) {
            let mut scattered = Ray::new(Point3::zero(), Vec3::zero(), 0.0);
            let mut attenuation = Colour::zero();
            if (rec.material).scatter(&ray, &rec, &mut attenuation, &mut scattered) {
                return attenuation * Self::ray_colour(scattered, depth - 1, world);
            }
            return Colour::zero();
        }

        let unit_direction = ray.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        Colour::new(1.0, 1.0, 1.0) * (1.0 - a) + Colour::new(0.5, 0.7, 1.0) * a
    }

    pub fn render(&self, world: &dyn Hittable) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprintln!("\rScanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_colour = Colour::zero();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_colour += Self::ray_colour(ray, self.max_depth, world);
                }
                write_colour(pixel_colour * self.pixel_samples_scale);
            }
        }

        eprintln!("\rDone.");
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            1.0,
            100,
            10,
            10,
            90.0,
            Point3::zero(),
            Point3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            0.0,
            10.0,
        )
    }
}
