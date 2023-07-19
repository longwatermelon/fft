use fft::complex::Complex;
use macroquad::prelude::*;
use std::f32::consts::PI;

fn map_y(y: f32, min: f32, max: f32) -> f32 {
    550. - (y - min) / (max - min) * 500.
}

fn plot(y: &[f32], color: Color) {
    let min: f32 = *y.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max: f32 = *y.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

    let mut prev_x: f32 = 50.;
    let mut prev_y: f32 = map_y(y[0], min, max);
    for k in 1..y.len() {
        let px: f32 = 50. + k as f32 / y.len() as f32 * 700.;
        let py: f32 = map_y(y[k], min, max);
        draw_line(prev_x, prev_y, px, py, 1., color);
        prev_x = px;
        prev_y = py;
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let samples: usize = 100;
    let time: f32 = 2.;
    let x: Vec<Complex> = (0..samples).map(
        |x| Complex::new(
            x as f32 / samples as f32 * time,
            f32::cos(2. * PI * (x as f32 / samples as f32 * time) * 3.) +
            f32::sin(2. * PI * (x as f32 / samples as f32 * time) * 9.)
        )
    ).collect();

    let xout: Vec<Complex> = fft::dft(x.as_slice());
    let y: Vec<f32> = x.iter().map(|x| x.im).collect();
    let yout: Vec<f32> = xout.iter().map(|x| f32::sqrt(x.re * x.re + x.im * x.im)).collect();

    let freqs: Vec<f32> = fft::frequencies(xout.as_slice(), samples, time);
    for freq in freqs {
        println!("Detected {} Hz", freq);
    }

    loop {
        clear_background(BLACK);

        plot(&y, GRAY);
        plot(&yout, WHITE);

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("DFT example"),
        window_resizable: false,
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

