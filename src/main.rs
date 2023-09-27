mod wgpuinit;
use wgpuinit::run;
mod chunk;
use chunk::Chunk;
mod square_march;
use square_march::SquareSet;
mod mesh;
use mesh::{Mesh, Meshable, Vertex};
mod plane;
use plane::{Plane, ReferencePoint};
use std::collections::HashMap;
use winit::{
    event::*
};

fn pattern_width(x: i32) -> i32 {
    if x == 0 {
        return 0;
    }
    2_i32.pow(x as u32) + pattern_width(x - 1)
}

pub fn gen_controls() -> (fn(&WindowEvent) -> bool) {
    return |event: &WindowEvent| {
        match event {
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(virtual_keycode),..
                },..
            } => match virtual_keycode {
                VirtualKeyCode::Up => {
                    
                    true
                },
                _ => false
            },
            _ => false 
        }
    }
}
fn input(event: &WindowEvent) -> bool {
    let test = event;
        match test {
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state,
                    virtual_keycode: Some(virtual_keycode),..
                },..
            } => {
                println!("state: {0}, key: {1}", *state == ElementState::Pressed, match virtual_keycode {
                    VirtualKeyCode::Up => "up_key",
                    _ => "unkn"
                });
                return true
            },
            _ => return false
        }

}

fn main() {
    let mut plane = Plane::new();

    let mut chunk = Chunk::new(32);

    let mut reference = ReferencePoint {
        position: (0.0, 0.0),
        render_dist: 32.0
    };

    //chunk.paint_antialiased_filled_circle(10.0, 10.0, 6.0);

    //chunk.print();

    //plane.set_chunk((0, 0), chunk.clone());
    let size = 6;
    //plane.clone_area(&chunk, 0, 0, pattern_width(size + 1), pattern_width(size + 1));

    for i in 1..=size {
        plane.paint_antialiased_filled_circle(
            pattern_width(i + 1) as f32,
            pattern_width(i + 1) as f32,
            2.0_f32.powf(i as f32),
        );
    }

    let zoom = 1.0;
    //plane.paint_antialiased_filled_circle(521.0, 512.0, 256.0);

    //let sets = SquareSet::chunk_to_sets(&chunk);

    //let mut mesh: Mesh = plane.clone().mesh_from_ref(reference);/*ReferencePoint {
    //    position: (52.0, 52.0),
     //    render_dist: 32.0 * zoom,
    //});*/

    //for vertex in &mesh.vertices {
    //    println!(
    //        "v{0}, {1}, {2}",
    //        vertex.position[0], vertex.position[1], vertex.position[2]
    //    );
    //}
    //for a in mesh.indices.windows(3) {
    //    println!("i{0}, {1}, {2}", a[0], a[1], a[2]);
    //}

    //println!("{}", mesh.vertices.len());

    //mesh.scale(1.0 / zoom);

    pollster::block_on(run(plane));
}
