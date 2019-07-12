use bmp::{Bmp, Pixel};

pub fn average(p: &(i32, i32, i32)) -> i32 {
    (p.0 + p.1 + p.2) / 3
}

pub fn luminance(p: &(i32, i32, i32)) -> i32 {
    (p.0 as f32 * 0.3 + p.1 as f32 * 0.59 + p.2 as f32 * 0.11) as i32
}

pub fn luma(p: &(i32, i32, i32)) -> i32 {
    (p.0 as f32 * 0.2126 + p.1 as f32 * 0.7152 + p.2 as f32 * 0.0722) as i32
}

pub fn bt601(p: &(i32, i32, i32)) -> i32 {
    (p.0 as f32 * 0.299 + p.1 as f32 * 0.587 + p.2 as f32 * 0.114) as i32
}

pub fn desaturate(p: &(i32, i32, i32)) -> i32 {
    use std::cmp::{max, min};
    let &(r, g, b) = p;
    let mx = max(max(r, g), b);
    let mn = min(min(r, g), b);
    (mx + mn) / 2
}

struct Matrix {
    values: (((), (), (), i32, i32),
             (i32, i32, i32, i32, i32),
             (i32, i32, i32, i32, i32)),
    divisor: i32,
}

pub fn black_matrix_dither<F>(bmp: &mut Bmp, grayscale: &F) where F: Fn(&(i32, i32, i32)) -> i32 {
    matrix_dither(
        bmp,
        &Matrix { values: (((), (), (), 0, 0),
                           ( 0,  0,  0, 0, 0),
                           ( 0,  0,  0, 0, 0)),
                  divisor: 1 },
        grayscale);
}

pub fn diffuse_matrix_dither<F>(bmp: &mut Bmp, grayscale: &F) where F: Fn(&(i32, i32, i32)) -> i32 {
    matrix_dither(
        bmp,
        &Matrix { values: (((), (), (), 1, 0),
                           ( 0,  0,  0, 0, 0),
                           ( 0,  0,  0, 0, 0)),
                  divisor: 1 },
        grayscale);
}

pub fn floyd_matrix_dither<F>(bmp: &mut Bmp, grayscale: &F) where F: Fn(&(i32, i32, i32)) -> i32 {
    matrix_dither(
        bmp,
        &Matrix { values: (((), (), (), 7, 0),
                           ( 0,  3,  5, 1, 0),
                           ( 0,  0,  0, 0, 0)),
                  divisor: 16 },
        grayscale);
}

pub fn false_floyd_matrix_dither<F>(bmp: &mut Bmp, grayscale: &F) where F: Fn(&(i32, i32, i32)) -> i32 {
    matrix_dither(
        bmp,
        &Matrix { values: (((), (), (), 3, 0),
                           ( 0,  0,  3, 2, 0),
                           ( 0,  0,  0, 0, 0)),
                  divisor: 8 },
        grayscale);
}

pub fn jjn_matrix_dither<F>(bmp: &mut Bmp, grayscale: &F) where F: Fn(&(i32, i32, i32)) -> i32 {
    matrix_dither(
        bmp,
        &Matrix { values: (((), (), (), 7, 5),
                           ( 3,  5,  7, 5, 3),
                           ( 1,  3,  5, 3, 1)),
                  divisor: 48 },
        grayscale);
}

pub fn stucki_matrix_dither<F>(bmp: &mut Bmp, grayscale: &F) where F: Fn(&(i32, i32, i32)) -> i32 {
    matrix_dither(
        bmp,
        &Matrix { values: (((), (), (), 8, 4),
                           ( 2,  4,  8, 4, 2),
                           ( 1,  2,  4, 2, 1)),
                  divisor: 42 },
        grayscale);
}

pub fn atkinson_matrix_dither<F>(bmp: &mut Bmp, grayscale: &F) where F: Fn(&(i32, i32, i32)) -> i32 {
    matrix_dither(
        bmp,
        &Matrix { values: (((), (), (), 1, 1),
                           ( 0,  1,  1, 1, 0),
                           ( 0,  0,  1, 0, 0)),
                  divisor: 8 },
        grayscale);
}

