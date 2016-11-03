#[macro_use]
extern crate clap;
#[macro_use]
extern crate bmp;
extern crate num;
extern crate rayon;

mod gradient;
mod camera;
mod fractal;
mod render;

use bmp::{Pixel};
use num::complex::Complex;
use std::sync::Arc;

use gradient::{Gradient, Stop};
use camera::{Camera};
use render::*;
use fractal::*;

fn main() {
    let matches = clap_app!(fractal =>
        (version: "1.0")
        (author: "A. Flores")
        (about: "VERY fast fractal generation at incredible hihg speed")
        (@arg width: -w --width +takes_value +required "Sets the width of the output image in pixels")
        (@arg height: -h --height +takes_value +required "Sets the height of the output image in pixels")
        (@arg real: -re --real +takes_value "Sets the position of the camera on the real axis (default: -1.6)")
        (@arg imaginary: -im --imag +takes_value "Sets the position of the camera on the imaginary axis (default: 0.0)")
        (@arg zoom: -z --zoom +takes_value "Sets the zoom level of the camera (default: -1.0)")
        (@arg file: -o --file +takes_value +required "The name of the file to output without extension (as a bmp)"))
        .get_matches();

    let width = value_t_or_exit!(matches, "width", usize);
    let height = value_t_or_exit!(matches, "height", usize);
    let cam_re = value_t!(matches, "real", f64).unwrap_or_else(|_| -0.6);
    let cam_im = value_t!(matches, "imaginary", f64).unwrap_or_else(|_| 0.0);
    let zoom = value_t!(matches, "zoom", f64).unwrap_or_else(|_| -1.0);
    let mut outfile = value_t_or_exit!(matches, "file", String);
    outfile.push_str(".bmp");

    let grad = {
        let initial = px!(0, 0, 0);
        let stops = vec![
            Stop::new(0.025, px!(255,   0,   0)),
            Stop::new(0.050, px!(255, 255,   0)),
            Stop::new(0.100, px!(  0, 255,   0)),
            Stop::new(0.105, px!(  0, 255, 255)),
            Stop::new(0.150, px!(  0,   0, 255)),
            Stop::new(0.250, px!(  0, 255,   0)),
            Stop::new(0.300, px!(255, 255,   0)),
            Stop::new(0.350, px!(255,   0,   0)),
            Stop::new(1.000, px!(  0,   0,   0)),
        ];
        Gradient::new(initial, stops).build_cache(1000)
    };
    
    let cam = Camera::new(Complex::new(cam_re, cam_im), zoom);
    let plot = make_plot(&cam, Arc::new(eval_mandelbrot), width, height);

    let img = make_image(&plot, grad, width, height);
    let _ = img.save(outfile.as_str());
}
