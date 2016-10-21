extern crate bmp;

use self::bmp::Pixel;
use std::cmp;

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
    pub fn new(initial: Pixel, stops: Vec<Stop>) -> Gradient {
        Gradient { initial: initial, stops: stops, cache: vec![] } 
    }

    // Builds the cache for a Gradient
    pub fn build_cache(self, cache_size : usize) -> Gradient {
        if self.cache != vec![] {
            return self;
        }

        let mut cache = Vec::with_capacity(cache_size);
        let d = 1f32 / (cache_size as f32);
        for t in 0..cache_size {
            cache.push(self.get_color((t as f32) * d));
        }

        Gradient { initial: self.initial, stops: self.stops, cache: cache }
    }

    pub fn get_color(&self, offset: f32) -> Pixel {
        if self.cache != vec![] {
            let max_idx = self.cache.len() - 1;
            let float_idx = offset * (self.cache.len() as f32);
            let base_idx = cmp::min(float_idx.floor() as usize, max_idx);
            if base_idx < max_idx {
                let base_hue = self.cache[base_idx];
                let mix_hue = self.cache[base_idx+1];
                let mix_amount = float_idx % 1.0;
                return Gradient::mix_color(base_hue, mix_hue, mix_amount);
            }
            return self.cache[base_idx];
        } else {
            let mut prev = self.initial;
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
