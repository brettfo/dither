extern crate k_means;

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
                    "auto" => {
                        let mut values = vec![];
                        let bmp = Bmp::load(&filename).unwrap();
                        for y in 0..bmp.height() as usize {
                            for x in 0..bmp.width() as usize {
                                let p = bmp.pixels[x][y];
                                let p = vec![
                                    p.r as f32,
                                    p.g as f32,
                                    p.b as f32,
                                ];
                                values.push(p);
                            }
                        }
                        let groups = 16;
                        let iterations = 10;
                        let auto: Vec<Pixel> = k_means::k_means(&values, groups, 3, iterations, 0.0, 255.0).iter()
                            .map(|v| Pixel {r: v[0] as u8, g: v[1] as u8, b: v[2] as u8 })
                            .collect();
                        println!("Auto colors:");
                        for p in &auto {
                            println!("  {:?}", p);
                        }
                        auto
                    },
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
