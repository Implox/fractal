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

const SCALE:u32 = 4;
const WIDTH:u32  = 1920 * SCALE;
const HEIGHT:u32 = 1080 * SCALE; 

const MAX_ITERS:u32 = 1000;

fn pix(r: u8, g: u8, b: u8) -> Pixel {
    Pixel { r: r, g: g, b: b }
}

#[inline]
fn get_row_points(origin: Complex<f64>, p_size: f64, col: u32) -> Vec<Complex<f64>> {
    let mut points = Vec::with_capacity(WIDTH as usize);
    let i = origin.im + p_size * (col as f64);
    for x in 0..WIDTH {
        let r = origin.re + p_size * (x as f64);
        points.push(Complex::new(r, i));
    }
    return points;
}

fn make_image_parallel<F>(cam: &Camera, grad: Gradient, eval: Arc<F>) -> Image 
where F: 'static + Send + Sync + Fn(Complex<f64>, u32) -> u32 {
    let n_threads = 2;

    let (origin, p_size) = cam.find_origin_and_pixel_size(WIDTH, HEIGHT);
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

    let mut img = Image::new(WIDTH, HEIGHT);
    println!("Starting to receive thread output");
    for _ in 0..HEIGHT {
        let result = agg_chan_out.recv().unwrap();
        for (x, y, iters) in result {
            let pixel = grad.get_color(iters as f32);
            img.set_pixel(x, y, pixel);
        }
    }

    println!("Checking all threads have ended.");
    for t in threads {
        t.join().unwrap();
    }
    println!("Finished generating image!");

    return img;
}

fn make_image(cam: &Camera, grad: Gradient, eval: &Fn(Complex<f64>, u32) -> u32) -> Image {
    let mut img = Image::new(WIDTH, HEIGHT);
    let (tl, p_size) = cam.find_origin_and_pixel_size(WIDTH, HEIGHT);
    for y in 0..HEIGHT {
        let i = tl.im + p_size * (y as f64);
        for x in 0..WIDTH {
            let r = tl.re + p_size * (x as f64);
            let iter = eval(Complex::new(r, i).scale(4.0), MAX_ITERS);
            let pix = grad.get_color(iter as f32);
            img.set_pixel(x, y, pix);
        }
    }
    return img;
}

fn main() {
    let grad = {
        let period = 100.0;
        let initial                   = pix(  0,   0,   0);
        let stops = vec![Stop::new(0.3, pix(255,   0,   0)),
                         Stop::new(0.5, pix(255, 255,   0)),
                         Stop::new(0.7, pix(  0, 255,   0)),
                         Stop::new(0.9, pix(  0, 255, 255))];
        let end                       = pix(  0,   0, 255);
        Gradient::new(period, initial, stops, end)
    };
    
    let cam = Camera::new(Complex::new(-0.6, 0.0), -1.0);
    //let img = make_image(&cam, grad, &eval_mandelbrot);
    let img = make_image_parallel(&cam, grad, Arc::new(eval_mandelbrot));
    let _ = img.save("img2.bmp");
}
