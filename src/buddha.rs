extern crate rand;

use bmp::{Image, Pixel};
use num::complex::Complex;
use getopts::Options;
use std::env;
use gradient::{Gradient, Stop};
use render::{Camera};
use fractal::{FractalEval, eval_mandelbrot, eval_julia};

use rand::Rng;
use std::cmp::max;

const SCALE:u32 = 2;
const WIDTH:u32 = 1920 * SCALE;
const HEIGHT:u32 = 1080 * SCALE; 

type Trajectory = Vec<Complex<f64>>;

type Coordinate = (u32, u32);
type HSlice = Vec<u32>;
type Histogram = Vec<HSlice>;

enum RenderMode {
    Greyscale(usize),
    RGB(usize, usize, usize),
}

#[inline]
fn init_hist() -> Histogram {
    let mut hist: Histogram = Vec::with_capacity(WIDTH as usize);
    for _ in 0..WIDTH {
        let mut slice: HSlice = Vec::with_capacity(HEIGHT as usize); 
        for _ in 0..HEIGHT { slice.push(0); }
        hist.push(slice);
    }
    return hist;
}

#[inline]
fn emit_mandelbrot_trajectory(pt: Complex<f64>, max: usize) -> Trajectory {
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
fn to_discrete(cam: &Camera, pt: Complex<f64>) -> Coordinate {
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

fn make_hist(cam: &Camera, samples: usize, theta: usize) -> Histogram {
    let mut rng = rand::thread_rng();

    let mut hist = init_hist();

    let mut max: u32 = 0;
    for _ in 0..samples {
        let rand_x = rng.gen::<u32>() % WIDTH;
        let rand_y = rng.gen::<u32>() % HEIGHT;

        let pt = cam.transform(rand_x as i32, rand_y as i32, WIDTH as i32, HEIGHT as i32);
        let traj = emit_mandelbrot_trajectory(pt, theta);

        for traj_pt in traj {
            let (x, y) = to_discrete(&cam, traj_pt);
            if x < WIDTH && y < HEIGHT {
                let x = x as usize;
                let y = y as usize;
                let v = hist[x][y] + 1;
                if v > max { max = v }
                hist[x][y] = v;
            }
        }
    }

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let v = hist[x as usize][y as usize];
            let vFloat = v as f64;
            let maxFloat = max as f64;
            let ratio = vFloat / maxFloat;
            hist[x as usize][y as usize] = (ratio * 255.0) as u32;
        }
    }

    return hist;
}

fn make_render(cam: Camera, samples: usize, mode: RenderMode) -> Image {
    let mut img = Image::new(WIDTH, HEIGHT);
    match mode {
        RenderMode::Greyscale(theta) => {
            let hist = make_hist(&cam, samples, theta);
            for x in 0..WIDTH {
                for y in 0..HEIGHT {
                    let v = hist[x as usize][y as usize];
                    img.set_pixel(x, y, pix!(v as u8, v as u8, v as u8));
                }
            }
            return img;
        },
        RenderMode::RGB(theta_r, theta_g, theta_b) => {
            let hist_r = make_hist(&cam, samples, theta_r);
            let hist_g = make_hist(&cam, samples, theta_g);
            let hist_b = make_hist(&cam, samples, theta_b);

            for x in 0..WIDTH {
                for y in 0..HEIGHT {
                    let vr = hist_r[x as usize][y as usize];
                    let vg = hist_g[x as usize][y as usize];
                    let vb = hist_b[x as usize][y as usize];
                    img.set_pixel(x, y, pix!(vr as u8, vg as u8, vb as u8));
                }
            }
            return img;
        },
    }
}

pub fn make_buddha(cam: Camera, samples: usize) -> Image {
    let img = make_render(cam, samples, RenderMode::Greyscale(2000));
    //let img = make_render(cam, samples, RenderMode::RGB(7000, 3000, 500));

    //post_process(&mut img);
    return img;
}
