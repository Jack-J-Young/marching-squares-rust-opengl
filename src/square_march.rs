use crate::{ Chunk, Vertex, Mesh, Meshable, HashMap };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SquareSet {
    pub a: f32, pub b: f32,
    pub d: f32, pub c: f32,
}


impl SquareSet {
    pub fn chunk_to_sets(chunk: &Chunk) -> Vec<Self> {
        /*
        [
            abcd
            1234
            ijlk
            ..
        ]
            .windows(2).map(x0.windows(2).zip(x1.windows(2)))
        [
            [
                [ab][bc][cd]
                [12][23][34]
            ]
            [
                [12][23][34]
                [ij][jl][lk]
            ]
            ..
        ]
            .flat_map.map
        [   set  set  set  set  set
            [ab  [bc  [cd  [12  ..
        ]    12]  23]  34]  ij] 
        */

        chunk.data.windows(2)
            .map(|x| 
                 x[0].windows(2).zip(x[1].windows(2))
            ).flat_map(|x| {                // [[un_map][in_map]]      -> [map, map] 

                x.map(|y| Self {            // tuples                     set
                    a: y.0[0], b: y.0[1],   // ((f32, f32),(f32, f32)) -> ab
                    d: y.1[0], c: y.1[1]    //                            dc
                })
        }).collect()
    }

    fn rotate(&mut self, _counter_clock: bool) -> &mut Self {
        // Rotations:
        // 1)ab 2)da 3)cd 4)bc
        //   dc   cb   ba   ad
        
        let temp = self.a;
        self.a = self.d;
        self.d = self.c;
        self.c = self.b;
        self.b = temp;

        //let temp: f32 = self.a;
        //self.a = self.b;
        //self.b = self.c;
        //self.c = self.d;
        //self.d = temp;
        
        self
    }
}

impl Meshable for SquareSet where {
    fn to_mesh(&self) -> Mesh {
        let cutoff = 0.2;
        
        let mut rotation = self.clone();
        for i in 0..=3 {
            let output: Option<Mesh> = match rotation {
                _ if check_pattern_1(&rotation, cutoff) => Some(pattern_1(rotation)),
                _ if check_pattern_2(&rotation, cutoff) => Some(pattern_2(rotation)),

                _ if check_pattern_3(&rotation, cutoff) => Some(pattern_3(rotation)),
                _ if check_pattern_4(&rotation, cutoff) => Some(pattern_4(rotation)),
                _ if check_pattern_5(&rotation, cutoff) => Some(pattern_5(rotation)),
                _ => None
            };
        
            match output {
                Some(x) => {
                    let mut rotated_mesh: Mesh = x.clone();
                    
                    for _ in 0..i {
                        rotated_mesh.rotate90(false);
                    }
                    
                    return rotated_mesh
                },
                None => ()
            }
            
            rotation.rotate(false);
        }
        
        Mesh { vertices: vec![], indices: vec![] }
    }
}

