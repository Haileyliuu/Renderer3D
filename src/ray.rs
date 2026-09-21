use core::fmt;
use glam::{Mat4, Vec3};

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {origin, direction}
    }

    pub fn get_origin(&self) -> Vec3 {
        self.origin
    }

    pub fn get_direction(&self) -> Vec3 {
        self.direction
    }

    pub fn find_point(&self, t: f32) -> Vec3 {
        // P(t) = O + tD
        self.origin + (t * self.direction)
    }

    pub fn transform(&mut self, m: Mat4) {
        self.origin = m.transform_point3(self.origin);
        self.direction = m.transform_vector3(self.direction);
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "origin: {}, direction: {}", self.origin, self.direction)
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec4;

    use super::*;

     #[test]
    fn new_ray_test() {
        let r = Ray::new (
            Vec3::new(1.0, 2.0, 3.0), 
            Vec3::new(0.0, 0.0, 1.0)
        );

        assert_eq!(Vec3::new(1.0, 2.0, 3.0), r.origin);
        assert_eq!(Vec3::new(0.0, 0.0, 1.0), r.direction);
    }

    #[test]
    fn get_origin_test() {
        let r = Ray {
            origin: Vec3::new(1.0, 2.0, 3.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };

        assert_eq!(r.origin, r.get_origin());
    }

    #[test]
    fn get_direction_test() {
        let r = Ray {
            origin: Vec3::new(1.0, 2.0, 3.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };

        assert_eq!(r.direction, r.get_direction());
    }

    #[test]
    fn find_point_test() {
        let mut r = Ray {
            origin: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };

        assert_eq!(Vec3::new(0.0, 0.0, 3.0), r.find_point(3.0));
    }

    #[test]
    fn transform_translate_test() {
        let mut r = Ray {
            origin: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };

        let m = Mat4::from_cols(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(3.0, 1.0, 2.0, 1.0),
        );

        r.transform(m);

        assert_eq!(Vec3::new(3.0, 1.0, 2.0), r.get_origin());
        assert_eq!(Vec3::new(0.0, 0.0, 1.0), r.get_direction());
    }

    #[test]
    fn transform_scale_test() {
        let mut r = Ray {
            origin: Vec3::new(1.0, 1.0, 1.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };

        let m = Mat4::from_cols(
            Vec4::new(2.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 3.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 5.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        );

        r.transform(m);

        assert_eq!(Vec3::new(2.0, 3.0, 5.0), r.get_origin());
        assert_eq!(Vec3::new(0.0, 0.0, 5.0), r.get_direction());
    }
}
