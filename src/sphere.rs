use super::vec::{Point3, Vec3};
use super::ray::Ray;
use super::hit::{Hit, HitRecord};


pub struct Sphere {
    center: Point3,
    radius: f64
}

impl Sphere {
    pub fn new(cen: Point3, rad: f64) -> Sphere {
        Sphere {center: cen, radius: rad}
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;      //because oc is vector from center to origin half_b formula is used as it is, else half_b would be replaced by -half_b
        let a = r.direction().dot(r.direction());
        let half_b = oc.dot(r.direction());
        let c = oc.length().powi(2) - self.radius.powi(2);
        
        let discriminant = half_b*half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root that lies in the acceptable range
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = r.at(root);
        let mut rec = HitRecord {
            t: root,
            p: p,
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false
        };

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }
    
}
