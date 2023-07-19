pub mod complex;
pub mod util;
use complex::Complex;
use std::f32::consts::PI;

pub fn dft(x: &[Complex]) -> Vec<Complex> {
    let mut xout: Vec<Complex> = vec![Complex::new(0., 0.); x.len()];

    let n: usize = x.len();
    for k in 0..n {
        let mut sum: Complex = Complex::new(0., 0.);

        for t in 0..n {
            // e^((2\pi i k)/n)
            let exp: f32 = 2. * PI * t as f32 * k as f32 / n as f32;

            // Euler's formula
            // e^(-ix) = cos(x) - i sin(x)
            // sum = (x.re + i x.im) * (cos(exp) - i sin(exp))
            // sum = x.re * cos(exp) + i x.im * cos(exp) - i x.re * sin(exp) + x.im * sin(exp)
            // sum.re = x.re * cos(exp) + x.im * sin(exp)
            // sum.im = x.im * cos(exp) - x.re * sin(exp)
            sum.re += x[t].re * f32::cos(exp) + x[t].im * f32::sin(exp);
            sum.im += x[t].im * f32::cos(exp) - x[t].re * f32::sin(exp);
        }

        xout[k] = sum;
    }

    xout
}



