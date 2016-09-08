extern crate num;
extern crate bmp;
extern crate getopts;
extern crate rand;

#[macro_use]
mod gradient;
mod render;
mod fractal;
mod buddha;

use bmp::{Image, Pixel};
use num::complex::Complex;
use getopts::Options;
use std::env;
use gradient::{Gradient, Stop};
use render::{Camera};
use fractal::{FractalEval, eval_mandelbrot, eval_julia};
use buddha::*;

const SCALE:u32 = 1;
const WIDTH:u32  = 1920 * SCALE;
const HEIGHT:u32 = 1080 * SCALE; 

const MAX_ITERS:i32 = 1000;

fn make_image(cam: Camera, grad: Gradient, eval: FractalEval) -> Image {
    let mut img = Image::new(WIDTH, HEIGHT);
    for (x, y) in img.coordinates() {
        let pt = cam.transform(x as i32, y as i32, WIDTH as i32, HEIGHT as i32);
        let iter = eval(pt.scale(4.0), MAX_ITERS);
        let col = grad.get_color(iter as f32);
        img.set_pixel(x, y, col);
    }
    return img;
}

/// Displays usage information
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let default_outfile = "fractal";

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    let o_desc = format!("Set the output file name (default: {})", default_outfile);
    opts.optopt("o", "outfile", &o_desc, "NAME");

    let w_desc = "Set the width of output image (default: 800)";
    opts.optopt("W", "width", &w_desc, "");

    let h_desc = "Set the height of output image (default: 600)";
    opts.optopt("H", "height", &h_desc, "");

    opts.optflag("m", "mandelbrot", "Render a Mandlebrot fractal (default)");
    opts.optflag("j", "julia",      "Render a Julia fractal");
    opts.optflag("h", "help",       "Display usage information");
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(e) => { panic!(e.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    } else {
        let eval = match matches.opt_present("j") {
            true  => eval_julia as FractalEval,
            false => eval_mandelbrot as FractalEval,
        };

        let out_file = matches.opt_str("o").unwrap_or(default_outfile.to_string());
        
        let grad = {
            let period = 40.0;
            let initial                   = pix!(  0,   0,   0);
            let stops = vec![Stop::new(0.3, pix!(255,   0,   0)),
                             Stop::new(0.5, pix!(255, 255,   0)),
                             Stop::new(0.7, pix!(  0, 255,   0)),
                             Stop::new(0.9, pix!(  0, 255, 255))];
            let end                       = pix!(  0,   0, 255);
            Gradient::new(period, initial, stops, end)
        };

        let cam = Camera::new(Complex::new(-0.4, 0.0), -1.0);
        //let img = make_image(cam, grad, eval);
        let img = make_buddha(cam, 1000000);
        let _ = img.save(&format!("{}.bmp", out_file));
    }
}