impl Meshable for Vec<SquareSet> where {
    fn to_mesh(&self) -> Mesh {
        let size = /*(self.len() as f32).sqrt()*/32 as i32;
        let mut meshes: Vec<Mesh> = vec![];
        
        let mut i = 0;
        for set in self {
            let x = i%size;
            let y = i/size;

            let mut set_mesh = set.to_mesh();
            set_mesh.translate([x as f32, y as f32, 0.0]);
            meshes.push(set_mesh);
            
            i += 1;
        }

        // new process
        //
        // check if any vertices are new, if so add them to the list
        //
        // then go through the vertices and map them to the list to get indices

        let mut total_vertices: Vec<Vertex> = vec![];
        let mut indices: Vec<i16> = vec![];

        for mesh in meshes {
            let mut index_map: Vec<i16> = vec![];
            for vertex in &mesh.vertices {
                match total_vertices.iter().position(|x| x.position[0] == vertex.position[0]
                                                      && x.position[1] == vertex.position[1]
                                                      && x.position[2] == vertex.position[2]) {
                    Some (x) => index_map.push(x as i16),
                    None => {
                        index_map.push((total_vertices.len()) as i16);
                        total_vertices.push(vertex.clone());
                    }
                }

                //if total_vertices.iter().all(|x| !( x.position[0] == vertex.position[0] 
                //                          && x.position[1] == vertex.position[1]
                //                          && x.position[2] == vertex.position[2]
                //)) {
                //    total_vertices.push(*vertex);
                //}
            }
            for index in &mesh.indices {
                indices.push(index_map[*index as usize]);
            }
            //indices.append();
        }

            //for vertex in &mesh.vertices {
            //    match total_vertices.iter().position(|x| x.position[0] == vertex.position[0]
            //                                          && x.position[1] == vertex.position[1]
            //                                          && x.position[2] == vertex.position[2]) {
            //        Some (x) => indices.push(x as i16),
            //        None =>  ()
            //    }
            //}
            
            //for vertex_index in 0..mesh.vertices.len() {
            //    let vertex = mesh.vertices[vertex_index];
            //    let pos = (
            //        vertex.position[0],
            //        vertex.position[1],
            //        vertex.position[2],
            //    );
            //    match vertex_to_index.get(pos) {
            //        Some(index) => mesh.indices.where
            //    }
            //    mesh.vertices.push();
            //}
            //total.union(&mesh);
        
        let mut total: Mesh = Mesh {
            vertices: total_vertices,
            indices: indices
        };
        
        total.translate([0.5, 0.5, 0.0]);
        total.scale(1.0/32.0);

        total
    }
}

fn smooth(i: f32) -> f32 {
    (std::f32::consts::E).powf(-1.0/i)
}

fn side_fn(p1: f32, p2: f32) -> f32 {
            //1 => self.data[0].clone().iter().map(|&x| vec![x]).collect(),
            //0 => vec![
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //    vec![0.0],
            //],
            //1 => vec![ vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]],
    let i = (p1 + p2) / 2.0;
    let smooth_i = smooth(i);
    smooth_i / (smooth_i + smooth(1.0 - i))
    //0.5
}

// a b
// c d
//
// Patterns:
// 1)#. 2)## 3) #. 4)## 5)##
//   ..   ..    .#   #.   ##

fn check_pattern_1(set: &SquareSet, cutoff: f32) -> bool {
    return set.a >  cutoff
        && set.b <= cutoff
        && set.c <= cutoff
        && set.d <= cutoff
}

fn check_pattern_2(set: &SquareSet, cutoff: f32) -> bool {
    return set.a >  cutoff
        && set.b >  cutoff
        && set.c <= cutoff
        && set.d <= cutoff
}

fn check_pattern_3(set: &SquareSet, cutoff: f32) -> bool {
    return set.a >  cutoff
        && set.b <= cutoff
        && set.c >  cutoff
        && set.d <= cutoff
}

fn check_pattern_4(set: &SquareSet, cutoff: f32) -> bool {
    return set.a >  cutoff
        && set.b >  cutoff
        && set.c <= cutoff
        && set.d >  cutoff
}

fn check_pattern_5(set: &SquareSet, cutoff: f32) -> bool {
    return set.a >= cutoff
        && set.b >= cutoff
        && set.c >= cutoff
        && set.d >= cutoff
}


// Sides:
// a-i>b
// /   |
// j . k
// |   \
// d<l-c

fn pattern_1(set: SquareSet) -> Mesh {
    // 0i-1
    // j /.
    // |/..
    // 2...
    // ....

    let side_i = side_fn(set.a, set.b);
    let side_j = side_fn(set.a, set.d);

    Mesh {
        vertices: vec![
            Vertex { position: [-0.5, -0.5, 0.0], colour: [0.0, 0.0, 0.0] },
            Vertex { position: [-0.5 + side_i, -0.5, 0.0], colour: [0.0, 0.0, 0.0] },
            Vertex { position: [-0.5, -0.5 + side_j, 0.0], colour: [0.0, 0.0, 0.0] }
        ],
        indices: vec![
            0, 1, 2
        ]
    }
}

