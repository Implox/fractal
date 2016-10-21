extern crate num;
extern crate bmp;
extern crate rayon;

mod gradient;
mod render;
mod fractal;

use bmp::{Image, Pixel};
use rayon::par_iter::*;
use num::complex::Complex;
use gradient::{Gradient, Stop};
use render::{Camera};
use fractal::*;

use std::thread;
use std::sync::Arc;
use std::sync::mpsc::channel;

const SCALE:usize = 2;
const WIDTH:usize  = 1920 * SCALE;
const HEIGHT:usize = 1080 * SCALE; 

const MAX_ITERS:u32 = 1000;

fn pix(r: u8, g: u8, b: u8) -> Pixel {
    Pixel { r: r, g: g, b: b }
}

fn make_plot_rayon<F>(cam: &Camera, eval: Arc<F>) -> Vec<Vec<f32>> 
where F: 'static + Send + Sync + Fn(Complex<f64>, u32) -> f32 {
    let (origin, p_size) = cam.find_origin_and_pixel_size(WIDTH as u32, HEIGHT as u32);

    let mut plot = (0..WIDTH).map(|_| {
        (0..HEIGHT).map(|_| 0.0).collect::<Vec<f32>>()
    }).collect::<Vec<Vec<f32>>>();

    plot.par_iter_mut().enumerate().for_each(
        |(row_idx, mut row)| {
            let re = origin.re + p_size * (row_idx as f64);
            *row = 
                (0..HEIGHT)
                .map(|col_idx| {
                    let im = origin.im + p_size * (col_idx as f64);
                    let pt = Complex::new(re, im);
                    eval(pt, MAX_ITERS) })
                .collect();
    });

    return plot;
}

fn make_image(plot: &Vec<Vec<f32>>, grad: Gradient) -> Image {
    let mut img = Image::new(WIDTH as u32, HEIGHT as u32);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let hue = plot[x][y];
            let pixel = grad.get_color(hue);
            img.set_pixel(x as u32, y as u32, pixel);
        }
    }
    return img;
}

fn main() {
    let grad = {
        let initial = pix(0, 0, 0);
        let stops = vec![
            Stop::new(0.025, pix(255,   0,   0)),
            Stop::new(0.050, pix(255, 255,   0)),
            Stop::new(0.100, pix(  0, 255,   0)),
            Stop::new(0.105, pix(  0, 255, 255)),
            Stop::new(0.110, pix(  0,   0, 255)),
            Stop::new(0.200, pix(255,   0,   0)),
        ];
        Gradient::new(initial, stops).build_cache(10000)
    };
    
    let cam = Camera::new(Complex::new(-0.6, 0.0), -1.0);
    let plot = make_plot_rayon(&cam, Arc::new(eval_mandelbrot));

    let img = make_image(&plot, grad);
    let _ = img.save("img.bmp");
}
