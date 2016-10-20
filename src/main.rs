extern crate num;
extern crate bmp;

mod gradient;
mod render;
mod fractal;

use bmp::{Image, Pixel};
use num::complex::Complex;
use gradient::{Gradient, Stop};
use render::{Camera};
use fractal::*;

use std::thread;
use std::sync::Arc;
use std::sync::mpsc::channel;

const SCALE:usize = 4;
const WIDTH:usize  = 1920 * SCALE;
const HEIGHT:usize = 1080 * SCALE; 

const MAX_ITERS:u32 = 1000;

fn pix(r: u8, g: u8, b: u8) -> Pixel {
    Pixel { r: r, g: g, b: b }
}

#[inline]
fn get_row_points(origin: Complex<f64>, p_size: f64, col: usize) -> Vec<Complex<f64>> {
    let mut points = Vec::with_capacity(WIDTH as usize);
    let i = origin.im + p_size * (col as f64);
    for x in 0..WIDTH {
        let r = origin.re + p_size * (x as f64);
        points.push(Complex::new(r, i));
    }
    return points;
}

fn make_plot<F>(cam: &Camera, eval: Arc<F>) -> Vec<Vec<f32>> 
where F: 'static + Send + Sync + Fn(Complex<f64>, u32) -> f32 {
    let n_threads = 2;

    let (origin, p_size) = cam.find_origin_and_pixel_size(WIDTH as u32, HEIGHT as u32);
    let (agg_chan_in, agg_chan_out) = channel();

    let mut threads = Vec::new();
    for thread in 0..n_threads {
        let agg_chan_in = agg_chan_in.clone();
        let eval = eval.clone();

        threads.push(thread::spawn(move || {
            println!("Thread {} starting.", thread);
            for data_idx in 0..(HEIGHT / n_threads) {
                let y = (data_idx * n_threads) + thread;
                let mut results = Vec::with_capacity(WIDTH as usize);
                let ref row = get_row_points(origin, p_size, y);
                for x in 0..row.len() {
                    results.push((x as u32, y, eval(row[x], MAX_ITERS)));
                }
                agg_chan_in.send(results).unwrap();
            }
            println!("Thread {} ending.", thread);
        }));
    }

    let mut plot = (0..WIDTH).map(|_| {
        (0..HEIGHT).map(|_| 0.0).collect::<Vec<f32>>()
    }).collect::<Vec<Vec<f32>>>();

    println!("Starting to receive thread output");
    for _ in 0..HEIGHT {
        let result = agg_chan_out.recv().unwrap();
        for (x, y, iters) in result {
            plot[x as usize][y as usize] = iters;
        }
    }

    println!("Checking all threads have ended.");
    for t in threads {
        t.join().unwrap();
    }
    println!("Finished generating plot!");

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
    let plot = make_plot(&cam, Arc::new(eval_mandelbrot));

    let img = make_image(&plot, grad);
    let _ = img.save("img.bmp");
}
