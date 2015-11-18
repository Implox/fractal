extern crate num;
use self::num::complex::Complex;

/// Performs the "escape time" algorithm to evaluate the mandelbrot set,
/// returning the number of iterations required for a given point to 
/// escape the bounds of the set. Points that are in the set never escape, 
/// and will return the given max value.
pub fn eval_mandelbrot(pt: Complex<f64>, max: i32) -> i32 {
    let mut iter = 0;
    let pt = pt + Complex::new(-0.6, 0.0); // offset for aesthetics
    let mut zr = pt.re;
    let mut zi = pt.im;

    while iter < max {
        let ozr = zr;
        let zis = zi * zi;
        zr = zr * zr;

        if zr + zis <= 4.0 {  
            zr = zr - zis + pt.re;
            zi = (zi * 2.0 * ozr) + pt.im;
            iter = iter + 1;
        } else { break; }
    }
    return iter;
}

/// Performs the same escape-time algorithm as above, but for the julia set
pub fn eval_julia(pt: Complex<f64>, max: i32) -> i32 {
    let mut z = pt;
    let mut i = 0;
    let c = Complex::new(-0.4, 0.6);
    for t in 0..max-1 {
        if z.norm() <= 2.0 { 
            z = z * z + c;
            i = t;
        } else { break; }
    }
    return i;
}

