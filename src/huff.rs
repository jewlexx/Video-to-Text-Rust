//  a,  8a
//  `8, `8)                            ,adPPRg,
//   8)  ]8                        ,ad888888888b
//  ,8' ,8'                    ,gPPR888888888888
// ,8' ,8'                 ,ad8""   `Y888888888P
// 8)  8)              ,ad8""        (8888888""
// 8,  8,          ,ad8""            d888""
// `8, `8,     ,ad8""            ,ad8""
//  `8, `" ,ad8""            ,ad8""
//     ,gPPR8b           ,ad8""
//    dP:::::Yb      ,ad8""
//    8):::::(8  ,ad8""
//    Yb:;;;:d888""  Normand
//     "8ggg8P"      Veilleux

use crate::{
    prediction::PredictionMatrix, Frame, VideoDataIterator, CHARSET, DATA, FRAME_SIZE, HEIGHT,
    NODE_COUNT, WIDTH,
};

pub struct HuffmanTree {
    tree_left: [u8; NODE_COUNT],
    tree_right: [u8; NODE_COUNT],
}

impl HuffmanTree {
    pub fn new() -> Self {
        Self {
            tree_left: [0; NODE_COUNT],
            tree_right: [0; NODE_COUNT],
        }
    }

    pub fn decode_tree(&mut self, data_iter: &mut VideoDataIterator) {
        for i in 0..CHARSET.len() - 1 {
            let c = data_iter.next().expect("non-exhausted iterator");
            let left = c / 16;
            let right = c % 16;

            self.tree_left[i + CHARSET.len()] = left;
            self.tree_right[i + CHARSET.len()] = right;
        }
    }

    pub fn decode_frame(
        &mut self,
        data_iter: &mut VideoDataIterator,
        prediction_matrix: &PredictionMatrix,
    ) -> Frame {
        let mut frame = Frame::default();

        let mut bit_count: u8 = 0;
        let mut c: Option<u8> = None;

        // Start our node at the root of the tree
        let mut node: usize = NODE_COUNT - 1;
        let mut last_char: usize = 0;

        let mut out_pos: usize = 0;

        let mut out_count: usize = 0;
        let mut line_count: usize = 0;

        loop {
            if bit_count == 0 {
                c = data_iter.next();
                bit_count = 8;
            }

            let bit = if let Some(c) = c.as_mut() {
                let bit = *c & 0b10000000;
                *c <<= 1;

                bit
            } else {
                break;
            };

            bit_count -= 1;

            node = if bit > 0 {
                self.tree_right[node] as usize
            } else {
                self.tree_left[node] as usize
            };

            if node < CHARSET.len() {
                // TODO: Verify the safety of this
                last_char = unsafe { prediction_matrix.as_ref()[last_char][node].assume_init() };

                out_count += 1;
                out_pos += 1;
                frame.as_mut()[out_pos] = CHARSET[last_char];

                if out_count >= WIDTH {
                    out_count = 0;
                    line_count += 1;

                    if line_count >= HEIGHT {
                        out_pos += 1;
                        frame.as_mut()[out_pos] = '\0';
                        break;
                    }
                }

                node = NODE_COUNT - 1;
            }
        }

        frame
    }
}
