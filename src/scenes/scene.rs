use glam::Vec3;
use crate::{camera::Camera, material::{Light}, shape::Shape};

pub struct Scene {
    pub cameras: Vec<Camera>,
    pub shapes: Vec<Box<dyn Shape>>,
    pub lights: Vec<Light>,
}