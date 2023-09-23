mod wgpuinit;
use wgpuinit::{ run, Vertex };

fn main() {
    let mut chunk = create_chunk();
    paint_antialiased_filled_circle(&mut chunk, 7.0, 7.0, 5.0);
    paint_antialiased_filled_circle(&mut chunk, 3.0, 3.0, 2.0);
    paint_antialiased_filled_circle(&mut chunk, 10.0, 4.0, 3.0);
    paint_antialiased_filled_circle(&mut chunk, 16.0, 16.0, 12.0);
    paint_antialiased_filled_circle(&mut chunk, 15.0, 25.0, 6.0);
    print_chunk(&chunk);
    chunk_to_image(&chunk);
    
    let tuple = process(&chunk);
    pollster::block_on(run(&tuple.0, &tuple.1, mouse_event));
}

pub fn mouse_event(pos: (f32, f32)) -> () {
    print!("{0}, {1}", pos.0, pos.1);
}

//fn test(chunk: &mut Vec<Vec<f32>>, x: f32, y:f32) -> impl Fn(f32, f32) -> () {
//    return(move |x, y| -> () {
//        paint_antialiased_filled_circle(chunk, x, y, 5.0);
//    })
//}

#[repr(C)]
#[derive(Copy, Clone)]
struct SquareSet {
    a: f32, b: f32,
    c: f32, d: f32,
}

fn process(chunk: &Vec<Vec<f32>>) -> (Vec<Vertex>, Vec<i16>){
    let mut vertex_results: Vec<Vertex> = vec![];
    let mut index_results: Vec<i16> = vec![];

    let input = chunk_to_square_sets(&chunk);
    
    let mut i: i32 = 0;
    let mut totalindex: i16 = 0;

    for item in input {
        let mapped_items = indextransform(set_to_vertices2(item), i, totalindex);

        let size = mapped_items.0.len();
        vertex_results.extend(mapped_items.0);
        index_results.extend(mapped_items.1);

        i += 1;
        totalindex += size as i16;
    }

    (vertex_results, index_results)
}

fn indextransform(tuple: (Vec<Vertex>, Vec<i16>), index: i32, totalIndex: i16) -> (Vec<Vertex>, Vec<i16>) {
    let scale = 31;
    let x = index % scale;
    let y = index / scale;
    
    let mut new_vertices: Vec<Vertex> = vec![];

    for vertex in tuple.0 {
        let new_vertex = Vertex {
            position: [
                (0.5 + &vertex.position[0] + x as f32) / scale as f32 * 2.0 - 1.0,
                (0.5 + &vertex.position[1] + y as f32) / scale as f32 * 2.0 - 1.0,
                0.0
            ],
            colour: vertex.colour
        };
        new_vertices.push(new_vertex);
    }
    
    let mut new_indices: Vec<i16> = vec![];

    for index in tuple.1 {
        let new_index = index + totalIndex;
        new_indices.push(new_index);
    }

    return (new_vertices, new_indices)
}

fn chunk_to_square_sets(chunk: &Vec<Vec<f32>>) -> Vec<SquareSet> {
    let mut square_sets = Vec::new();

    for y in 0..chunk.len() - 1 {
        let row = &chunk[y];

        for x in 0..row.len() - 1 {
            let a = row[x];
            let b = row[x + 1];
            let c = chunk[y + 1][x];
            let d = chunk[y + 1][x + 1];

            let square_set = SquareSet { a, b, c, d };
            square_sets.push(square_set);
        }
    }

    square_sets
}

fn set_to_vertices(set: SquareSet) -> (Vec<Vertex>, Vec<i16>) {
    let colour = (set.a + set.b + set.c + set.d) / 4.0;

    let at = set.a > 0.5;

    if set.a == 1.0 {
        return((vec![
            Vertex { position: [0.0, 0.0, 0.0], colour: [colour, colour, colour]},
            Vertex { position: [1.0, 0.0, 0.0], colour: [colour, colour, colour]},
            Vertex { position: [0.0, 1.0, 0.0], colour: [colour, colour, colour]},
            Vertex { position: [1.0, 1.0, 0.0], colour: [colour, colour, colour]},
        ],
        vec![
            0, 1, 2,
            1, 3, 2
        ]));
    }
    
    if set.a == 1.0 {
        return((vec![
            Vertex { position: [0.0, 0.0, 0.0], colour: [colour, colour, colour]},
            Vertex { position: [1.0, 0.0, 0.0], colour: [colour, colour, colour]},
            Vertex { position: [0.0, 1.0, 0.0], colour: [colour, colour, colour]},
            Vertex { position: [1.0, 1.0, 0.0], colour: [colour, colour, colour]},
        ],
        vec![
            0, 1, 2,
            1, 3, 2
        ]));
    }
    (vec![], vec![])
}

