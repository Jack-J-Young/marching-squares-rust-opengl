#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub colour: [f32; 3]
}

impl Vertex {
    pub fn print(&self) {
        println!("p: [{0},{1},{2}]\nc:[{3},{4},{5}]", self.position[0], self.position[1], self.position[2], self.colour[0], self.colour[1], self.colour[2]);
    }

    pub fn scale(&mut self, factor: f32) -> &mut Self {
        self.position = self.position.map(|x| x * factor);

        self
    }

    pub fn translate(&mut self, translation: [f32; 3]) -> &mut Self {
        self.position = [
            self.position[0] + translation[0],
            self.position[1] + translation[1],
            self.position[2] + translation[2],
        ];

        self
    }

    pub fn transform(&mut self, transformation: [f32; 3]) -> &mut Self {
        self.position = [
            self.position[0] * transformation[0],
            self.position[1] * transformation[1],
            self.position[2] * transformation[2],
        ];

        self
    }

    pub fn rotate90(&mut self, _counter_clock: bool) -> &mut Self {
        self.position = [
            self.position[1],
            -self.position[0],
            self.position[2],
        ];

        self
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<i16>
}

impl Mesh {
    pub fn print(&self) {
        for vertex in self.vertices.iter() {
            vertex.print();
        }

        for indices in self.indices.chunks(3) {
            println!("{0}, {1}, {2}", indices[0], indices[1], indices[2]);
        }
    }

    pub fn scale(&mut self, factor: f32) -> &mut Self {
        self.vertices.iter_mut().for_each(|x| {x.scale(factor);});

        self
    }

    pub fn translate(&mut self, translation: [f32; 3]) -> &mut Self {
        self.vertices.iter_mut().for_each(|x| {x.translate(translation);});

        self
    }

    pub fn transform(&mut self, transformation: [f32; 3]) -> &mut Self {
        self.vertices.iter_mut().for_each(|x| {x.transform(transformation);});

        self
    }

    pub fn rotate90(&mut self, counter_clock: bool) -> &mut Self {
        self.vertices.iter_mut().for_each(|x| {x.rotate90(counter_clock);});

        self
    }

    pub fn union(&mut self, mesh: &Mesh) -> &mut Self {
        let index_offset = self.vertices.len() as i16;
        self.vertices.extend(&mesh.vertices);
        for index in &mesh.indices {
            self.indices.push(index + index_offset);
        }

        self
        //let index_offset = self.vertices.len() as i16;
        //self.vertices.extend(&mesh.vertices);
        //for index in &mesh.indices {
        //    self.indices.push(index + index_offset);
        //}
        //
        //self
    }
}

pub trait Meshable {
    fn to_mesh(&self) -> Mesh;
}
