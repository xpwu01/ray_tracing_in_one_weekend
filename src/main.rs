fn main() {
    use ray_tracing_in_one_weekend::*;

    let mut world = HittableList::empty();

    let checker = Rc::new(CheckerTexture::from_colours(
        0.32,
        Colour::new(0.2, 0.3, 0.1),
        Colour::new(0.9, 0.9, 0.9),
    ));
    let ground_material = Rc::new(Lambertian::new(checker));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let centre = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            let mut centre2 = centre;

            if (centre - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material> = if choose_mat < 0.8 {
                    let albedo = Colour::random() * Colour::random();
                    centre2 += Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    Rc::new(Lambertian::from_colour(albedo))
                } else if choose_mat < 0.95 {
                    let albedo = Colour::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    Rc::new(Metal::new(albedo, fuzz))
                } else {
                    Rc::new(Dielectric::new(1.5))
                };

                world.add(Rc::new(Sphere::new(centre, centre2, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::from_colour(Colour::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let world = HittableList::new(Rc::new(BVHNode::from_list(world)));

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;

    let vfov: f64 = 20.0;
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.6;
    let focus_distance = 10.0;

    let cam = Camera::new(
        aspect_ratio,
        image_width,
        max_depth,
        samples_per_pixel,
        vfov,
        look_from,
        look_at,
        vup,
        defocus_angle,
        focus_distance,
    );

    cam.render(&world);
}
