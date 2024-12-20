use ray_tracing_in_one_weekend::*;

fn main() {
    let case = 9;

    match case {
        0 => final_scene(400, 250, 4),
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        9 => final_scene(800, 10000, 40),
        _ => panic!("Invalid case"),
    }
}

fn bouncing_spheres() {
    let mut world = HittableList::empty();

    let ground_material = Rc::new(Lambertian::from_colour(Colour::new(0.5, 0.5, 0.5)));

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
    let background = Colour::new(0.7, 0.8, 1.0);

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
        background,
    );

    cam.render(&world);
}

fn checkered_spheres() {
    let mut world = HittableList::empty();

    let checker = Rc::new(CheckerTexture::from_colours(
        0.32,
        Colour::new(0.2, 0.3, 0.1),
        Colour::new(0.9, 0.9, 0.9),
    ));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Rc::new(Lambertian::new(checker.clone())),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Rc::new(Lambertian::new(checker)),
    )));

    let world = HittableList::new(Rc::new(BVHNode::from_list(world)));

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;
    let background = Colour::new(0.7, 0.8, 1.0);

    let vfov: f64 = 20.0;
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
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
        background,
    );

    cam.render(&world);
}

fn earth() {
    let earth_texture = Rc::new(ImageTexture::new(RtwImage::new("earthmap.jpg").unwrap()));
    let earth_surface = Rc::new(Lambertian::new(earth_texture));
    let globe = Rc::new(Sphere::new(
        Point3::zero(),
        Point3::zero(),
        2.0,
        earth_surface,
    ));

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;
    let background = Colour::new(0.7, 0.8, 1.0);

    let vfov: f64 = 20.0;
    let look_from = Point3::new(0.0, 0.0, 12.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
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
        background,
    );

    cam.render(&HittableList::new(globe));
}

fn perlin_spheres() {
    let mut world = HittableList::empty();

    let perlin_texture = Rc::new(NoiseTexture::<256>::new(4.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(perlin_texture.clone())),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new(perlin_texture)),
    )));

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;
    let background = Colour::new(0.7, 0.8, 1.0);

    let vfov: f64 = 20.0;
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
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
        background,
    );

    cam.render(&world);
}

fn quads() {
    let mut world = HittableList::empty();

    let left_red = Rc::new(Lambertian::from_colour(Colour::new(1.0, 0.2, 0.2)));
    let back_green = Rc::new(Lambertian::from_colour(Colour::new(0.2, 1.0, 0.2)));
    let right_blue = Rc::new(Lambertian::from_colour(Colour::new(0.2, 0.2, 1.0)));
    let upper_orange = Rc::new(Lambertian::from_colour(Colour::new(1.0, 0.5, 0.0)));
    let lower_teal = Rc::new(Lambertian::from_colour(Colour::new(0.2, 0.8, 0.8)));

    world.add(Rc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;
    let background = Colour::new(0.7, 0.8, 1.0);

    let vfov: f64 = 80.0;
    let look_from = Point3::new(0.0, 0.0, 9.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
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
        background,
    );

    cam.render(&world);
}

fn simple_light() {
    let mut world = HittableList::empty();

    let pertext = Rc::new(NoiseTexture::<256>::new(4.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(pertext.clone())),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new(pertext)),
    )));

    let difflight = Rc::new(DiffuseLight::from_colour(Colour::new(4.0, 4.0, 4.0)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight,
    )));

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;
    let background = Colour::zero();

    let vfov: f64 = 20.0;
    let look_from = Point3::new(26.0, 3.0, 6.0);
    let look_at = Point3::new(0.0, 2.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
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
        background,
    );

    cam.render(&world);
}

