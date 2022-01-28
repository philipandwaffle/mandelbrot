use num_complex::Complex64;
mod mandelbrot;

fn main() {
    println!("Hello, world!");

    let m = mandelbrot::Mandelbrot {
        id: 0,
        x_min: -2.0,
        x_max: 1.0,
        y_max: 1.0,
        y_min: -1.0,
        pixel_per_unit: 1000,
        iteration_max: 10000,
        centre_of_barrier: Complex64::new(0.0, 0.0),
    };
    m.gen_image().save("test_003.jpg");
}
