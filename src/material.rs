use std::cmp::max_by;

use glam::Vec3;

use crate::{camera::Camera, film::Color, shape::Hit};

#[derive(Clone, Copy)]
pub struct Material {
    k_d: Vec3,      // diffuse color
    k_s: Vec3,      // specular color
    p: f32,         // Phong exponent
    k_m: Vec3,      // mirror coefficient (0 = not a mirror)
}

impl Material {
    pub fn new(k_d: Vec3, k_s: Vec3, p: f32, k_m: Vec3) -> Material{
        Material { k_d, k_s, p, k_m }
    }

    // uses Blinn Phong shading to shade pixel 
    pub fn shade_pixel(&self, camera: &Camera, hit: &Hit, lights: &[Light]) -> Color {
        // Blinn Phong: L = ka Ia + sum(kd I max(0, n · l) + ks I max(0, n · h)^p)
        // Get p, n, and the material: ka, kd, ks, p
        
        // normalize n
        let n = hit.n.normalize();
        
        // compute v, the unit vector from p toward the eye
        let v = camera.eye - hit.p.normalize();

        // first term ambient: ka Ia 
        // ka = kd; Ia = a small constant, light from everywhere
        let ia = Vec3::new(0.1, 0.1, 0.1);
        let ambient = self.k_d * ia;

        // for each light in the scene
        // sum(kd I max(0, n · l) + ks I max(0, n · h)^p)
        let mut L : Vec3 = ambient;
        for light in lights {
            // l points from hit point to light
            let l = (light.position - hit.p).normalize();
            
            let diffuse = self.k_d * light.intensity * n.dot(l).max(0.0);

            // find h: the half vector between l and v
            let h = (v + l).normalize();
            let specular = self.k_s * light.intensity * n.dot(h).max(0.0).powf(self.p);

            L += diffuse + specular;
        }
        
        // return the color of the pixel
        let colorL = Color::new(L.x, L.y, L.z, 1.0);
        return  colorL;

    }
}

pub struct Light {
    pub position: Vec3,
    pub intensity: Vec3, // RGB triple: has color and it's intensity
}

impl Light {
    pub fn new (position: Vec3, intensity: Vec3) -> Light{
        Light {position, intensity}
    }
}