use num::{Complex, traits::bounds};
use std::str::FromStr;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use std::env;

///  マンデルブロ集合はzが発散しない複素数cの集合である。
///  半径2の円の外側に出たら発散すると知られていることから、zの絶対値の2乗が4を超えたら発散とみなす。
fn complex_aquare_add_loop(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

/// "400,600"や"1.0x0.75"のような文字列をパースしてタプルに変換する。
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None,
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("0.5x", 'x'), None);
}

/// "1.0,0.75"のような文字列をパースして複素数に変換する。
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair::<f64>(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.0,0.75"), Some(Complex { re: 1.0, im: 0.75 }));
    assert_eq!(parse_complex("1.0x0.75"), None);
}

/// 画像のピクセル座標を複素平面上の点に変換する。
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);

    Complex {
        re: upper_left.re + (pixel.0 as f64) * width / (bounds.0 as f64),
        im: upper_left.im - (pixel.1 as f64) * height / (bounds.1 as f64),
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 100),
            (25, 75),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 },
        ),
        Complex { re: -0.5, im: -0.5 }
    );
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
){
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match complex_aquare_add_loop(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize))
    -> Result<(), std::io::Error> {
        let output = File::create(filename)?;
        let encoder = PNGEncoder::new(output);
        encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;
        Ok(())
}

fn _single_thread_mandelbrot(pixels: &mut [u8], bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>){
    render(pixels, bounds, upper_left, lower_right);
}

fn multi_thread_mandelbrot(pixels: &mut [u8], bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>){
    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;

    {
        let bands : Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner|
            for (i, band) in bands.into_iter().enumerate(){
                let top = i * rows_per_band;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(
                    bounds,
                    (0, top),
                    upper_left,
                    lower_right,
                );
                let band_lower_right = pixel_to_point(
                    bounds,
                    (bounds.0, top + height),
                    upper_left,
                    lower_right,
                );
                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        ).unwrap();
    }
}

///シングルスレッド版
/// 実行例 
/// cargo run mandel.png 4000x3000 -1.20,0.35 -1,0.20
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT\n", args[0]);
        eprintln!("Example: {} mandelbrot.png 400x600 -2.0,1.0 1.0,-1.0", args[0]);
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    // _single_thread_mandelbrot(&mut pixels, bounds, upper_left, lower_right);
    multi_thread_mandelbrot(&mut pixels, bounds, upper_left, lower_right);

    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}