fn pattern_2(set: SquareSet) -> Mesh {
    // 0---1
    // j / k
    // 2---3
    // .....
    // .....

    let side_j = side_fn(set.a, set.d);
    let side_k = side_fn(set.b, set.c);

    Mesh {
        vertices: vec![
            Vertex { position: [-0.5, -0.5, 0.0], colour: [0.0, 0.0, 0.0] },
            Vertex { position: [0.5, -0.5, 0.0], colour: [0.0, 0.0, 0.0] },
            Vertex { position: [-0.5, -0.5 + side_j, 0.0], colour: [0.0, 0.0, 0.0] },
            Vertex { position: [0.5, -0.5 + side_k, 0.0], colour: [0.0, 0.0, 0.0] }
        ],
        indices: vec![
            0, 1, 2,
            1, 3, 2
        ]
    }
}

fn pattern_3(set: SquareSet) -> Mesh {
    // 0i1..
    // j /\.
    // 2/| 3
    // .\|/k
    // ..4l5

    let side_i = side_fn(set.a, set.b);
    let side_j = side_fn(set.a, set.d);
    let side_k = side_fn(set.c, set.b);
    let side_l = side_fn(set.c, set.d);

    Mesh {
        vertices: vec![
            Vertex { position: [-0.5, -0.5, 0.0], colour: [0.0, 0.0, 0.0] },
            Vertex { position: [-0.5 + side_i, -0.5, 0.0], colour: [0.0, 0.0, 0.0] },
            Vertex { position: [-0.5, -0.5 + side_j, 0.0], colour: [0.0, 0.0, 0.0] },
            Vertex { position: [0.5, 0.5 - side_k, 0.0], colour: [0.0, 0.0, 0.0] },
            Vertex { position: [0.5 - side_l, 0.5, 0.0], colour: [0.0, 0.0, 0.0] },
            Vertex { position: [0.5, 0.5, 0.0], colour: [0.0, 0.0, 0.0] }
        ],
        indices: vec![
            0, 1, 2,
            1, 4, 2,
            1, 3, 4,
            3, 5, 4
        ]
    }
}

fn pattern_4(set: SquareSet) -> Mesh {
    // 0-----1
    // |\  \ k
    // | \   3
    // |  \ /
    // 2-l-4

    let side_k = side_fn(set.b, set.c);
    let side_l = side_fn(set.d, set.c);

    Mesh {
        vertices: vec![
            Vertex { position: [-0.5, -0.5, 0.0], colour: [0.0, 0.0, 0.0] },
            Vertex { position: [0.5, -0.5, 0.0], colour: [0.0, 0.0, 0.0] },
            Vertex { position: [-0.5, 0.5, 0.0], colour: [0.0, 0.0, 0.0] },
            Vertex { position: [0.5, -0.5 + side_k, 0.0], colour: [0.0, 0.0, 0.0] },
            Vertex { position: [-0.5 + side_l, 0.5, 0.0], colour: [0.0, 0.0, 0.0] }
        ],
        indices: vec![
            0, 1, 3,
            0, 3, 4,
            0, 4, 2
        ]
    }
}

fn pattern_5(_set: SquareSet) -> Mesh {
    Mesh {
        vertices: vec![
            Vertex { position: [-0.5, -0.5, 0.0], colour: [0.0, 0.0, 0.0] },
            Vertex { position: [0.5, -0.5, 0.0], colour: [0.0, 0.0, 0.0] },
            Vertex { position: [-0.5, 0.5, 0.0], colour: [0.0, 0.0, 0.0] },
            Vertex { position: [0.5, 0.5, 0.0], colour: [0.0, 0.0, 0.0] }
        ],
        indices: vec![
            0, 1, 2,
            1, 3, 2
        ]
    }
}
