extern crate num;

use self::num::complex::Complex;

/// Represents a view of the complex plane
pub struct Camera {
    pub center: Complex<f64>,
    pub zoom: f64
}

impl Camera {
    pub fn new(center: Complex<f64>, zoom: f64) -> Camera {
        Camera { center: center, zoom: zoom }
    }

    /// Transforms a point from a discrete coordinate space
    pub fn transform(&self, x: i32, y: i32, width: i32, height: i32) -> Complex<f64> {
        let x = ((x as f64) + 0.5) / (width as f64) - 0.5;
        let y = ((y as f64) + 0.5) / (height as f64) - 0.5;
        let z = f64::powf(0.5, self.zoom);
        let ar = (width as f64) / (height as f64);
        let offset = {
            if ar > 1.0 { Complex::new(2.0 * x * z, 2.0 * y * z / ar) } 
            else { Complex::new(2.0 * x * z * ar, 2.0 * y * z) }
        };
        return self.center + offset;
    }

    /// Finds the center of the top-left corner and the pixel size for a
    /// discrete coordinate space using this camera
    pub fn find_origin_and_pixel_size(&self, w: u32, h: u32) -> (Complex<f64>, f64) {
        let z = f64::powf(0.5, self.zoom);
        let pix_size: f64;
        let offset: Complex<f64>;
        if w > h {
            pix_size = z * 2.0 / (w as f64);
            offset = Complex {
                re: pix_size * 0.5 - z, 
                im: ((w - h) as f64) * pix_size * 0.5 + pix_size * 0.5 - z
            };
        } else {
            pix_size = z * 2.0 / (h as f64);
            offset = Complex {
                re: ((h - 2) as f64) * pix_size * 0.5 + pix_size * 0.5 - z,
                im: pix_size * 0.5 - z
            };
        }
        (self.center + offset, pix_size)
    }
}
