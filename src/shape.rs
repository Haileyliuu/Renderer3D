pub mod sphere;
pub mod plane;
pub mod polymesh;

use glam::{Mat4, Vec2, Vec3};
use crate::{material::Material, ray::Ray};

pub trait Shape {
    // methods automatically public bc trait is pub
    
    // to store its translate, rotation, scale
    fn transform(&mut self, m: Mat4);

    fn get_material(&self) -> &Material;

    // returns nearest hit between tmin, tmax
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>; 

}

pub struct Hit {
    pub t: f32,     // ray parameter at the hit
    pub p: Vec3,    // hit point = e + t * d
    pub n: Vec3,     // unit surface normal at p
    pub uv: Option<Vec2>,
}

impl Hit {
    pub fn new(t: f32, p: Vec3, n: Vec3, uv: Option<Vec2>) -> Hit{
        Hit{t, p, n, uv}
    }

    pub fn get_t(&self) -> f32 {
        return self.t;
    }

    pub fn get_n(&self) -> Vec3 {
        return self.n;
    }
}