mod bmp;
mod dither;

use bmp::{Bmp, Pixel};
use dither::*;
use std::env::args;

fn main() {
    let filename = args().nth(1).unwrap();
    let output_file = args().nth(2).unwrap();
    let colors: Vec<Pixel> =
        match args().nth(3) {
            Some(value) => {
                match &*value {
                    "bw" => vec![
                        Pixel::black(),
                        Pixel::white(),
                    ],
                    "rgb" => vec![
                        Pixel::red(),
                        Pixel::green(),
                        Pixel::blue(),
                    ],
                    "basic" => vec![
                        Pixel::red(),
                        Pixel::green(),
                        Pixel::blue(),
                        Pixel::cyan(),
                        Pixel::magenta(),
                        Pixel::yellow(),
                        Pixel::white(),
                        Pixel::black(),
                    ],
                    _ => value.split('/').map(Pixel::parse).collect(),
                }
            }
            _ => panic!("specify colors"),
        };
    match args().nth(4) {
        Some(action) => {
            let mut bmp = Bmp::load(&filename).unwrap();
            println!("Loaded bitmap: {:?}", bmp);
            let delegate = match action.as_str() {
                "closest" => closest_matrix_dither,
                "diffuse" => diffuse_matrix_dither,
                "floyd" => floyd_matrix_dither,
                "ffloyd" => false_floyd_matrix_dither,
                "jarvis" => jjn_matrix_dither,
                "stucki" => stucki_matrix_dither,
                "atkinson" => atkinson_matrix_dither,
                "burkes" => burkes_matrix_dither,
                "sierra" => sierra_matrix_dither,
                "sierra2" => sierra2_matrix_dither,
                "sierra_lite" => sierra_lite_matrix_dither,
                "bayer4" => bayer_4x4,
                "bayer8" => bayer_8x8,
                a => panic!("unrecognized action '{}'", a),
            };
            delegate(&mut bmp, &colors);
            bmp.save(&output_file).unwrap();
        },
        _ => panic!("specify action"),
    }
}
