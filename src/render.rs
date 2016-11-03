use bmp::{Image};
use rayon::par_iter::*;
use num::complex::Complex;
use gradient::{Gradient};
use camera::{Camera};
use std::sync::Arc;

const MAX_ITERS:u32 = 1000;

pub fn make_plot<F>(cam: &Camera, eval: Arc<F>, width: usize, height: usize) -> Vec<Vec<f64>> 
where F: 'static + Send + Sync + Fn(Complex<f64>, u32) -> f64 {
    let (origin, p_size) = cam.find_origin_and_pixel_size(width as u32, height as u32);

    let mut plot:Vec<Vec<f64>> = (0..width).map(|_| {
        (0..height).map(|_| 0.0).collect()
    }).collect();

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

pub fn make_image(plot: &Vec<Vec<f64>>, grad: Gradient, width: usize, height: usize) -> Image {
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

