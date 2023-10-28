use std::{fmt::Display, thread, time::Duration};

use derive_more::{AsMut, AsRef};

use crate::{huff::HuffmanTree, prediction::PredictionMatrix};

const DATA: &[u8] = include_bytes!("../data");
const CHARSET: [char; 7] = [' ', ',', '(', 'S', '#', 'g', '@'];

const NODE_COUNT: usize = CHARSET.len() * 2 - 1;

const WIDTH: usize = 80;
const HEIGHT: usize = 22;
const FPS: usize = 15;

const FRAME_SIZE: usize = (WIDTH + 1) * HEIGHT + 1;

mod huff;
mod prediction;

#[derive(Debug, Copy, Clone, AsRef, AsMut)]
#[must_use]
pub struct Frame([char; FRAME_SIZE]);

impl Default for Frame {
    fn default() -> Self {
        Self([0 as char; FRAME_SIZE])
    }
}

impl Frame {
    fn coerced(&self) -> [u8; FRAME_SIZE] {
        self.0.map(|c| c as u8)
    }
}

impl Display for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // This is relatively safe, as we know all the data that will be in the slice
        let byte_array = self.coerced();
        let frame_string = unsafe { std::str::from_utf8_unchecked(&byte_array) };
        f.pad(frame_string)
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct VideoDataIterator {
    data_pos: usize,
}

impl VideoDataIterator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn peek_char(&self) -> Option<u8> {
        // TODO: having <= maybe breaks stuff, validate this
        if self.data_pos <= DATA.len() {
            Some(DATA[self.data_pos])
        } else {
            None
        }
    }
}

impl Iterator for VideoDataIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.data_pos += 1;
        self.peek_char()
    }
}

fn main() {
    assert_eq!(CHARSET.len(), 7);

    let mut data = VideoDataIterator::new();
    let mut prediction_matrix = PredictionMatrix::new();
    let mut tree = HuffmanTree::new();

    while data.peek_char().is_some() {
        for row in 0..CHARSET.len() {
            let row_data: usize =
                ((data.next().unwrap() as usize) * 256) + (data.next().unwrap() as usize);

            prediction_matrix.decode_matrix_row(row, row_data);
        }

        tree.decode_tree(&mut data);
        let frame = tree.decode_frame(&mut data, &prediction_matrix);

        println!("{frame}");

        thread::sleep(Duration::from_millis(1000 / 60));
    }
}
