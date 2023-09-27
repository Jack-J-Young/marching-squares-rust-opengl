use std::iter;
 
#[repr(C)]
#[derive(Clone)]
pub struct Chunk {
    pub data: Vec<Vec<f32>>,
    pub size: usize
}

impl Chunk {
    pub fn new(size: usize) -> Self {
        let data: Vec<Vec<f32>> = iter::repeat(iter::repeat(1.0).take(size).collect()).take(size).collect();
        Self {
            data,
            size
        }
    }

    pub fn get_edge(&self, dir: i32) -> Vec<Vec<f32>> {
        match dir {
            0 => vec![self.data[0].clone()],
            1 => self.data.iter().map(|x| vec![x[0]]).collect(),
            2 => vec![vec![self.data[0][0]]],
            _ => vec![],
        }
    }

    pub fn add_data(&mut self, data: &Vec<Vec<f32>>, x_offset: usize, y_offset: usize) -> &mut Self {
        for x in 0..data.len() {
            for y in 0..data[x].len() {
                self.data[x + x_offset][y + y_offset] = data[x][y];
            }
        }

        self
    }

    pub fn paint_antialiased_filled_circle(&mut self, x: f32, y: f32, radius: f32) {
        let radius_squared = radius * radius;
    
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                let distance_squared = (i as f32 - x).powf(2.0) + (j as f32 - y).powf(2.0);
    
                if distance_squared <= radius_squared {
                    let distance = distance_squared.sqrt();
                    let alpha = 1.0 - (distance - radius).abs() / 1.0;
                    self.data[i as usize][j as usize] = alpha.max(0.0).min(self.data[i as usize][j as usize]);
                }
            }
        }
    }

    pub fn print(&self) {
        for row in self.data.iter() {
            for value in row {
                print!("{} ", value);
            }
            println!();
        }
    }
}
