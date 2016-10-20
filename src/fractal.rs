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
    let max = max as f64;

    if check_cardioid(pt) { 
        return 1f32;
    }

    let bound_radius = (1 << 16) as f64;
    let mut iter = 0f64;
    let mut z = Complex64::new(0.0, 0.0);
    while z.re*z.re + z.im*z.im < bound_radius && iter < max {
        z = z*z + pt;
        iter += 1.0;
    }

    if iter < max as f64 {
        iter += 1.0 - (z.norm().ln()).ln() / (2f64).ln();
    }
    
    return (iter / max) as f32;
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

