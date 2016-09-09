extern crate bmp;

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

const CACHE_SIZE: usize = 10000;

pub struct Gradient {
    pub period: f32,
    pub initial: Pixel,
    pub stops: Vec<Stop>,
    pub end: Pixel,
    cache: [Pixel; CACHE_SIZE]
}

impl Gradient {
    pub fn new(period: f32, initial: Pixel, stops: Vec<Stop>, end: Pixel) -> Gradient {
        let blank = Pixel { r: 0, g: 0, b: 0 };
        let d = 1.0 / (CACHE_SIZE as f32);
        let mut cache: [Pixel; CACHE_SIZE] = [blank; CACHE_SIZE];
        for t in 0..CACHE_SIZE-1 {
            cache[t] = Gradient::get_period_color(initial, &stops, (t as f32) * d);
        }

        Gradient {
            period: period,
            initial: initial,
            stops: stops,
            end: end,
            cache: cache
        }
    }

    fn get_period_color(initial: Pixel, stops: &[Stop], offset: f32) -> Pixel {
        let mut prev = initial;;
        let mut prev_offset = 0.0;
        for t in 0..stops.len() {
            let ref stop = stops[t];
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
        let amount = (offset - prev_offset) / (1.0 - prev_offset);
        return Gradient::mix_color(prev, initial, amount);
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

    pub fn get_color(&self, iters: f32) -> Pixel {
        let scale = (CACHE_SIZE as f32) / self.period;
        let index = (iters % self.period) * scale;
        return self.cache[index as usize];
    }
}
