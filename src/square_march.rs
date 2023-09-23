use crate::Chunk;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SquareSet {
    pub a: f32, pub b: f32,
    pub d: f32, pub c: f32,
}

impl SquareSet {
    fn chunk_to_sets(chunk: Chunk) -> Vec<Self> {
        let unprocessed_sets: Vec<Vec<Vec<f32>>> = 
        chunk.data.windows(2)
            .map(|square_set|
                square_set.iter()
                    .map(|row| 
                        row.iter()
                            .map(|value| *value)
                            .collect())
                    .collect())
            .collect();

        unprocessed_sets.iter().map(|x| Self {
            a: x[0][0], b: x[1][0],
            d: x[0][1], c: x[1][1],
        }).collect()
    }

    fn to_vertices(&self) {
        
    }

    fn rotate(&mut self, counter_clock: bool) {
        let temp = self.a;
        self.a = self.d;
        self.d = self.c;
        self.c = self.b;
        self.b = temp;
    }
}
