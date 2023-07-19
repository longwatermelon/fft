/// From frequency index to Hz
pub fn k_to_hz(k: usize, samples: usize, time: f32) -> f32 {
    k as f32 * (samples as f32 / time) / samples as f32
}

/// Returns indices of peaks in y
/// Doesn't work well on noisy data
pub fn peaks(y: &[f32]) -> Vec<usize> {
    let mut peaks: Vec<usize> = Vec::new();

    // TODO fix peak detection
    let mut avg: f32 = 0.;
    for i in 1..(y.len() - 1) {
        avg += y[i] / (y.len() - 2) as f32;

        if y[i] > y[i - 1] && y[i] > y[i + 1] && y[i] > avg * 5. {
            peaks.push(i);
        }
    }

    peaks
}

