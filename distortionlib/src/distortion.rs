pub struct Distortion {
    gain: f32,
}

impl Distortion {
    pub fn new(sr: usize) -> Self {
        let dist = Distortion {
            gain: 1.0,
        };

        dist
    }

    pub fn process_sample(&mut self, x: f32) -> f32 {
        x * self.gain
    }
}
