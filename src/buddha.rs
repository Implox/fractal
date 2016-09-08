extern crate rand;

use bmp::{Image, Pixel};
use num::complex::Complex;
use std::env;
use std::collections::HashMap;
use fractal::{eval_mandelbrot};
use render::{Camera};

use rand::Rng;
use std::cmp::max;

const SCALE:u32 = 4;
const WIDTH:u32 = 1600 * SCALE;
const HEIGHT:u32 = 900 * SCALE; 

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
fn emit_mandelbrot_trajectory(pt: Complex<f64>, max: usize, min: usize) -> Trajectory {
    let num_iters = eval_mandelbrot(pt, max as i32) as usize;
    if num_iters == max || num_iters < min {
        return vec![];
    }

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

            let c = Complex::new(zr, zi);
            let c_neg = Complex::new(zr, -zi);
            traj.push(c);
            traj.push(c_neg);
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
fn post_process(hist: &mut Histogram) {
    let mut color_hist = Vec::new();

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            color_hist.push(hist[x as usize][y as usize]);
        }
    }
    color_hist.sort();
    color_hist.dedup();

    println!("Ping!");
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let mut index = 0;
            while index < color_hist.len() {
                if color_hist[index] == hist[x as usize][y as usize] {
                    hist[x as usize][y as usize] = index as u32;
                    break;
                } else {
                    index += 1;
                }
            }
        }
    }
}

fn make_hist(cam: &Camera, samples: usize, theta: usize, min: usize) -> Histogram {
    let mut rng = rand::thread_rng();

    let mut hist = init_hist();

    //let prev_samples = HashMap::new();

    for _ in 0..samples {
        let rand_x = rng.gen::<u32>() % WIDTH;
        let rand_y = rng.gen::<u32>() % HEIGHT;


        let pt = cam.transform(rand_x as i32, rand_y as i32, WIDTH as i32, HEIGHT as i32);
        let traj = emit_mandelbrot_trajectory(pt, theta, min);

        for traj_pt in traj {
            let (x, y) = to_discrete(&cam, traj_pt);
            if x < WIDTH && y < HEIGHT {
                let x = x as usize;
                let y = y as usize;
                let v = hist[x][y] + 1;
                hist[x][y] = v;
            }
        }
    }

    post_process(&mut hist);

    let mut max = 0;
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            if hist[x as usize][y as usize] > max {
                max = hist[x as usize][y as usize];
            }
        }
    }

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let sqrt_max = (max as f64).sqrt();
            let sqrt_val = (hist[x as usize][y as usize] as f64).sqrt();
            hist[x as usize][y as usize] = (255.0 * (sqrt_val / sqrt_max)).trunc() as u32;
        }
    }

    return hist;
}

fn make_render(cam: Camera, samples: usize, mode: RenderMode) -> Image {
    let mut img = Image::new(WIDTH, HEIGHT);
    match mode {
        RenderMode::Greyscale(theta) => {
            let hist = make_hist(&cam, samples, theta, 10000);
            for x in 0..WIDTH {
                for y in 0..HEIGHT {
                    let mut v = hist[x as usize][y as usize];
                    img.set_pixel(x, y, pix!(v as u8, v as u8, v as u8));
                }
            }
            return img;
        },
        RenderMode::RGB(theta_r, theta_g, theta_b) => {
            let hist_r = make_hist(&cam, samples, theta_r, theta_r / 20);
            let hist_g = make_hist(&cam, samples, theta_g, theta_g / 20);
            let hist_b = make_hist(&cam, samples, theta_b, theta_b / 20);

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
    let img = make_render(cam, samples, RenderMode::Greyscale(1000000));
    //let img = make_render(cam, samples, RenderMode::RGB(12000, 9000, 8000));

    //post_process(&mut img);
    return img;
}
