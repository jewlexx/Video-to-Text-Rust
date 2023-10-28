use std::mem::MaybeUninit;

use derive_more::AsRef;

use crate::CHARSET;

#[derive(Debug, Copy, Clone, AsRef)]
pub struct PredictionMatrix([[MaybeUninit<usize>; CHARSET.len()]; CHARSET.len()]);

impl PredictionMatrix {
    pub fn new() -> Self {
        Self([[MaybeUninit::uninit(); CHARSET.len()]; CHARSET.len()])
    }

    pub fn decode_matrix_row(&mut self, row: usize, row_data: usize) {
        // Copy here to be mutable later
        let mut row_data = row_data;
        let mut indexes: [usize; CHARSET.len()] = [0, 1, 2, 3, 4, 5, 6];

        for i in 0..CHARSET.len() {
            let index = row_data % (CHARSET.len() - i);
            let rank = indexes[index];

            row_data /= CHARSET.len() - i;

            // Shift all the indexes over by one
            for j in 0..CHARSET.len() - 1 {
                indexes[j] = indexes[j + 1];
            }

            self.0[row][i] = MaybeUninit::new(rank);
        }
    }
}
