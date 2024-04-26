use super::vec::{Vec3, Point3, Color};
use super::ray::Ray;

use super::hit::{Hit, World};

pub struct Camera {
    pub origin: Point3,
    pub bottom_left_corner: Vec3,
    pub width_ratio: f64,
    pub height_ratio: f64
}



impl Camera {
    pub fn new() -> Camera {

        const ASPECT_RATIO : f64 = 1.0;
        const IMAGE_WIDTH : u64 = 400;
        const IMAGE_HEIGHT : u64 = (IMAGE_WIDTH as f64/ASPECT_RATIO) as u64;

        const VIEWPORT_HEIGHT : f64 = 2.0;
        const VIEWPORT_WIDTH : f64 = VIEWPORT_HEIGHT * ((IMAGE_WIDTH/IMAGE_HEIGHT) as f64);
        let camera_center  = Point3::new(0.0, 0.0, 0.0);
        let focal_length = 1.0;
        
        let w_r = VIEWPORT_WIDTH/((IMAGE_WIDTH-1) as f64);
        let h_r = VIEWPORT_HEIGHT/((IMAGE_HEIGHT-1) as f64);

        let blc = camera_center - Vec3::new(VIEWPORT_WIDTH/2.0, VIEWPORT_HEIGHT/2.0, focal_length);

        Camera {
            origin: camera_center,
            bottom_left_corner: blc,
            width_ratio: w_r,
            height_ratio: h_r
        }
    }


    pub fn ray_color(r: &Ray, world: &World) -> Color {

        if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
            return 0.5*(rec.normal + Color::new(1.0, 1.0, 1.0));
        }
        else {
            let t = 0.5 * (r.direction().normalised().y() +1.0);
            let color : Color = (1.0-t) * Color::new(1.0, 1.0, 1.0) + (t)*Color::new(0.5, 0.7, 1.0);
            return color;
        }
    }
    pub fn get_ray(&self, u: f64,v: f64 ) -> Ray {
        Ray::new(self.origin,self.bottom_left_corner +Vec3::new(self.width_ratio*u, self.height_ratio *v, 0.0))
    }
}