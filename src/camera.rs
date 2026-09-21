use crate::ray::Ray;    // crate::module(file name)::Struct
use glam::{Vec3};

struct Camera {
    eye: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    viewport_h: f32,
    viewport_w: f32
}

impl Camera {
    fn new(eye: Vec3, target: Vec3, aspect: f32, fov_deg: f32, up: Vec3) -> Camera {
        let theta = fov_deg.to_radians();
        let h = (theta * 0.5).tan();
        let viewport_h = 2.0 * h;                   // view-plane height at d = 1
        let viewport_w = aspect * viewport_h;       // and its width
        
        // orthonormal basis (right-handed; the camera looks down -w)
        let w = (eye - target).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        Camera {eye, u, v, w, viewport_h, viewport_w}
    }


    // returns camera ray given normalized (s, t) on a viewplane
    // (s, t) is a pixel on plane
    // find ray from eye of camera to pixel
    fn get_ray(&self, s: f32, t: f32) -> Ray {  
        // M =  eye + dw    (d = 1)
        let m = self.eye - self.w;

        // lower_left = M - (h/2 * v) - (w/2 * u)
        let lower_left = m - (self.viewport_h/2.0 * self.v) - (self.viewport_w/2.0 * self.u);

        // P = lower_left + s·horizontal + t·vertical
        // (horizontal = u*width; vertical = v*height)
        let p = lower_left + (s * (self.u*self.viewport_w)) + (t * (self.v*self.viewport_h));
        
        // origin = the eye
        // direction = P − eye
        Ray::new(self.eye, p - self.eye)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec3;

    fn assert_vec3_near(actual: Vec3, expected: Vec3) {
        let epsilon = 0.00001;
        assert!(
            actual.abs_diff_eq(expected, epsilon),
            "expected {:?}, got {:?}",
            expected,
            actual
        );
    }

    fn test_camera() -> Camera {
        Camera::new(
            Vec3::ZERO,                 // eye
            Vec3::new(0.0, 0.0, -1.0),  // target
            1.0,                        // aspect ratio
            90.0,                       // field of view
            Vec3::new(0.0, 1.0, 0.0),   // up
        )
    }

    #[test]
    fn camera_basis_vectors_are_correct() {
        let camera = test_camera();

        assert_vec3_near(camera.u, Vec3::new(1.0, 0.0, 0.0));
        assert_vec3_near(camera.v, Vec3::new(0.0, 1.0, 0.0));
        assert_vec3_near(camera.w, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn viewport_dimensions_are_correct() {
        let camera = test_camera();

        // For a 90-degree FOV:
        // viewport_h = 2 * tan(45 degrees) = 2
        // viewport_w = aspect * viewport_h = 1 * 2 = 2
        assert!((camera.viewport_h - 2.0).abs() < 0.00001);
        assert!((camera.viewport_w - 2.0).abs() < 0.00001);
    }

    #[test]
    fn rays_hit_each_viewplane_corner() {
        let camera = test_camera();

        let test_cases = [
            // (s, t, expected direction)
            (
                0.0,
                0.0,
                Vec3::new(-1.0, -1.0, -1.0), // lower-left
            ),
            (
                1.0,
                0.0,
                Vec3::new(1.0, -1.0, -1.0), // lower-right
            ),
            (
                0.0,
                1.0,
                Vec3::new(-1.0, 1.0, -1.0), // upper-left
            ),
            (
                1.0,
                1.0,
                Vec3::new(1.0, 1.0, -1.0), // upper-right
            ),
        ];

        for (s, t, expected_direction) in test_cases {
            let ray = camera.get_ray(s, t);

            assert_vec3_near(ray.get_origin(), Vec3::ZERO);
            assert_vec3_near(ray.get_direction(), expected_direction);
        }
    }

    #[test]
    fn ray_through_viewplane_center_points_forward() {
        let camera = test_camera();

        let ray = camera.get_ray(0.5, 0.5);

        assert_vec3_near(ray.get_origin(), Vec3::ZERO);
        assert_vec3_near(
            ray.get_direction(),
            Vec3::new(0.0, 0.0, -1.0),
        );
    }
}