// a b
// c d
//
// Patterns:
// 1)#. 2)## 3) #. 4)## 5)##
//   ..   ..    .#   #.   ##

fn check_pattern_1(set: &SquareSet, cutoff: f32) -> bool {
    return(set.a >  cutoff
        && set.b <= cutoff
        && set.c <= cutoff
        && set.d <= cutoff
    )
}

fn check_pattern_2(set: &SquareSet, cutoff: f32) -> bool {
    return(set.a >  cutoff
        && set.b >  cutoff
        && set.c <= cutoff
        && set.d <= cutoff
    )
}

fn check_pattern_3(set: &SquareSet, cutoff: f32) -> bool {
    return(set.a >  cutoff
        && set.b <= cutoff
        && set.c <= cutoff
        && set.d >  cutoff
    )
}

fn check_pattern_4(set: &SquareSet, cutoff: f32) -> bool {
    return(set.a >  cutoff
        && set.b >  cutoff
        && set.c >  cutoff
        && set.d <= cutoff
    )
}

fn check_pattern_5(set: &SquareSet, cutoff: f32) -> bool {
    return(set.a >= cutoff
        && set.b >= cutoff
        && set.c >= cutoff
        && set.d >= cutoff
    )
}

fn smooth(i: f32) -> f32 {
    (std::f32::consts::E).powf(-1.0/i)
}

fn side_fn(p1: f32, p2: f32) -> f32 {
    let i = (p1 + p2) / 2.0;
    let smooth_i = smooth(i);
    smooth_i / (smooth_i + smooth(1.0 - i))
    //0.5
}

// Sides:
// a-i>b
// /   |
// j . k
// |   \
// c<l-d

fn pattern_1(set: SquareSet) -> (Vec<Vertex>, Vec<i16>) {
    // 0i-1
    // j /.
    // |/..
    // 2...
    // ....

    let side_i = side_fn(set.a, set.b);
    let side_j = side_fn(set.a, set.c);

    return (vec![
        Vertex { position: [-0.5, -0.5, 0.0], colour: [0.0, 0.0, 0.0] },
        Vertex { position: [-0.5 + side_i, -0.5, 0.0], colour: [0.0, 0.0, 0.0] },
        Vertex { position: [-0.5, -0.5 + side_j, 0.0], colour: [0.0, 0.0, 0.0] }
    ], vec![
        0, 1, 2
    ])
}

fn pattern_2(set: SquareSet) -> (Vec<Vertex>, Vec<i16>) {
    // 0---1
    // j / k
    // 2---3
    // .....
    // .....

    let side_j = side_fn(set.a, set.c);
    let side_k = side_fn(set.b, set.d);

    return (vec![
        Vertex { position: [-0.5, -0.5, 0.0], colour: [0.0, 0.0, 0.0] },
        Vertex { position: [0.5, -0.5, 0.0], colour: [0.0, 0.0, 0.0] },
        Vertex { position: [-0.5, -0.5 + side_j, 0.0], colour: [0.0, 0.0, 0.0] },
        Vertex { position: [0.5, -0.5 + side_k, 0.0], colour: [0.0, 0.0, 0.0] }
    ], vec![
        0, 1, 2,
        1, 3, 2
    ])
}

fn pattern_3(set: SquareSet) -> (Vec<Vertex>, Vec<i16>) {
    // 0i1..
    // j /\.
    // 2/| 3
    // .\|/k
    // ..4l5

    let side_i = side_fn(set.a, set.b);
    let side_j = side_fn(set.c, set.a);
    let side_k = side_fn(set.d, set.b);
    let side_l = side_fn(set.d, set.c);

    return (vec![
        Vertex { position: [-0.5, -0.5, 0.0], colour: [0.0, 0.0, 0.0] },
        Vertex { position: [-0.5 + side_i, -0.5, 0.0], colour: [0.0, 0.0, 0.0] },
        Vertex { position: [-0.5, -0.5 + side_j, 0.0], colour: [0.0, 0.0, 0.0] },
        Vertex { position: [0.5, 0.5 - side_k, 0.0], colour: [0.0, 0.0, 0.0] },
        Vertex { position: [0.5 - side_l, 0.5, 0.0], colour: [0.0, 0.0, 0.0] },
        Vertex { position: [0.5, 0.5, 0.0], colour: [0.0, 0.0, 0.0] }
    ], vec![
        0, 1, 2,
        1, 4, 2,
        1, 3, 4,
        3, 5, 4
    ])
}

