mod bmp;
mod dither;

use bmp::Bmp;
use dither::*;
use std::env::args;

fn main() {
    let filename = args().nth(1).unwrap();
    let output_file = args().nth(2).unwrap();
    match (args().nth(3), args().nth(4)) {
        (Some(action), Some(grayscale)) => {
            let mut bmp = Bmp::load(&filename).unwrap();
            println!("Loaded bitmap: {:?}", bmp);
            let delegate = match action.as_str() {
                "black" => black_matrix_dither,
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
            let grayscale = match grayscale.as_str() {
                "average" => average,
                "luminance" => luminance,
                "luma" => luma,
                "bt601" => bt601,
                "desaturate" => desaturate,
                g => panic!("unrecognized grayscale function '{}'", g),
            };
            delegate(&mut bmp, &grayscale);
            bmp.save(&output_file).unwrap();
        },
        _ => panic!("specify action and grayscale conversion"),
    }
}
