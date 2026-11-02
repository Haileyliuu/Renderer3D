use glam::{Mat3, Mat4, Vec3};
use crate::{material::Material, ray::Ray, shape::{Hit, Shape}};

pub struct PolyMesh {
    vertices: Vec<Vec3>,            // the vertex table: one point per vertex
    triangles: Vec<[u32; 3]>,       // the index table: three vertex indices per face
    normals: Vec<Vec3>,             // optional: one per vertex (same length as vertices)
    transform: Mat4,
    material: Material
}

impl PolyMesh {
     pub fn new(vertices: Vec<Vec3>, triangles: Vec<[u32; 3]>, normals: Vec<Vec3>, material: Material) -> PolyMesh {
        PolyMesh {vertices, triangles, normals, transform: Mat4::IDENTITY, material}
     }

     // gets index of triangle (t) and returns 3 vertices of triangle
     fn corners(&self, t: usize) -> [Vec3; 3] {
        let [i, j, k] = self.triangles[t];
        [self.vertices[i as usize], self.vertices[j as usize], self.vertices[k as usize]] 
     }

     fn hit_triangle(a: Vec3, b: Vec3, c: Vec3, ray: &Ray, t_min: f32, t_max: f32) -> Option<(f32, f32, f32, Vec3)> {
        let d = ray.get_direction();
        let e = ray.get_origin(); 

        // hit point for triangle: p = (1 - beta - gamma)a + beta b + gamma c
        // rearrange with ray equation (p = e + t*d): (a - b) beta + (a - c) gamma + d t = a - e
        // c0 * beta + c1 * gamma + d * t = rhs
        let (c0, c1) = (a - b, a - c); // the two columns
        let rhs = a - e; // right-hand side

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

        let face_normal = (b - a).cross(c - a).normalize();

        Some((t, beta, gamma, face_normal))
    }

    // return shading normal from 
    fn shading_normal(&self, tri: usize, beta: f32, gamma: f32, face_normal: Vec3) -> Vec3 {
        // flat shading
        if self.normals.is_empty() {
            return face_normal;
        }

        let [i, j, k] = self.triangles[tri];
        let alpha = 1.0 - beta - gamma;
        let n = alpha * self.normals[i as usize]
                + beta * self.normals[j as usize]
                + gamma * self.normals[k as usize];
        n.normalize()   // the blend is < 1 long
    }




}

impl Shape for PolyMesh {
    fn transform(&mut self, m: Mat4) {
        self.transform = m * self.transform;
    }

    fn get_material(&self) -> Material {
        self.material
    }

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let d = ray.get_direction();
        let e = ray.get_origin(); 
        
        let mut best: Option<Hit> = None;
        let mut closest = t_max;

        for i in 0..self.triangles.len() {
            let vertices = self.corners(i);
            let (a, b, c) = (vertices[0], vertices[1], vertices[2]);
            if let Some((t, beta, gamma, n)) = Self::hit_triangle(a, b, c, ray, t_min, closest) {
                closest = t;
                let normal = self.shading_normal(i, beta, gamma, n);
                best = Some(Hit {t , p: e + t * d, n: normal})
            }
        }
        best
    }
}