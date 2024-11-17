use tobj;
use nalgebra_glm::{Vec2, Vec3};
use crate::vertex::Vertex;

pub struct Obj {
    meshes: Vec<Mesh>,
}

struct Mesh {
    vertices: Vec<Vec3>,
    normals: Vec<Vec3>,
    texcoords: Vec<Vec2>,
    indices: Vec<u32>,
}

impl Obj {
    pub fn load(filename: &str) -> Result<Self, tobj::LoadError> {
        let (models, _) = tobj::load_obj(filename, &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        })?;

        let meshes = models.into_iter().map(|model| {
            let mesh = model.mesh;

            // Cargar posiciones de vértices
            let vertices: Vec<Vec3> = mesh.positions.chunks(3)
                .map(|v| Vec3::new(v[0], v[1], v[2]))
                .collect();

            // Cargar normales o generarlas si están ausentes
            let normals: Vec<Vec3> = if !mesh.normals.is_empty() {
                mesh.normals.chunks(3)
                    .map(|n| Vec3::new(n[0], n[1], n[2]))
                    .collect()
            } else {
                // Generar normales calculadas
                let mut generated_normals = vec![Vec3::new(0.0, 0.0, 0.0); vertices.len()];
                for indices in mesh.indices.chunks(3) {
                    let i0 = indices[0] as usize;
                    let i1 = indices[1] as usize;
                    let i2 = indices[2] as usize;

                    let v0 = vertices[i0];
                    let v1 = vertices[i1];
                    let v2 = vertices[i2];

                    // Calcular la normal del triángulo
                    let normal = (v1 - v0).cross(&(v2 - v0)).normalize();

                    // Asignar la normal a los tres vértices
                    generated_normals[i0] += normal;
                    generated_normals[i1] += normal;
                    generated_normals[i2] += normal;
                }

                // Normalizar todas las normales
                generated_normals.iter_mut().for_each(|n| *n = n.normalize());
                generated_normals
            };

            // Cargar coordenadas de textura
            let texcoords: Vec<Vec2> = mesh.texcoords.chunks(2)
                .map(|t| Vec2::new(t[0], 1.0 - t[1]))
                .collect();

            Mesh {
                vertices,
                normals,
                texcoords,
                indices: mesh.indices,
            }
        }).collect();

        Ok(Obj { meshes })
    }

    /// Genera un arreglo de vértices (`Vertex`) a partir de los datos cargados
    pub fn get_vertex_array(&self) -> Vec<Vertex> {
        let mut vertices = Vec::new();

        for mesh in &self.meshes {
            for &index in &mesh.indices {
                let position = mesh.vertices[index as usize];
                let normal = mesh.normals.get(index as usize)
                    .cloned()
                    .unwrap_or(Vec3::new(0.0, 1.0, 0.0));
                let tex_coords = mesh.texcoords.get(index as usize)
                    .cloned()
                    .unwrap_or(Vec2::new(0.0, 0.0));

                vertices.push(Vertex::new(position, normal, tex_coords));
            }
        }

        vertices
    }
}
