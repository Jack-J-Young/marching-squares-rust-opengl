use crate::{ Chunk, Mesh, Meshable, HashMap, SquareSet, WindowEvent, ElementState, VirtualKeyCode };

#[repr(C)]
#[derive(Clone)]
pub struct ReferencePoint {
    pub position: (f32, f32),
    pub render_dist: f32,
}

#[repr(C)]
#[derive(Clone)]
pub struct Plane {
    seed: i32,
    chunks: HashMap<(i32, i32), Chunk>
}

impl Plane {
    pub fn new() -> Self {
        Self {
            seed: 0,
            chunks: HashMap::new()
        }
    }

    pub fn get_chunk(&self, coord: (i32, i32)) -> Option<Chunk> {
        self.chunks.clone().get(&coord).cloned()
    }

    pub fn get_or_gen_chunk(&mut self, coord: (i32, i32)) -> Chunk {
        match self.get_chunk(coord) {
            Some(x) => return x,
            None => match self.generate(coord) {
                Some(x) => return x,
                None => (),
            },
        }

        Chunk::new(32)
    }

    pub fn set_chunk(&mut self, coord: (i32, i32), chunk: Chunk) -> Option<Chunk> {
        self.chunks.insert(coord, chunk)
    }

    fn generate(&mut self, coord: (i32, i32)) -> Option<Chunk> {
        self.set_chunk(coord, Chunk::new(32)) as Option<Chunk>
    }
        
    pub fn total_chunks(&self) -> usize {
        self.chunks.iter().len()
    }

    pub fn mesh_from_ref(&self, ref_point: &ReferencePoint) -> Mesh {
        let scale = 32.0;

        let mut meshes: Vec<Mesh> = vec![];
        
        let min_chunk_x = ((ref_point.position.1 - ref_point.render_dist)/scale).floor() as i32;
        let max_chunk_x = ((ref_point.position.1 + ref_point.render_dist)/scale).ceil() as i32;
        let min_chunk_y = ((ref_point.position.0 - ref_point.render_dist)/scale).floor() as i32;
        let max_chunk_y = ((ref_point.position.0 + ref_point.render_dist)/scale).ceil() as i32;

        println!("min: {0}, {1}", min_chunk_x, min_chunk_y);
        println!("max: {0}, {1}", max_chunk_x, max_chunk_y);

        for chunk_x in min_chunk_x..=max_chunk_x {
            for chunk_y in min_chunk_y..=max_chunk_y {
                if let Some(chunk) = self.get_chunk((chunk_x, chunk_y)) {
                    let mut edged_chunk = Chunk::new(33);
                    edged_chunk.add_data(&chunk.data, 0, 0);
                    match self.get_chunk((chunk_x + 1, chunk_y)) {
                        Some(v) => {edged_chunk.add_data(&v.get_edge(0), 32, 0);},
                        _ => {}
                    }
                    match self.get_chunk((chunk_x, chunk_y + 1)) {
                        Some(v) => {edged_chunk.add_data(&v.get_edge(1), 0, 32);},
                        _ => {}
                    }
                    match self.get_chunk((chunk_x + 1, chunk_y + 1)) {
                        Some(v) => {edged_chunk.add_data(&v.get_edge(2), 32, 32);},
                        _ => {}
                    }
                    let mut chunk_mesh = SquareSet::chunk_to_sets(&edged_chunk).to_mesh();
                    chunk_mesh.translate([chunk_y as f32, chunk_x as f32, 0.0]);
                    meshes.push(chunk_mesh);
                }
            }
        }

        let mut union: Mesh = Mesh { vertices: vec![], indices: vec![] };
        
        for mesh in meshes {
            union.union(&mesh);
        }
        println!("idk{0}, {1}", -(ref_point.position.0 as f32)/32.0, -(ref_point.position.1 as f32)/32.0);
        union.translate([-(ref_point.position.0 as f32)/scale, -(ref_point.position.1 as f32)/scale, 0.0]);

        union
    }

    pub fn clone_area(&mut self, chunk: &Chunk, start_x: i32, start_y: i32, end_x: i32, end_y: i32) {
        for chunk_x in start_x..=end_x {
            for chunk_y in start_y..=end_y {
                self.set_chunk((chunk_x, chunk_y), chunk.clone());
            }
        }
    }

    pub fn paint_antialiased_filled_circle(&mut self, x: f32, y: f32, radius: f32) -> &mut Self {
        let min_chunk_x = ((x - radius).floor() as i32)/32;
        let max_chunk_x = ((x + radius).ceil() as i32)/32;
        let min_chunk_y = ((y - radius).floor() as i32)/32;
        let max_chunk_y = ((y + radius).ceil() as i32)/32;
        
        for chunk_x in min_chunk_x..=max_chunk_x {
            for chunk_y in min_chunk_y..=max_chunk_y {
                let mut chunk = self.get_or_gen_chunk((chunk_x, chunk_y));
                let local_x = x - chunk_x as f32 * 32.0;
                let local_y = y - chunk_y as f32 * 32.0;
                chunk.paint_antialiased_filled_circle(local_x, local_y, radius);
                self.set_chunk((chunk_x, chunk_y), chunk);
            }
        }

        self
    }

}

impl Meshable for Plane where {
    fn to_mesh(&self) -> Mesh {
        let mut meshes: Vec<Mesh> = vec![];
        
        for ((x, y), chunk) in self.chunks.iter() {
            let mut edged_chunk = Chunk::new(33);
            edged_chunk.add_data(&chunk.data, 0, 0);
            match self.get_chunk((*x + 1, *y)) {
                Some(v) => {edged_chunk.add_data(&v.get_edge(0), 32, 0);},
                _ => {}
            }
            match self.get_chunk((*x, *y + 1)) {
                Some(v) => {edged_chunk.add_data(&v.get_edge(1), 0, 32);},
                _ => {}
            }
            match self.get_chunk((*x + 1, *y + 1)) {
                Some(v) => {edged_chunk.add_data(&v.get_edge(2), 32, 32);},
                _ => {}
            }
            let mut chunk_mesh = SquareSet::chunk_to_sets(&edged_chunk).to_mesh();
            chunk_mesh.translate([*y as f32, *x as f32, 0.0]);
            meshes.push(chunk_mesh);
        }

        let mut union: Mesh = Mesh { vertices: vec![], indices: vec![] };
        
        for mesh in meshes {
            union.union(&mesh);
        }

        union
    }
}
