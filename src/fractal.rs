extern crate num;
use self::num::complex::*;


pub fn check_cardioid(pt: Complex<f64>) -> bool {
    let x = pt.re - 0.25;
    let y = pt.im;
    let q = (x*x + y*y).sqrt();

    return q * (q - x) < 0.25 * y * y;
}

/// Performs the "escape time" algorithm to evaluate the mandelbrot set,
/// returning the number of iterations required for a given point to 
/// escape the bounds of the set. Points that are in the set never escape, 
/// and will return the given max value.
pub fn eval_mandelbrot(pt: Complex<f64>, max: u32) -> f32 {
    if check_cardioid(pt) { 
        return max as f32;
    } else if pt.re < -2.5 || pt.re > 1.0 || pt.im < -1.0 || pt.im > 1.0 {
        return 0.0;
    }

    let mut iter = 0f64;
    let mut z = Complex64::new(0.0, 0.0);
    while z.re*z.re + z.im*z.im < (1 << 16) as f64 && iter < max as f64 {
        z = z*z + pt;
        iter += 1.0;
    }

    if iter < max as f64 {
        let log2: f64 = (2f64).ln();
        let log_zn = (z.re*z.re + z.im*z.im).ln() / 2.0;
        let nu = (log_zn / log2).ln() / log2;
        iter = iter + 1.0 - nu;
        return (iter + 1.0 - nu) as f32;
    }
    return iter as f32;
}

/// Performs the same escape-time algorithm as above, but for the julia set
pub fn eval_julia(pt: Complex<f64>, max: u32) -> u32 {
    let mut z = pt;
    let mut i = 0;
    let c = Complex::new(-0.4, 0.6);
    for t in 0..max {
        if z.norm() <= 2.0 { 
            z = z * z + c;
            i = t;
        } else { break; }
    }
    return i;
}

