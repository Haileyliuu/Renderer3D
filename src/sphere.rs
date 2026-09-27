use crate::film::Color;
use crate::ray::Ray;
use glam::{Vec3};

pub struct Sphere {
    radius: f32,
    position: Vec3,
    color: Color
}

impl Sphere {
    pub fn new(radius: f32, position: Vec3, color: Color) -> Sphere {
        Sphere{radius, position, color}
    }

    // if ray intersects with sphere, return nearest point of intersection between t_min and t_max
    pub fn ray_intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<Vec3> {
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
        return Some(hit_point);

    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec3;

    fn assert_vec3_approx_eq(actual: Vec3, expected: Vec3) {
        let epsilon = 0.0001;

        assert!(
            (actual.x - expected.x).abs() < epsilon,
            "x: actual {}, expected {}",
            actual.x,
            expected.x
        );
        assert!(
            (actual.y - expected.y).abs() < epsilon,
            "y: actual {}, expected {}",
            actual.y,
            expected.y
        );
        assert!(
            (actual.z - expected.z).abs() < epsilon,
            "z: actual {}, expected {}",
            actual.z,
            expected.z
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
    fn intersection_returns_near_point() {
        let sphere = test_sphere();

        let ray = Ray::new(
            Vec3::ZERO,
            Vec3::new(0.0, 0.0, -1.0),
        );

        let result = sphere.ray_intersect(ray, 0.0, f32::INFINITY);

        assert!(result.is_some());

        let point = result.unwrap();

        // Ray intersects at t = 4 and t = 6.
        // The nearer point is (0, 0, -4).
        assert_vec3_approx_eq(point, Vec3::new(0.0, 0.0, -4.0));
    }

    #[test]
    fn ray_misses_sphere() {
        let sphere = test_sphere();

        let ray = Ray::new(
            Vec3::ZERO,
            Vec3::new(0.0, 1.0, 0.0),
        );

        let result = sphere.ray_intersect(ray, 0.0, f32::INFINITY);

        assert!(result.is_none());
    }

    #[test]
    fn tangent_ray_returns_tangent_point() {
        let sphere = test_sphere();

        let ray = Ray::new(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
        );

        let result = sphere.ray_intersect(ray, 0.0, f32::INFINITY);

        assert!(result.is_some());

        let point = result.unwrap();

        // The ray touches the sphere at exactly one point.
        assert_vec3_approx_eq(point, Vec3::new(1.0, 0.0, -5.0));
    }

    #[test]
    fn t_min_excludes_near_intersection_and_returns_far_point() {
        let sphere = test_sphere();

        let ray = Ray::new(
            Vec3::ZERO,
            Vec3::new(0.0, 0.0, -1.0),
        );

        // Near intersection is t = 4.
        // Since t_min is 5, the far intersection at t = 6 is returned.
        let result = sphere.ray_intersect(ray, 5.0, f32::INFINITY);

        assert!(result.is_some());

        let point = result.unwrap();

        assert_vec3_approx_eq(point, Vec3::new(0.0, 0.0, -6.0));
    }

    #[test]
    fn t_max_can_exclude_all_intersections() {
        let sphere = test_sphere();

        let ray = Ray::new(
            Vec3::ZERO,
            Vec3::new(0.0, 0.0, -1.0),
        );

        // Intersections are at t = 4 and t = 6.
        let result = sphere.ray_intersect(ray, 0.0, 3.0);

        assert!(result.is_none());
    }
}