pub fn burkes_matrix_dither<F>(bmp: &mut Bmp, grayscale: &F) where F: Fn(&(i32, i32, i32)) -> i32 {
    matrix_dither(
        bmp,
        &Matrix { values: (((), (), (), 8, 4),
                           ( 2,  4,  8, 4, 2),
                           ( 0,  0,  0, 0, 0)),
                  divisor: 32 },
        grayscale);
}

pub fn sierra_matrix_dither<F>(bmp: &mut Bmp, grayscale: &F) where F: Fn(&(i32, i32, i32)) -> i32 {
    matrix_dither(
        bmp,
        &Matrix { values: (((), (), (), 5, 3),
                           ( 2,  4,  5, 4, 2),
                           ( 0,  2,  3, 2, 0)),
                  divisor: 32 },
        grayscale);
}

pub fn sierra2_matrix_dither<F>(bmp: &mut Bmp, grayscale: &F) where F: Fn(&(i32, i32, i32)) -> i32 {
    matrix_dither(
        bmp,
        &Matrix { values: (((), (), (), 4, 3),
                           ( 1,  2,  3, 2, 1),
                           ( 0,  0,  0, 0, 0)),
                  divisor: 16 },
        grayscale);
}

pub fn sierra_lite_matrix_dither<F>(bmp: &mut Bmp, grayscale: &F) where F: Fn(&(i32, i32, i32)) -> i32 {
    matrix_dither(
        bmp,
        &Matrix { values: (((), (), (), 2, 0),
                           ( 0,  1,  1, 0, 0),
                           ( 0,  0,  0, 0, 0)),
                  divisor: 4 },
        grayscale);
}

pub fn bayer_4x4<F>(bmp: &mut Bmp, grayscale: &F) where F: Fn(&(i32, i32, i32)) -> i32 {
    let mut matrix = Vec::with_capacity(4);
    matrix.push(vec![1, 9, 3, 11]);
    matrix.push(vec![13, 5, 15, 7]);
    matrix.push(vec![4, 12, 2, 10]);
    matrix.push(vec![16, 8, 14, 6]);
    ordered_dither(bmp, grayscale, &matrix);
}

pub fn bayer_8x8<F>(bmp: &mut Bmp, grayscale: &F) where F: Fn(&(i32, i32, i32)) -> i32 {
    let mut matrix = Vec::with_capacity(8);
    matrix.push(vec![1, 49, 13, 61, 4, 52, 16, 64]);
    matrix.push(vec![33, 17, 45, 29, 36, 20, 48, 32]);
    matrix.push(vec![9, 57, 5, 53, 12, 60, 8, 56]);
    matrix.push(vec![41, 25, 37, 21, 44, 28, 40, 24]);
    matrix.push(vec![3, 51, 15, 63, 2, 50, 14, 62]);
    matrix.push(vec![35, 19, 47, 31, 34, 18, 46, 30]);
    matrix.push(vec![11, 59, 7, 55, 10, 58, 6, 54]);
    matrix.push(vec![43, 27, 39, 23, 42, 26, 38, 22]);
    ordered_dither(bmp, grayscale, &matrix);
}

fn ordered_dither<F>(bmp: &mut Bmp, grayscale: &F, matrix: &Vec<Vec<i32>>) where F: Fn(&(i32, i32, i32)) -> i32 {
    let threshold = 127;
    let size = matrix.len();
    let factor = size as i32 * size as i32 + 1;
    for y in 0..bmp.height() as usize {
        for x in 0..bmp.width() as usize {
            let p = bmp.pixels[x][y];
            let pt = p.as_tuple();
            let v = matrix[x % size][y % size];
            let pv = mul(&div(&mul(&pt, factor), 255), v);
            let g = grayscale(&pv);
            bmp.pixels[x][y] = if g < threshold { Pixel::black() } else { Pixel::white() };
        }
    }
}

