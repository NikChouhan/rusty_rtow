mod vec;
mod ray;
mod sphere;
mod hit;
mod camera;

use std::{fs::File, io::{BufWriter, Write}};
use rand::Rng;
use vec::{Vec3, Point3, Color};
use ray::Ray;
use hit::{Hit,World};
use sphere::Sphere;
use camera::Camera;


fn ray_color(r: &Ray, world: &World) -> Color {

    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        0.5*(rec.normal + Color::new(1.0, 1.0, 1.0))
    }
    else {
        let t = 0.5 * (r.direction().normalised().y() +1.0);
        let color : Color = (1.0-t) * Color::new(1.0, 1.0, 1.0) + (t)*Color::new(0.5, 0.7, 1.0);
        return color;
    }
}
fn main(){ 

    //Image
    const ASPECT_RATIO : f64 = 1.0;
    const IMAGE_WIDTH : u64 = 400;
    const IMAGE_HEIGHT : u64 = (IMAGE_WIDTH as f64/ASPECT_RATIO) as u64;

    const SAMPLES_PER_PIXEL: u64 = 100;

    //World

    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    //Camera
    let cam = Camera::new();
    let mut rng = rand::thread_rng();

    let file = File::create("image.ppm").expect("unable to create file");
    //Render

    let mut file = BufWriter::new(file);

    write!(file,"P3\n{} {} \n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).expect("unable to write");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();
                let u = i as f64 + random_u;
                let v = j as f64 + random_v;
                let r = cam.get_ray(u,v);

                pixel_color += ray_color(&r, &world);
            }
            write!(file, "{} \n", pixel_color.format_color(SAMPLES_PER_PIXEL)).expect("Error in writing to the last line");
            
        }
    }  
    
}
