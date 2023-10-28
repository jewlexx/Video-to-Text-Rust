const DATA: &[u8] = include_bytes!("../data");
const CHARSET: [char; 7] = [' ', ',', '(', 'S', '#', 'g', '@'];

const NODE_COUNT: usize = CHARSET.len() * 2 - 1;

const WIDTH: u16 = 80;
const HEIGHT: u16 = 22;
const FPS: u8 = 15;

const FRAME_SIZE: u16 = (WIDTH + 1) * HEIGHT + 1;

mod huff;
mod prediction;

fn main() {
    println!("Hello, world!");
}