fn matrix_dither<F>(bmp: &mut Bmp, matrix: &Matrix, grayscale: &F) where F: Fn(&(i32, i32, i32)) -> i32 {
    let threshold = 128;
    let mut err_next_1;
    let mut err_next_2;
    let mut err_cur_row = vec![(0, 0, 0); bmp.width() as usize];
    let mut err_next_row_1 = vec![(0, 0, 0); bmp.width() as usize];
    let mut err_next_row_2 = vec![(0, 0, 0); bmp.width() as usize];

    let (((), (), (), d, e),
         (f, g, h, i, j),
         (k, l, m, n, o)) = matrix.values;

    for y in 0..bmp.height() as usize {
        // reset for each line
        err_next_1 = (0, 0, 0);
        err_next_2 = (0, 0, 0);
        // 3-way swap/rotation
        ::std::mem::swap(&mut err_cur_row, &mut err_next_row_1); // now err_cur_row = err_next_row_1 => err_cur_row is correct
        ::std::mem::swap(&mut err_next_row_1, &mut err_next_row_2); // now err_next_row_1 = err_next_row_2 => err_next_row_1 is correct
        for pixel in err_next_row_2.iter_mut() {
            *pixel = (0, 0, 0);
        } // now err_next_row_2 is empty and is correct

        for x in 0..bmp.width() as usize {
            let p = bmp.pixels[x][y];
            let adjusted = (
                p.r as i32 + err_next_1.0 + err_cur_row[x].0,
                p.g as i32 + err_next_1.1 + err_cur_row[x].1,
                p.b as i32 + err_next_1.2 + err_cur_row[x].2,
            );

            let gray = grayscale(&adjusted);
            let (new_val, pixel_error) = if gray < threshold {
                (Pixel::black(), adjusted)
            }
            else {
                (Pixel::white(), (adjusted.0 - 255, adjusted.1 - 255, adjusted.2 - 255))
            };

            let individual_error = (pixel_error.0 / matrix.divisor, pixel_error.1 / matrix.divisor, pixel_error.2 / matrix.divisor);
            err_next_1 = add(&mul(&individual_error, d), &err_next_2);
            err_next_2 = mul(&individual_error, e);

            if x > 1 {
                // able to write pixels 2 to the left of the current; e.g., `f` and `k`
                err_next_row_1[x - 2] = add(&err_next_row_1[x - 2], &mul(&individual_error, f));
                err_next_row_2[x - 2] = add(&err_next_row_2[x - 2], &mul(&individual_error, k));
            }
            if x > 0 {
                // able to write pixels 1 to the left of the current; e.g., `g` and `l`
                err_next_row_1[x - 1] = add(&err_next_row_1[x - 1], &mul(&individual_error, g));
                err_next_row_2[x - 1] = add(&err_next_row_2[x - 1], &mul(&individual_error, l));
            }
            // set pixels in same column; e.g., `h` and `m`
            err_next_row_1[x] = add(&err_next_row_1[x], &mul(&individual_error, h));
            err_next_row_2[x] = add(&err_next_row_2[x], &mul(&individual_error, m));
            if (x as u32) < (bmp.width() - 1) {
                // able to write to pixels 1 to the right of the current; e.g., `i` and `n`
                err_next_row_1[x + 1] = add(&err_next_row_1[x + 1], &mul(&individual_error, i));
                err_next_row_2[x + 1] = add(&err_next_row_2[x + 1], &mul(&individual_error, n));
            }
            if (x as u32) < (bmp.width() - 2) {
                // able to write to pixels 2 right of the current; e.g., `j` and `o`
                err_next_row_1[x + 2] = add(&err_next_row_1[x + 2], &mul(&individual_error, j));
                err_next_row_2[x + 2] = add(&err_next_row_2[x + 2], &mul(&individual_error, o));
            }

            bmp.pixels[x][y] = new_val;
        }
    }
}

fn add(a: &(i32, i32, i32), b: &(i32, i32, i32)) -> (i32, i32, i32) {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn mul(t: &(i32, i32, i32), v: i32) -> (i32, i32, i32) {
    (t.0 * v, t.1 * v, t.2 * v)
}

fn div(t: &(i32, i32, i32), v: i32) -> (i32, i32, i32) {
    (t.0 / v, t.1 / v, t.2 / v)
}
