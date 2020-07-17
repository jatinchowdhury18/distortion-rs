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
    const SQRT2: f32 = 1.414427157;

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
        let mut x = x - self.outputs[3] * self.fb;
        x *= 0.35013 * self.wc.powi(4);

        Filter::process_pole(x, &mut self.inputs[0], &mut self.outputs[0], self.wc);
        Filter::process_pole(self.outputs[0], &mut self.inputs[1], &mut self.outputs[1], self.wc);
        Filter::process_pole(self.outputs[1], &mut self.inputs[2], &mut self.outputs[2], self.wc);
        Filter::process_pole(self.outputs[2], &mut self.inputs[3], &mut self.outputs[3], self.wc);

        self.outputs[3] * Filter::SQRT2
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

// @TODO write tests...
