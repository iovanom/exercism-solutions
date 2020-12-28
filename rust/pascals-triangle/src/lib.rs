#[derive(Default)]
pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self { row_count }
    }

    fn calculate_cell_value(prev_value: u32, row: u32, col: u32) -> u32 {
        ((prev_value as f64) * ((row + 1 - col) as f64 / col as f64)).floor() as u32
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut vec = Vec::with_capacity(self.row_count as usize);
        for i in 0..self.row_count {
            let mut inner_vec = Vec::with_capacity((i + 1) as usize);
            for j in 0..(i + 1) {
                match j {
                    0 => inner_vec.push(1),
                    _ => inner_vec.push(Self::calculate_cell_value(
                        inner_vec[(j - 1) as usize],
                        i,
                        j,
                    )),
                }
            }
            vec.push(inner_vec);
        }
        vec
    }
}
