#[macro_use]
extern crate clap;

extern crate num;
extern crate bmp;
extern crate rayon;

mod gradient;
mod render;
mod fractal;
mod cli;

use bmp::{Image, Pixel};
use rayon::par_iter::*;
use num::complex::Complex;
use gradient::{Gradient, Stop};
use render::{Camera};
use fractal::*;
use cli::{get_app};
use std::sync::Arc;

const MAX_ITERS:u32 = 1000;

fn pix(r: u8, g: u8, b: u8) -> Pixel {
    Pixel { r: r, g: g, b: b }
}

fn make_plot<F>(cam: &Camera, eval: Arc<F>, width: usize, height: usize) -> Vec<Vec<f32>> 
where F: 'static + Send + Sync + Fn(Complex<f64>, u32) -> f32 {
    let (origin, p_size) = cam.find_origin_and_pixel_size(width as u32, height as u32);

    let mut plot = (0..width).map(|_| {
        (0..height).map(|_| 0.0).collect::<Vec<f32>>()
    }).collect::<Vec<Vec<f32>>>();

    plot.par_iter_mut().weight_max().enumerate().for_each(
        |(row_idx, mut row)| {
            let re = origin.re + p_size * (row_idx as f64);
            *row = 
                (0..height).map(|col_idx| {
                    let im = origin.im + p_size * (col_idx as f64);
                    let pt = Complex::new(re, im);
                    eval(pt, MAX_ITERS) })
                .collect();
    });

    return plot;
}

fn make_image(plot: &Vec<Vec<f32>>, grad: Gradient, width: usize, height: usize) -> Image {
    let mut img = Image::new(width as u32, height as u32);
    for y in 0..height {
        for x in 0..width {
            let hue = plot[x][y];
            let pixel = grad.get_color(hue);
            img.set_pixel(x as u32, y as u32, pixel);
        }
    }
    return img;
}

fn main() {
    let matches = get_app().get_matches();
    let width = value_t_or_exit!(matches, "width", usize);
    let height = value_t_or_exit!(matches, "height", usize);
    let cam_re = value_t!(matches, "real", f64).unwrap_or_else(|_| -0.6);
    let cam_im = value_t!(matches, "imaginary", f64).unwrap_or_else(|_| 0.0);
    let zoom = value_t!(matches, "zoom", f64).unwrap_or_else(|_| -1.0);
    let mut outfile = value_t_or_exit!(matches, "out_file", String);
    outfile.push_str(".bmp");

    let grad = {
        let initial = pix(0, 0, 0);
        let stops = vec![
            Stop::new(0.025, pix(255,   0,   0)),
            Stop::new(0.050, pix(255, 255,   0)),
            Stop::new(0.100, pix(  0, 255,   0)),
            Stop::new(0.105, pix(  0, 255, 255)),
            Stop::new(0.110, pix(  0,   0, 255)),
            Stop::new(0.200, pix(  0,   0,   0)),
        ];
        Gradient::new(initial, stops).build_cache(1000)
    };
    
    let cam = Camera::new(Complex::new(cam_re, cam_im), zoom);
    let plot = make_plot(&cam, Arc::new(eval_mandelbrot), width, height);

    let img = make_image(&plot, grad, width, height);
    let _ = img.save(outfile.as_str());
}
