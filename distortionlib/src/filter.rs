pub struct Filter {
    fs: f32,
    wc: f32,
    q: f32,
    inputs: [f32; 4],
    outputs: [f32; 4],
    fb: f32,
}

// A lowpass filter based loosely on Tim Stilson and Julius Smith's
// Moog VCF model (https://ccrma.stanford.edu/~stilti/papers/moogvcf.pdf)
impl Filter {
    pub fn new(sr: usize) -> Self {
        Filter {
            fs: sr as f32,
            wc: 1000.0,
            q: 0.707,
            inputs: [0.0; 4],
            outputs: [0.0; 4],
            fb: 0.0,
        }
    }

    #[inline(always)]
    fn process_pole(x0: f32, x1: &mut f32, y1: &mut f32, wc: f32) {
        *y1 = x0 + 0.3 * *x1 + (1.0 - wc) * *y1;
    }

    #[inline(always)]
    fn process_sample(&mut self, x: f32) -> f32 {
        const MAKEUP: f32 = 5.0 / std::f32::consts::SQRT_2;

        let mut x = x - self.outputs[3] * self.fb;
        x *= 0.35013 * self.wc.powi(4);

        Filter::process_pole(x, &mut self.inputs[0], &mut self.outputs[0], self.wc);
        Filter::process_pole(self.outputs[0], &mut self.inputs[1], &mut self.outputs[1], self.wc);
        Filter::process_pole(self.outputs[1], &mut self.inputs[2], &mut self.outputs[2], self.wc);
        Filter::process_pole(self.outputs[2], &mut self.inputs[3], &mut self.outputs[3], self.wc);

        self.outputs[3] * MAKEUP
    }

    pub fn process_block(&mut self, block: &mut [f32]) {
        self.fb = self.q * (1.0 - 0.15 * self.wc * self.wc);

        for x in block {
            *x = self.process_sample(*x);
        }
    }

    pub fn set_freq(&mut self, freq_hz: f32) {
        let freq_hz = if freq_hz < (self.fs * 0.45) { freq_hz } else { self.fs * 0.45 };
        self.wc = 1.16 * freq_hz / (self.fs / 2.0);
    }

    pub fn set_resonance(&mut self, q: f32) {
        self.q = q;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use crate::utils;

    fn create_sine(freq: f32, num_samples: usize, fs: f32) -> Vec<f32> {
        // TODO: use a generator instead??

        let mut sine: Vec<f32> = vec![0.0; num_samples];
        for n in 0..num_samples {
            sine[n] = (2.0 * std::f32::consts::PI * freq * (n as f32) / fs).sin();
        }

        sine
    }

    fn get_rms_level(signal: &[f32]) -> f32 {
        let square_sum: f32 = signal.iter().map(|&x: &f32| x * x).sum();

        (square_sum / signal.len() as f32).sqrt()
    }

    #[test]
    fn test_filter_apply() {
        const FS: f32 = 44100.0;
        const N: usize = 8192;

        let mut filter = Filter::new(FS as usize);
        filter.set_freq(1000.0);
        filter.set_resonance(0.7071);

        let mut sine_10 = create_sine(10.0, N, FS);
        let mut sine_2k = create_sine(2000.0, N, FS);

        filter.process_block(&mut sine_10);
        filter.process_block(&mut sine_2k);

        let rms_10_db = utils::gain_2_db(get_rms_level(&sine_10));
        let rms_2k_db = utils::gain_2_db(get_rms_level(&sine_2k));

        const TOL: f32 = 1.0;
        assert_approx_eq!(rms_10_db, -3.0, TOL);
        assert_approx_eq!(rms_2k_db, -24.0, TOL);
    }
}