fn pattern_4(set: SquareSet) -> (Vec<Vertex>, Vec<i16>) {
    // 0-----1
    // |\  \ k
    // | \   3
    // |  \ /
    // 2-l-4

    let side_k = side_fn(set.b, set.d);
    let side_l = side_fn(set.c, set.d);

    return (vec![
        Vertex { position: [-0.5, -0.5, 0.0], colour: [0.0, 0.0, 0.0] },
        Vertex { position: [0.5, -0.5, 0.0], colour: [0.0, 0.0, 0.0] },
        Vertex { position: [-0.5, 0.5, 0.0], colour: [0.0, 0.0, 0.0] },
        Vertex { position: [0.5, -0.5 + side_k, 0.0], colour: [0.0, 0.0, 0.0] },
        Vertex { position: [-0.5 + side_l, 0.5, 0.0], colour: [0.0, 0.0, 0.0] }
    ], vec![
        0, 1, 3,
        0, 3, 4,
        0, 4, 2
    ])
}

fn pattern_5(set: SquareSet) -> (Vec<Vertex>, Vec<i16>) {
    return (vec![
        Vertex { position: [-0.5, -0.5, 0.0], colour: [0.0, 0.0, 0.0] },
        Vertex { position: [0.5, -0.5, 0.0], colour: [0.0, 0.0, 0.0] },
        Vertex { position: [-0.5, 0.5, 0.0], colour: [0.0, 0.0, 0.0] },
        Vertex { position: [0.5, 0.5, 0.0], colour: [0.0, 0.0, 0.0] }
    ], vec![
        0, 1, 2,
        1, 3, 2
    ])
}

fn rotate_pattern(pattern: (Vec<Vertex>, Vec<i16>), index: i32) -> (Vec<Vertex>, Vec<i16>) {
    let mut rotation = pattern;
    for i in 0..index {
        let mut new_vertices: Vec<Vertex> = vec![];
    
        for vertex in rotation.0 {
            new_vertices.push(Vertex {
                position: [vertex.position[1], -vertex.position[0], vertex.position[2]],
                colour: vertex.colour//[index as f32/3.0, index as f32/3.0, index as f32/3.0]
            });
        }

        rotation = (new_vertices, rotation.1);
    }
    
    rotation
}

fn set_to_vertices2(set: SquareSet) -> (Vec<Vertex>, Vec<i16>) {
    // Rotations:
    // 1)ab 2)ca 3)dc 4)bd
    //   cd   db   ba   ac

    let cutoff = 0.2;
    
    let mut rotation = set.clone();
    for i in 0..=3 {
        let output: Option<(Vec<Vertex>, Vec<i16>)> = match rotation {
            _ if check_pattern_1(&rotation, cutoff) => Some(pattern_1(rotation)),
            _ if check_pattern_2(&rotation, cutoff) => Some(pattern_2(rotation)),
            _ if check_pattern_3(&rotation, cutoff) => Some(pattern_3(rotation)),
            _ if check_pattern_4(&rotation, cutoff) => Some(pattern_4(rotation)),
            _ if check_pattern_5(&rotation, cutoff) => Some(pattern_5(rotation)),
            _ => None
        };

        match output {
            Some(x) => {
                let rotate = rotate_pattern(x.clone(), i);
                println!("init: {0}, \t{1}\n      {2}, \t{3}", set.a, set.b, set.c, set.d);
                println!("rota: {0}, \t{1}\n      {2}, \t{3}", rotation.a, rotation.b, rotation.c, rotation.d);

                for vertex in &x.0 {
                    println!("normal: [{0}, {1}, {2}]", vertex.position[0], vertex.position[1], vertex.position[2]);
                }
                
                for vertex in &rotate.0 {
                    println!("rotate: [{0}, {1}, {2}]", vertex.position[0], vertex.position[1], vertex.position[2]);
                }

                return rotate
            },
            None => ()
        }
        
        rotation = SquareSet { a: rotation.c, b: rotation.a, c: rotation.d, d: rotation.b};
        //rotation = SquareSet { a: rotation.b, b: rotation.d, c: rotation.a, d: rotation.c};

    }

    return(vec![], vec![])
}

