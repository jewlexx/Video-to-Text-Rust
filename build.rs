// TODO: Replace the makefile and convert.py with this build script

use std::fs::File;

fn main() {
    println!("cargo:rerun-if-changed=vid.mp4");

    // let video = File::open("vid.mp4").expect("video file at vid.mp4");
}
