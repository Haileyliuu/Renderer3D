use std::f32::consts::PI;

use crate::{material::{Material}, ray::Ray, shape::{Hit, Shape}};
use glam::{Mat4, Vec2, Vec3};

pub struct Sphere {
    radius: f32,
    position: Vec3,
    transform: Mat4,
    material: Material
}

impl Sphere {
    pub fn new(radius: f32, position: Vec3, material: Material) -> Sphere {
        Sphere{radius, position, transform: Mat4::IDENTITY, material}
    }

    // from point on sphere get u, v coordinates
    fn sphere_uv(&self, point: Vec3) -> Vec2 {
        let p = (point - self.position) / self.radius;

        let u = 0.5 + p.z.atan2(p.x) / (2.0 * PI); // vertical (like longitude)
        let v = 0.5 + p.y.asin() / PI; // horizontal (like latitude)

        Vec2::new(u, v)
    }
}

impl Shape for Sphere {
    fn transform(&mut self, m: Mat4) {
        self.transform = m * self.transform;
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    // if ray intersects with sphere, return nearest hit between t_min and t_max
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        // put p = e + td into sphere equation (p − c) · (p − c) − R^2 = 0
        // gets a quadratic (A*t^2 + Bt + C = 0)
        
        let d = ray.get_direction();
        let e = ray.get_origin(); 
        

        let a = d.dot(d);    // =1 if d is normalized
        let b = (2.0*d).dot(e - self.position);
        let c = (e - self.position).dot(e-self.position) - (self.radius*self.radius);

        // discriminant = b^2 - 4ac
        let disc = b*b - 4.0*a*c;

        // 0 hits
        if disc < 0.0 {
            return None;
        } 

        let sqrt_d = disc.sqrt();

        // two roots of quadratic formula, smaller = near, bigger = far
        let mut t = (-b - sqrt_d) / (2.0 * a); // near root first
        
        if t < t_min || t > t_max { // else the far root (if not between t_min and t_max)
            t = (-b + sqrt_d) / (2.0 * a); 
        }

        if t < t_min || t > t_max { 
            return None; 
        }

        let hit_point = e + t * d;
        let uv = self.sphere_uv(hit_point);
        return Some(Hit::new(t, hit_point, (hit_point - self.position)/self.radius, Some(uv)));
    }
}


/* 

#[cfg(test)]
mod tests {
    use super::*;
    use crate::film::Color;
    use crate::ray::Ray;
    use glam::Vec3;

    fn assert_vec3_close(actual: Vec3, expected: Vec3) {
        assert!(
            actual.abs_diff_eq(expected, 0.00001),
            "expected {:?}, got {:?}",
            expected,
            actual
        );
    }

    fn test_sphere() -> Sphere {
        Sphere::new(
            1.0,
            Vec3::new(0.0, 0.0, -5.0),
            Color::new(1.0, 0.0, 0.0, 1.0),
        )
    }

    #[test]
    fn ray_hits_sphere_at_nearest_point() {
        let sphere = test_sphere();

        let ray = Ray::new(
            Vec3::ZERO,
            Vec3::new(0.0, 0.0, -1.0),
        );

        let hit = sphere
            .hit(&ray, 0.0, f32::INFINITY)
            .expect("Ray should hit sphere");

        // The ray enters the sphere at z = -4.
        assert!((hit.t - 4.0).abs() < 0.00001);

        assert_vec3_close(
            hit.p,
            Vec3::new(0.0, 0.0, -4.0),
        );

        // The outward normal at the front of the sphere points toward +z.
        assert_vec3_close(
            hit.n,
            Vec3::new(0.0, 0.0, 1.0),
        );
    }

    #[test]
    fn ray_misses_sphere() {
        let sphere = test_sphere();

        let ray = Ray::new(
            Vec3::ZERO,
            Vec3::new(0.0, 1.0, 0.0),
        );

        let hit = sphere.hit(&ray, 0.0, f32::INFINITY);

        assert!(hit.is_none());
    }

    #[test]
    fn tangent_ray_hits_sphere_once() {
        let sphere = test_sphere();

        let ray = Ray::new(
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
        );

        let hit = sphere
            .hit(&ray, 0.0, f32::INFINITY)
            .expect("Tangent ray should hit sphere");

        assert!((hit.t - 5.0).abs() < 0.00001);

        assert_vec3_close(
            hit.p,
            Vec3::new(0.0, 1.0, -5.0),
        );

        assert_vec3_close(
            hit.n,
            Vec3::new(0.0, 1.0, 0.0),
        );
    }

    #[test]
    fn ray_starting_inside_sphere_uses_far_intersection() {
        let sphere = Sphere::new(
            1.0,
            Vec3::ZERO,
            Color::new(1.0, 0.0, 0.0, 1.0),
        );

        let ray = Ray::new(
            Vec3::ZERO,
            Vec3::new(1.0, 0.0, 0.0),
        );

        let hit = sphere
            .hit(&ray, 0.0, f32::INFINITY)
            .expect("Ray should exit sphere");

        assert!((hit.t - 1.0).abs() < 0.00001);

        assert_vec3_close(
            hit.p,
            Vec3::new(1.0, 0.0, 0.0),
        );

        assert_vec3_close(
            hit.n,
            Vec3::new(1.0, 0.0, 0.0),
        );
    }

    #[test]
    fn t_range_can_select_far_intersection() {
        let sphere = test_sphere();

        let ray = Ray::new(
            Vec3::ZERO,
            Vec3::new(0.0, 0.0, -1.0),
        );

        // The intersections are t = 4 and t = 6.
        // Excluding t = 4 should return the far intersection.
        let hit = sphere
            .hit(&ray, 4.5, 6.5)
            .expect("Far intersection should be accepted");

        assert!((hit.t - 6.0).abs() < 0.00001);

        assert_vec3_close(
            hit.p,
            Vec3::new(0.0, 0.0, -6.0),
        );

        assert_vec3_close(
            hit.n,
            Vec3::new(0.0, 0.0, -1.0),
        );
    }

    #[test]
    fn intersections_outside_t_range_return_none() {
        let sphere = test_sphere();

        let ray = Ray::new(
            Vec3::ZERO,
            Vec3::new(0.0, 0.0, -1.0),
        );

        let hit = sphere.hit(&ray, 6.1, 10.0);

        assert!(hit.is_none());
    }
}*/