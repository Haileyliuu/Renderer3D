use crate::{material::Material, ray::Ray, shape::{Hit, Shape}};
use glam::{Mat4, Vec3};

pub struct Plane {
    a: Vec3,    // point on the plane
    n: Vec3,    // normal vector
    transform: Mat4,
    material: Material
}

impl Plane {
    pub fn new(a: Vec3, n: Vec3, material: Material) -> Plane {
        Plane {a, n, transform: Mat4::IDENTITY, material}
    }
}

impl Shape for Plane {
    fn transform(&mut self, m: Mat4) {
        self.transform = m * self.transform;
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    // if ray intersects with plane, return hit between t_min and t_max
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        // put p = e + td into plane equation (p − a) * n = 0
        // gets t = (a - e) * n / (d*n) where t is where it intersects on ray

        let d = ray.get_direction();
        let e = ray.get_origin(); 

        let denom = d.dot(self.n);  // denominator (d*n)
        if denom.abs() < 1e-6 {     // parallel to plane
            return None;
        }

        let t = (self.a - e).dot(self.n) / denom;
        if t < t_min || t > t_max {     // not in range, behind us or too far
            return None;
        }

        // flip normal vector if pointing in same direction of ray
        let n = if denom < 0.0 {self.n} else {-self.n};  // face the ray
        Some(Hit {t, p: e + t*d, n, uv: None})
    }
}