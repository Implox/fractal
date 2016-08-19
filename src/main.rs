extern crate num;
extern crate bmp;
extern crate getopts;
extern crate rand;

#[macro_use]
mod gradient;
mod render;
mod fractal;

use bmp::{Image, Pixel};
use num::complex::Complex;
use getopts::Options;
use std::env;
use gradient::{Gradient, Stop};
use render::{Camera};
use fractal::{FractalEval, eval_mandelbrot, eval_julia};

use rand::Rng;
use std::cmp::max;

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

fn make_buddha(cam: Camera, grad: Gradient, samples: usize) -> Image {
    #[inline]
    fn emit_mandelbrot_trajectory(pt: Complex<f64>, max: i32) -> Vec<Complex<f64>> {
        let mut traj = Vec::new();
        let mut iter = 0;
        let mut zr = pt.re;
        let mut zi = pt.im;

        while iter < max {
            let ozr = zr;
            let zis = zi * zi;
            zr = zr * zr;

            if zr + zis <= 4.0 {  
                zr = zr - zis + pt.re;
                zi = (zi * 2.0 * ozr) + pt.im;

                traj.push(Complex::new(zr, zi));
                iter = iter + 1;
            } else { 
                return traj; 
            }
        }
        return vec![];
    }
    
    #[inline]
    fn to_discrete(cam: &Camera, pt: Complex<f64>) -> (u32, u32) {
        let min_pt = cam.transform(0, 0, WIDTH as i32, HEIGHT as i32);
        let max_pt = cam.transform((WIDTH - 1) as i32, (HEIGHT - 1) as i32, 
                                   WIDTH as i32, HEIGHT as i32);
        let delta_x = (max_pt.re - min_pt.re) / WIDTH as f64;
        let delta_y = (max_pt.im - min_pt.im) / HEIGHT as f64;
        let x = (pt.re - min_pt.re) / delta_x;
        let y = (pt.im - min_pt.im) / delta_y;
        return (x as u32, y as u32);
    }

    #[inline]
    fn post_process(img: &mut Image) {
        fn window_avg(x: u32, y:u32, r: u32, img: Image) {
            let mut values = Vec::new();
            for i in 1..r {
                for j in 1..r {
                    if x + i >= img.get_width() || y + j >= img.get_height() {
                        values.push(pix!(0,0,0))
                    } else { 
                        values.push(img.get_pixel(x+i, y+j)); 
                    }

                    if (x as i32 - i as i32) < 0 || (y as i32 - j as i32) < 0 {
                        values.push(pix!(0,0,0))
                    } else {
                        values.push(img.get_pixel(x-i, y-j));
                    }
                }
            }
        }

        let mut avg_r = 0;
        let mut avg_g = 0;
        let mut avg_b = 0;
        {
            let mut total_r: u64 = 0;
            let mut total_g: u64 = 0;
            let mut total_b: u64 = 0;
            let mut sum_r: u64 = 0;
            let mut sum_g: u64 = 0;
            let mut sum_b: u64 = 0;
            for (x, y) in img.coordinates() {
                let Pixel{r, g, b} = img.get_pixel(x, y);
                if r > 0 {
                    sum_r += r as u64;
                    total_r += 1;
                }
                if g > 0 {
                    sum_g += g as u64;
                    total_g += 1;
                }
                if b > 0 {
                    sum_b += b as u64;
                    total_b += 1;
                }
            }
            avg_r = (sum_r / total_r) as u8;
            avg_g = (sum_g / total_g) as u8;
            avg_b = (sum_b / total_b) as u8;
        }

        let adjust = |c, avg| if c > avg { c - avg } else { 0 };
        let scale = |c| (c as f32).log2() / 8.0 * 255.0;

        for (x, y) in img.coordinates() {
            let Pixel{r, g, b} = img.get_pixel(x, y);
            let r = scale(adjust(r, avg_r));
            let g = scale(adjust(g, avg_g));
            let b = scale(adjust(b, avg_b));
            img.set_pixel(x, y, pix!(r as u8, g as u8, b as u8));
        }
    }

    let mut img = Image::new(WIDTH, HEIGHT);
    let mut rng = rand::thread_rng();

    let scale = 1.0;
    let theta_r = 7000;
    let theta_g = 2000;
    let theta_b = 500;

    for _ in 0..samples {
        let rand_x = rng.gen::<u32>() % WIDTH;
        let rand_y = rng.gen::<u32>() % HEIGHT;
        let pt = cam.transform(rand_x as i32, rand_y as i32, WIDTH as i32, HEIGHT as i32);
        let traj = emit_mandelbrot_trajectory(pt, theta_r);
        for traj_pt in traj {
            let (x, y) = to_discrete(&cam, traj_pt);
            if x < WIDTH && y < HEIGHT {
                let Pixel{r, g, b}  = img.get_pixel(x, y);
                if r < 255 {
                    img.set_pixel(x, y, pix!(r+1, g+1, b+1));
                }
            }
        }
    }

    /*for _ in 0..samples {
        let rand_x = rng.gen::<u32>() % WIDTH;
        let rand_y = rng.gen::<u32>() % HEIGHT;
        let pt = cam.transform(rand_x as i32, rand_y as i32, WIDTH as i32, HEIGHT as i32);
        let traj = emit_mandelbrot_trajectory(pt, theta_g);
        for traj_pt in traj {
            let (x, y) = to_discrete(&cam, traj_pt);
            if x < WIDTH && y < HEIGHT {
                let Pixel{r, g, b}  = img.get_pixel(x, y);
                if g < 255 {
                    img.set_pixel(x, y, pix!(r, g+1, b));
                }
            }
        }
    }

    for _ in 0..samples {
        let rand_x = rng.gen::<u32>() % WIDTH;
        let rand_y = rng.gen::<u32>() % HEIGHT;
        let pt = cam.transform(rand_x as i32, rand_y as i32, WIDTH as i32, HEIGHT as i32);
        let traj = emit_mandelbrot_trajectory(pt, theta_b);
        for traj_pt in traj {
            let (x, y) = to_discrete(&cam, traj_pt);
            if x < WIDTH && y < HEIGHT {
                let Pixel{r, g, b}  = img.get_pixel(x, y);
                if b < 255 {
                    img.set_pixel(x, y, pix!(r, g, b+1));
                }
            }
        }
    }*/

    post_process(&mut img);
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
        let eval = if matches.opt_present("j") { 
                eval_julia as FractalEval 
            } else { 
                eval_mandelbrot as FractalEval
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

        let cam = Camera::new(Complex::new(0.0, 0.0), -1.5);
        //let img = make_image(cam, grad, eval);
        let img = make_buddha(cam, grad, 500000);
        let _ = img.save(&format!("{}.bmp", out_file));
    }
}


