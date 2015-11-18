extern crate num;
extern crate bmp;

mod gradient;
mod render;
mod fractal;

use bmp::{Image, Pixel};
use num::complex::Complex;
use gradient::{Gradient, Stop};
use render::{Camera};
use fractal::{eval_julia, eval_mandelbrot};

const SCALE:u32 = 1;
const WIDTH:u32  = 3840 * SCALE;
const HEIGHT:u32 = 2160 * SCALE; 

const MAX_ITERS:i32 = 100;

fn pix(r: u8, g: u8, b: u8) -> Pixel {
    Pixel { r: r, g: g, b: b }
}

fn make_image(cam: Camera, grad: Gradient, eval: &Fn(Complex<f64>, i32) -> i32) -> Image {
    let mut img = Image::new(WIDTH, HEIGHT);
    let (tl, p_size) = cam.find_origin_and_pixel_size(WIDTH, HEIGHT);
    for y in 0..HEIGHT-1 {
        let i = tl.im + p_size * (y as f64);
        for x in 0..WIDTH-1 {
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
    
    let cam = Camera::new(Complex::new(-0.0, 0.0), 1.15);
    let img = make_image(cam, grad, &eval_julia);
    let _ = img.save("img.bmp");
}
