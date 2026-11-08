use std::fs;
use glam::Vec3;
use crate::shape::polymesh::PolyMesh;

pub fn load_obj_file(path: &str) -> Result<PolyMesh, std::io::Error> {
    let text = fs::read_to_string(path)?;
    Ok(load_obj(&text))
}

pub fn load_obj(text: &str) -> PolyMesh {
    let mut mesh = PolyMesh::default();
    for line in text.lines() {
        // split line into words at whitespace
        let mut w = line.split_whitespace();
        // go through words in line
        match w.next() {
            // if first word is v (v -1.0 0.0 1.0)
            Some("v") => {
                let p: Vec<f32> = w.map(|s| s.parse().unwrap()).collect();
                mesh.vertices.push(Vec3::new(p[0], p[1], p[2]));
            }
            // if first word is vn (vn 0.0 0.8 0.6)
            Some("vn") => {
                let n: Vec<f32> = w.map(|s| s.parse().unwrap()).collect();
                mesh.normals.push(Vec3::new(n[0], n[1], n[2]));
            }
            // if first word is f (f 1/1/1 2/2/1 5/3/1) vertex/texcoord/normal
            Some("f") => {
                let mut v_index: Vec<u32> = Vec::new(); // vertex indices (f 1/1/1 2/2/1 5/3/1) -> [0, 1, 4]
                let mut vn_index: Vec<u32> = Vec::new(); // vertex normal indices (f 1/1/1 2/2/1 5/3/1) -> [0, 0, 0]

                for s in w {
                    let mut parts = s.split('/');

                    v_index.push(parts.next().unwrap() // "7/3/2" -> "7"
                        .parse::<u32>().unwrap()-1); // 1-based -> 0-based

                    parts.next(); // skip texture coordinate index

                    vn_index.push(parts.next().unwrap() // "7/3/2" -> "2"
                        .parse::<u32>().unwrap()-1); // 1-based -> 0-based
                }

                // fan: polygon -> triangle 
                for i in 1..v_index.len() - 1 {
                    mesh.triangles.push([v_index[0], v_index[i], v_index[i+1]]);
                    mesh.normal_indices.push([vn_index[0], vn_index[i], vn_index[i + 1]]);
                }
            }
            _ => {} // vt, #, g, usemtl ... ignored for now
        }
    }
    mesh
}