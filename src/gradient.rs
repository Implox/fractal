extern crate bmp;

use num::Float;
use self::bmp::Pixel;

pub struct Stop {
    pub offset: f32,
    pub color: Pixel
}

impl Stop {
    pub fn new(o: f32, c: Pixel) -> Stop {
        Stop { offset: o, color: c }
    }
}

pub struct Gradient {
    pub initial: Pixel,
    pub stops: Vec<Stop>,
    cache: Vec<Pixel>,
}

impl Gradient {
    pub fn new(initial: Pixel, stops: Vec<Stop>, cache_size: usize) -> Gradient {
        let d = 1.0 / (cache_size as f32);
        let mut cache = Vec::with_capacity(cache_size);
        /*for t in 0..cache_size {
            cache.push(Gradient::get_color(initial, &stops, (t as f32) * d));
        }*/

        Gradient {
            initial: initial,
            stops: stops,
            cache: cache
        }
    }

    pub fn get_color(&self, offset: f32) -> Pixel {
        let mut prev = self.initial;;
        let mut prev_offset = 0.0;
        for t in 0..self.stops.len() {
            let ref stop = self.stops[t];
            let next = stop.color;
            let next_offset = stop.offset;
            if offset < next_offset {
                let amount = (offset - prev_offset) / (next_offset - prev_offset);
                return Gradient::mix_color(prev, next, amount);
            } else { 
                prev = next;
                prev_offset = next_offset;
            }
        }

        // Default value
        let amount = (offset - prev_offset) / (1.0 - prev_offset);
        return Gradient::mix_color(prev, self.initial, amount);
    }

    fn mix_color(a: Pixel, b: Pixel, amount: f32) -> Pixel {
        fn mix(a: u8, a_scale: f32, b: u8, b_scale: f32) -> u8 {
            (((a as f32) * a_scale) + ((b as f32) * b_scale)) as u8
        }
        let af = 1.0 - amount;
        let bf = amount;
        Pixel { r: mix(a.r, af, b.r, bf),
                g: mix(a.g, af, b.g, bf),
                b: mix(a.b, af, b.b, bf) }
    }

    pub fn _get_color(&self, iters: f32) -> Pixel {
        //let scale = (CACHE_SIZE as f32) / self.period;
        let index = iters; //% self.period;
        return self.cache[index as usize];
    }
}
