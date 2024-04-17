mod vec;
mod ray;

use std::{fs::File,io::{BufWriter, Write}};
use vec::{Vec3, Point3, Color};
use ray::Ray;

fn ray_color(r: &Ray) -> Color {
    let t = 0.5 * (r.direction().normalised().y() +1.0);
    let color : Color = (t) * Color::new(1.0, 1.0, 1.0) + (1.0-t)*Color::new(0.5, 0.7, 1.0);
    return color;
}

fn main(){ 
    const ASPECT_RATIO : f64 = 16.0 / 9.0;
    const IMAGE_WIDTH : u64 = 400;
    const IMAGE_HEIGHT : u64 = (IMAGE_WIDTH as f64/ASPECT_RATIO) as u64;

    //Camera

    const VIEWPORT_HEIGHT : f64 = 2.0;
    const VIEWPORT_WIDTH : f64 = VIEWPORT_HEIGHT * ((IMAGE_WIDTH/IMAGE_HEIGHT) as f64);
    let camera_center  = Point3::new(0.0, 0.0, 0.0);
    let focal_length = 1.0;
    
    let width_ratio = VIEWPORT_WIDTH/(IMAGE_WIDTH as f64);
    let height_ratio = VIEWPORT_HEIGHT/(IMAGE_HEIGHT as f64);

    let botom_left_corner = camera_center - Vec3::new(VIEWPORT_WIDTH/2.0, VIEWPORT_HEIGHT/2.0, focal_length);


    let file = File::create("image.ppm").expect("unable to create file");
    //Render

    let mut file = BufWriter::new(file);

    write!(file,"P3\n{} {} \n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).expect("unable to write");

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let dir = botom_left_corner +Vec3::new(width_ratio*(i as f64), height_ratio*(j as f64), 0.0);
            let r = Ray::new(camera_center, dir);

            let pixel_color = ray_color(&r);

            write!(file, "{} \n", pixel_color.format_color()).expect("Error in writing to the last line");
            
        }
    }  
    
}
