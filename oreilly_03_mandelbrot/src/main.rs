use num::Complex;
use std::str::FromStr;

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