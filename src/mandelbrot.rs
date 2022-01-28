use image::{ImageBuffer, Luma};
use num_complex::Complex64;
use std::ops::{Add, Mul};

pub struct Mandelbrot {
    pub id: u32,
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub pixel_per_unit: u32,
    pub iteration_max: u32,
    pub centre_of_barrier: Complex64,
}
impl Mandelbrot {
    pub fn new(
        id: &u32,
        x_min: &f64,
        x_max: &f64,
        y_min: &f64,
        y_max: &f64,
        pixel_per_unit: &u32,
        iteration_max: &u32,
        centre_of_barrier: &Complex64,
    ) -> Mandelbrot {
        return Mandelbrot {
            id: *id,
            x_min: *x_min,
            x_max: *x_max,
            y_min: *y_min,
            y_max: *y_max,
            pixel_per_unit: *pixel_per_unit,
            iteration_max: *iteration_max,
            centre_of_barrier: *centre_of_barrier,
        };
    }

    pub fn gen_image(&self) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let x_pixels: u32 = (self.x_max.abs() + self.x_min.abs()) as u32 * self.pixel_per_unit;
        let y_pixels: u32 = (self.y_max.abs() + self.y_min.abs()) as u32 * self.pixel_per_unit;

        let mut progress: f64 = 0.0;
        // Construct a new by repeated calls to the supplied closure.
        let mut img = ImageBuffer::from_fn(x_pixels, y_pixels, |x, y| {
            let point = Complex64::new(
                (x as f64 / self.pixel_per_unit as f64) - self.x_min.abs(),
                (y as f64 / self.pixel_per_unit as f64) - self.y_min.abs(),
            );

            let r = Mandelbrot::calc_r_value(self, &point);

            let temp_p: f64 = (x * y) as f64 / (x_pixels * y_pixels) as f64;

            if (temp_p - progress) > 0.01 {
                progress = temp_p;
                println!("Progress of id:{} is {}%", self.id, temp_p * 100.0);
            }

            let scaled_r = r as f64 / self.iteration_max as f64;
            let luma = (scaled_r * 255.0).round() as u8;

            return image::Luma([luma as u8]);
        });
        return img;
    }

    fn calc_r_value(&self, point: &Complex64) -> u32 {
        let mut z: Complex64 = self.centre_of_barrier.clone();
        let mut n: u32 = 0;
        while (z.norm() <= 2.5) && n < self.iteration_max {
            z = z.mul(z).add(point);
            n += 1;
        }
        return n;
    }
}
