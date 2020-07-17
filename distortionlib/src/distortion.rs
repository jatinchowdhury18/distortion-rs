use super::gain::Gain;
use super::waveshape::WaveShape;
use super::filter::Filter;

pub struct Distortion {
    gain_proc: Gain,
    filter_proc: Filter,
    dist_proc: WaveShape,
}

impl Distortion {
    pub fn new(sr: usize) -> Self {
        let mut dist = Distortion {
            gain_proc: Gain::new(),
            filter_proc: Filter::new(sr),
            dist_proc: WaveShape::new(),
        };

        dist.gain_proc.reset();

        dist
    }

    pub fn process_block(&mut self, block: &mut [f32]) {
        self.gain_proc.process_block(block);
        self.filter_proc.process_block(block);
        self.dist_proc.process_block(block);
    }

    // parameter setters
    pub fn set_gain_db(&mut self, gain_db: f32) {
        self.gain_proc.set_gain_db(gain_db);
    }

    pub fn set_drive(&mut self, drive: f32) {
        self.dist_proc.set_drive(drive);
    }

    pub fn set_filter_params(&mut self, freq_hz: f32, q: f32) {
        self.filter_proc.set_freq(freq_hz);
        self.filter_proc.set_resonance(q);
    }
}
