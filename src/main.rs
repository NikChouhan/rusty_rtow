mod vec;
mod ray;
mod sphere;
mod hit;

use std::{fs::File, io::{BufWriter, Write}};
use vec::{Vec3, Point3, Color};
use ray::Ray;
use hit::{Hit,World};
use sphere::Sphere;


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

    //World

    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    //Camera

    const VIEWPORT_HEIGHT : f64 = 2.0;
    const VIEWPORT_WIDTH : f64 = VIEWPORT_HEIGHT * ((IMAGE_WIDTH/IMAGE_HEIGHT) as f64);
    let camera_center  = Point3::new(0.0, 0.0, 0.0);
    let focal_length = 1.0;
    
    let width_ratio = VIEWPORT_WIDTH/((IMAGE_WIDTH-1) as f64);
    let height_ratio = VIEWPORT_HEIGHT/((IMAGE_HEIGHT-1) as f64);

    let bottom_left_corner = camera_center - Vec3::new(VIEWPORT_WIDTH/2.0, VIEWPORT_HEIGHT/2.0, focal_length);


    let file = File::create("image.ppm").expect("unable to create file");
    //Render

    let mut file = BufWriter::new(file);

    write!(file,"P3\n{} {} \n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).expect("unable to write");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let dir = bottom_left_corner +Vec3::new(width_ratio*(i as f64), height_ratio*(j as f64), 0.0);
            let r = Ray::new(camera_center, dir);

            let pixel_color = ray_color(&r, &world);

            write!(file, "{} \n", pixel_color.format_color()).expect("Error in writing to the last line");
            
        }
    }  
    
}