fn cornell_box() {
    let mut world = HittableList::empty();

    let red = Rc::new(Lambertian::from_colour(Colour::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::from_colour(Colour::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::from_colour(Colour::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::from_colour(Colour::new(15.0, 15.0, 15.0)));

    world.add(Rc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = Quad::block(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = Rc::new(RotateY::new(box1, 15.0));
    let box1 = Rc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let box2 = Quad::block(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2 = Rc::new(RotateY::new(box2, -18.0));
    let box2 = Rc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = 600;
    let samples_per_pixel: u32 = 200;
    let max_depth: u32 = 50;
    let background = Colour::zero();

    let vfov: f64 = 40.0;
    let look_from = Point3::new(278.0, 278.0, -800.0);
    let look_at = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
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
        background,
    );

    cam.render(&world);
}

fn cornell_smoke() {
    let mut world = HittableList::empty();

    let red = Rc::new(Lambertian::from_colour(Colour::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::from_colour(Colour::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::from_colour(Colour::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::from_colour(Colour::new(7.0, 7.0, 7.0)));

    world.add(Rc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = Quad::block(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = Rc::new(RotateY::new(box1, 15.0));
    let box1 = Rc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    let box2 = Quad::block(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2 = Rc::new(RotateY::new(box2, -18.0));
    let box2 = Rc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    world.add(Rc::new(ConstantMedium::with_colour(
        box1,
        0.01,
        Colour::zero(),
    )));
    world.add(Rc::new(ConstantMedium::with_colour(
        box2,
        0.01,
        Colour::one(),
    )));

    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = 600;
    let samples_per_pixel: u32 = 200;
    let max_depth: u32 = 50;
    let background = Colour::zero();

    let vfov: f64 = 40.0;
    let look_from = Point3::new(278.0, 278.0, -800.0);
    let look_at = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
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
        background,
    );

    cam.render(&world);
}

fn final_scene(image_width: u32, samples_per_pixel: u32, max_depth: u32) {
    let mut boxes1 = HittableList::empty();

    let ground = Rc::new(Lambertian::from_colour(Colour::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;

    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Quad::block(
                &Point3::new(x0, y0, z0),
                &Point3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }

    let mut world = HittableList::empty();

    world.add(Rc::new(BVHNode::from_list(boxes1)));

    let light = Rc::new(DiffuseLight::from_colour(Colour::new(7.0, 7.0, 7.0)));
    world.add(Rc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    )));

    let centre1 = Point3::new(400.0, 400.0, 200.0);
    let centre2 = centre1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Rc::new(Lambertian::from_colour(Colour::new(0.7, 0.3, 0.1)));
    world.add(Rc::new(Sphere::new(
        centre1,
        centre2,
        50.0,
        sphere_material,
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Rc::new(Metal::new(Colour::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Rc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Rc::new(Dielectric::new(1.5)),
    ));

    world.add(boundary.clone());
    world.add(Rc::new(ConstantMedium::with_colour(
        boundary.clone(),
        0.2,
        Colour::new(0.2, 0.4, 0.9),
    )));
    let boundary = Rc::new(Sphere::new(
        Point3::zero(),
        Point3::zero(),
        5000.0,
        Rc::new(Dielectric::new(1.5)),
    ));
    world.add(Rc::new(ConstantMedium::with_colour(
        boundary.clone(),
        0.0001,
        Colour::one(),
    )));

    let emat = Rc::new(Lambertian::new(Rc::new(ImageTexture::new(
        RtwImage::new("earthmap.jpg").unwrap(),
    ))));
    world.add(Rc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = Rc::new(NoiseTexture::<256>::new(0.2));
    world.add(Rc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Rc::new(Lambertian::new(pertext)),
    )));

    let mut boxes2 = HittableList::empty();
    let white = Rc::new(Lambertian::from_colour(Colour::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        let pt = Point3::random_range(0.0, 165.0);
        boxes2.add(Rc::new(Sphere::new(pt, pt, 10.0, white.clone())));
    }

    world.add(Rc::new(Translate::new(
        Rc::new(RotateY::new(Rc::new(BVHNode::from_list(boxes2)), 15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    let aspect_ratio: f64 = 1.0;
    let background = Colour::zero();

    let vfov: f64 = 40.0;
    let look_from = Point3::new(478.0, 278.0, -600.0);
    let look_at = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
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
        background,
    );

    cam.render(&world);
}
