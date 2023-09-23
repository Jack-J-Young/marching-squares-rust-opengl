pub struct Mesh (
    vertices: &[Vertex],
    indices: &[i16]
)

impl Mesh {
    fn scale(&self, factor: f32) {
        self.position.map(|x| x.mult(factor));
    }

    fn translate(&self, translation: (f32, f32)) {
        self.position.map(|x| x.translation(translation));
    }

    fn union(&self, mesh: Mesh) {
       self.vertices.extend(mesh.vertices);
       let index_offset = vertices.len() as i16;
       mesh.indices.map(|x| x + index_offset);
       self.indices.extend(mesh.vertices);
    }
}

pub struct Vertex (
    position: [f32, f32, f32],
    colour: [f32, f32, f32]
)

impl Vertex {
    fn scale(&self, factor: f32) {
        self.position[0] *= factor;
        self.position[1] *= factor;
    }

    fn translate(&self, translation: (f32, f32)) {
        self.position[0] += translation.0;
        self.position[1] += translation.1;
    }
}
