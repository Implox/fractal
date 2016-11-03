extern crate num;
use self::num::complex::{Complex, Complex64};


/// Determines if a point is within the main cardiod of a mandelbrot.
#[inline]
pub fn check_cardioid(pt: Complex64) -> bool {
    let x = pt.re - 0.25;
    let y = pt.im;
    let q = (x*x + y*y).sqrt();

    return q * (q - x) < 0.25 * y * y;
}

#[inline]
fn apply_mandel_smoothing(pt: Complex64, iter: u32, max: u32) -> f64 {
    let mut iter = iter as f64;
    let max = max as f64;

    if iter < max as f64 {
        iter += 1.0 - (pt.norm().ln()).ln() / (2f64).ln();
    }    
    return iter / max;
}

/// Performs the "escape time" algorithm to evaluate the mandelbrot set,
/// returning the number of iterations required for a given point to 
/// escape the bounds of the set. Points that are in the set never escape, 
/// and will return the given max value.
pub fn eval_mandelbrot(pt: Complex64, max: u32) -> f64 {
    let mut iter = 0;
    let mut z = Complex64::new(0.0, 0.0);

    if check_cardioid(pt) { 
        iter = max;
    } else {
        let bound_radius = (1 << 16) as f64;
        while z.norm_sqr() < bound_radius && iter < max {
            z = z*z + pt;
            iter += 1;
        }
    }

    return apply_mandel_smoothing(z, iter, max);
}

/// Performs the escape-time algorithm for the julia set
pub fn eval_julia(pt: Complex64, max: u32) -> u32 {
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

