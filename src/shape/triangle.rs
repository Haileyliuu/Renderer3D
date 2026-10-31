use glam::{Vec3, Mat3};
use crate::{ray::Ray, shape::Hit};

pub struct Triangle {
    a: Vec3, 
    b: Vec3,
    c: Vec3
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Triangle {
        Triangle {a, b, c}
    }

    // if ray intersects with triangle, return nearest hit between t_min and t_max
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let d = ray.get_direction();
        let e = ray.get_origin(); 

        // hit point for triangle: p = (1 - beta - gamma)a + beta b + gamma c
        // rearrange with ray equation (p = e + t*d): (a - b) beta + (a - c) gamma + d t = a - e
        // c0 * beta + c1 * gamma + d * t = rhs
        let (c0, c1) = (self.a - self.b, self.a - self.c); // the two columns
        let rhs = self.a - e; // right-hand side

        // determinant of | a-b   a-c   d |
        let det = Mat3::from_cols(c0, c1, d).determinant();
        if det.abs() < 1e-8 {return None;}  // ray parallel to the plane

        let t = Mat3::from_cols(c0, c1, rhs).determinant() / det;
        if t<t_min || t>t_max {
            return None;    // too close or too far
        }

        let gamma = Mat3::from_cols(c0, rhs, d).determinant() / det;
        if gamma<0.0 || gamma>1.0 {
            return None;    // point not inside triangle (gamma not between 0, 1)
        }

        let beta = Mat3::from_cols(rhs, c1, d).determinant() / det;
        if beta < 0.0 || beta > 1.0 - gamma {
            return None;    // alpha = 1 - beta - gamma
                            // For point inside triangle: beta >=0, gamme >= 0, beta + gamma <= 1
        }

        let n = {self.b - self.a}.cross(self.c - self.a).normalize();

        Some(Hit{t, p: e + t * d, n})
    }

}