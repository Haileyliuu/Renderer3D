use glam::{Mat4, Vec3};
use crate::{material::Material, ray::Ray, shape::{Hit, Shape}};

pub struct PolyMesh {
    pub vertices: Vec<Vec3>,            // the vertex table: one point per vertex
    pub triangles: Vec<[u32; 3]>,       // the index table: three vertex indices per face
    pub normals: Vec<Vec3>,             // table of normals
    pub normal_indices: Vec<[u32; 3]>,  // index table for normals, matches vertices of triangles
    transform: Mat4,
    material: Material
}

impl PolyMesh {
     pub fn new(vertices: Vec<Vec3>, triangles: Vec<[u32; 3]>, normals: Vec<Vec3>, normal_indices: Vec<[u32; 3]>, material: Material) -> PolyMesh {
        PolyMesh {vertices, triangles, normals, normal_indices, transform: Mat4::IDENTITY, material}
     }

     pub fn default() -> PolyMesh {
        let light_gray = Material::new(Vec3::new(0.7, 0.7, 0.7), Vec3::new(0.1, 0.1, 0.1), 8.0);
        //let red = Material::new(Vec3::new(0.8, 0.0, 0.0), Vec3::new(0.3, 0.3, 0.3), 100.0);
        PolyMesh {vertices: Vec::<Vec3>::new(), triangles: Vec::<[u32; 3]>::new(), normals: Vec::<Vec3>::new(), 
                   normal_indices: Vec::<[u32; 3]>::new(), transform: Mat4::IDENTITY, material: light_gray}
     }

     // gets index of triangle (t) and returns 3 vertices of triangle
     fn corners(&self, t: usize) -> [Vec3; 3] {
        let [i, j, k] = self.triangles[t];
        [self.vertices[i as usize], self.vertices[j as usize], self.vertices[k as usize]] 
     }

     // if ray intersects with triangle, return nearest t and beta, gamma, face_normal between t_min and t_max
     fn hit_triangle(a: Vec3, b: Vec3, c: Vec3, ray: &Ray, t_min: f32, t_max: f32) -> Option<(f32, f32, f32, Vec3)> {
        let d = ray.get_direction();
        let e = ray.get_origin(); 

        // hit point for triangle: p = (1 - beta - gamma)a + beta b + gamma c
        // rearrange with ray equation (p = e + t*d): (b - a) beta + (c - a) gamma - d*t = e - a
        // e1 * beta + e2 * gamma - d * t = s
        let (e1, e2) = (b - a, c - a); // the two columns

        // determinant of | b-a   c-a   -d |
        let p_vec = d.cross(e2);
        let det = e1.dot(p_vec);
        if det.abs() < 1e-8 {return None;}  // ray parallel to the plane

        let inv = 1.0 / det;
        let s = e - a;
        let beta = s.dot(p_vec) * inv;
        if beta < 0.0 || beta > 1.0 {
            return None;    // point not inside triangle (beta not between 0, 1)
        }

        let q_vec = s.cross(e1);
        let gamma = d.dot(q_vec) * inv;
        if gamma < 0.0 || beta + gamma > 1.0 {
            return None;    // alpha = 1 - beta - gamma
                            // For point inside triangle: beta >=0, gamma >= 0, beta + gamma <= 1
        }

        let t = e2.dot(q_vec) * inv;
        if t<t_min || t>t_max {
            return None;    // too close or too far
        }

        let face_normal = (b - a).cross(c - a).normalize();

        Some((t, beta, gamma, face_normal))
    }

    // return shading normal from interpolizing across triangle 
    fn shading_normal(&self, tri: usize, beta: f32, gamma: f32, face_normal: Vec3) -> Vec3 {
        // flat shading
        if self.normals.is_empty() {
            return face_normal;
        }

        let [ni, nj, nk] = self.normal_indices[tri];

        let alpha = 1.0 - beta - gamma;
        let n = alpha * self.normals[ni as usize]
                + beta * self.normals[nj as usize]
                + gamma * self.normals[nk as usize];
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

    // return the closest hit where the ray hit the mesh
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let d = ray.get_direction();
        let e = ray.get_origin(); 
        
        let mut best: Option<Hit> = None;
        let mut closest = t_max;

        // go through triangles to find which one was closest hit 
        for i in 0..self.triangles.len() {
            let vertices = self.corners(i);
            let (a, b, c) = (vertices[0], vertices[1], vertices[2]);
            // if hit was closest, save t, get shading normal, and save hit 
            if let Some((t, beta, gamma, n)) = Self::hit_triangle(a, b, c, ray, t_min, closest) {
                closest = t;
                let normal = self.shading_normal(i, beta, gamma, n);
                best = Some(Hit {t , p: e + t * d, n: normal})
            }
        }
        best
    }
}