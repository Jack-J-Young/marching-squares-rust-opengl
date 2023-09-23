use std::iter;

pub struct Chunk {
    pub data: Vec<Vec<f32>>,
    size: usize
}

impl Chunk {
    fn new(size: usize) -> Self {
        let data = iter::repeat(iter::repeat(1.0).take(size).collect()).take(size).collect();
        //let data: Vec<Vec<f32>> = vec![vec![1.0; size as usize]; size as usize];

        Self {
            data,
            size
        }
    }
}