//fn test_rotations(square_set: &SquareSet) -> Option<(Vec<Vertex>, Vec<i16>)> {
//    let rotations = vec![
//        (square_set.a, square_set.b, square_set.c, square_set.d),
//        (square_set.c, square_set.a, square_set.d, square_set.b),
//        (square_set.d, square_set.c, square_set.b, square_set.a),
//        (square_set.b, square_set.d, square_set.a, square_set.c),
//    ];
//
//    for (a, b, c, d) in rotations {
//        let result = pattern_match(a, b, c, d);
//        if !result.is_empty() {
//      .0      match result.0.len() {
//                0 => {}
//                _ => return(rotate_set(result))
//            };
//        }
//    }
//
//    None
//}
//
//fn pattern_match(a: bool, b: bool, c: bool, d: bool) -> (Vec<Vertex>, Vec<i16>) {
//    if a && b && c && d {
//        return (
//            vec![
//                Vertex {position: [-0.5, -0.5, 0.0], colour: [1.0, 1.0, 1.0]},
//                Vertex {position: [0.5, -0.5, 0.0], colour: [1.0, 1.0, 1.0]},
//                Vertex {position: [-0.5, 0.5, 0.0], colour: [1.0, 1.0, 1.0]},
//                Vertex {position: [0.5, 0.5, 0.0], colour: [1.0, 1.0, 1.0]},
//            ],
//            vec![
//                0, 1, 3,
//                0, 2, 3
//            ]
//        );
//    } else if (a && b && c && !d) {
//        return (
//            vec![
//                Vertex {position: [-0.5, -0.5, 0.0], colour: [1.0, 1.0, 1.0]},
//                Vertex {position: [0.5, -0.5, 0.0], colour: [1.0, 1.0, 1.0]},
//                Vertex {position: [-0.5, 0.5, 0.0], colour: [1.0, 1.0, 1.0]},
//                Vertex {position: [0.5, 0.0, 0.0], colour: [1.0, 1.0, 1.0]},
//                Vertex {position: [0.0, 0.5, 0.0], colour: [1.0, 1.0, 1.0]},
//            ],
//            vec![

//                0, 1, 3,
//                0, 3, 4,
//                0, 2, 4
//            ]
//        );
//    } else if a && !b && c && !d {
//        return (
//            vec![
//                Vertex {position: [-0.5, -0.5, 0.0], colour: [1.0, 1.0, 1.0]},
//                Vertex {position: [0.0, -0.5, 0.0], colour: [1.0, 1.0, 1.0]},
//                Vertex {position: [-0.5, 0.5, 0.0], colour: [1.0, 1.0, 1.0]},
//                Vertex {position: [0.0, 0.5, 0.0], colour: [1.0, 1.0, 1.0]},
//            ],
//            vec![
//                0, 2, 3,
//                0, 1, 3,
//            ]
//        );
//    } else if a && !b && !c && !d {
//        return (
//            vec![
//                Vertex {position: [-0.5, -0.5, 0.0], colour: [1.0, 1.0, 1.0]},
//                Vertex {position: [0.0, -0.5, 0.0], colour: [1.0, 1.0, 1.0]},
//                Vertex {position: [-0.5, 0.0, 0.0], colour: [1.0, 1.0, 1.0]},
//            ],
//            vec![
//                0, 1, 2,
//            ]
//        );
//    }
//    return (
//        vec![],
//        vec![]
//    );
//}
//fn flat_map<t, u, f>(input: Vec<t>, f: f) -> Vec<u>
//where
//    f: fn(t) -> Vec<u>,
//{
//    let mut result = vec::new();
//    for item in input {
//        let mapped_items = f(item);
//        result.extend(mapped_items);
//    }
//    result
//}
//
//fn vec_to_arr<T>(vec: Vec<T>) -> &[T] {
//    let slice: &[T] = &vec;
//    slice
//}

fn create_chunk() -> Vec<Vec<f32>> {
    vec![vec![1.0; 32]; 32]
}

fn print_chunk(chunk: &Vec<Vec<f32>>) {
    for row in chunk.iter() {
        for value in row {
            print!("{} ", value);
        }
        println!();
    }
}

fn paint_antialiased_filled_circle(chunk: &mut Vec<Vec<f32>>, x: f32, y: f32, radius: f32) {
    let radius_squared = radius * radius;
    let len1 = chunk.len() as i32;

    for i in 0..len1 {
        for j in 0..len1 {
            let distance_squared = ((i as f32 - x).powf(2.0) + (j as f32 - y).powf(2.0));

            if distance_squared <= radius_squared {
                let distance = distance_squared.sqrt();
                let alpha = 1.0 - (distance - radius).abs() / 1.0;
                chunk[i as usize][j as usize] = alpha.max(0.0).min(chunk[i as usize][j as usize]);
            }
        }
    }
}

fn chunk_to_image(chunk: &Vec<Vec<f32>>) {
    // Create an empty image with the same dimensions as the chunk
    let width = chunk[0].len();
    let height = chunk.len();
    let mut image = image::ImageBuffer::new(width as u32, height as u32);

    // Iterate over each pixel in the chunk
    for (y, row) in chunk.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            // Convert the value to a grayscale color
            let color = (value * 255.0) as u8;
            let pixel = image::Rgb([color, color, color]);

            // Set the pixel in the image
            image.put_pixel(x as u32, y as u32, pixel);
        }
    }

    // Save the image to a file
    image.save("chunk.png").unwrap();
}
