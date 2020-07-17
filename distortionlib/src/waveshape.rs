use super::utils;

pub struct WaveShape {
    amount: f32,
    skew: f32,
}

impl WaveShape {
    const MIN_EXP: f32 = 0.4;
    const MAX_EXP: f32 = 5.0;

    pub fn new() -> Self {
        WaveShape {
            amount: 2.5,
            skew: utils::get_skew_for_centre(WaveShape::MIN_EXP, WaveShape::MAX_EXP, 2.5),
        }
    }

    pub fn set_drive(&mut self, drive: f32) {
        self.amount = utils::jmap01((1.0 - drive).powf(self.skew), WaveShape::MIN_EXP, WaveShape::MAX_EXP);
    }

    // saturating waveshaper, adapted from D. Yeh thesis
    // (https://ccrma.stanford.edu/~dtyeh/papers/DavidYehThesissinglesided.pdf),
    // see "tanh approx" on page 11
    #[inline(always)]
    fn process_sample(x: f32, p: f32) -> f32 {
        x / (1.0 + x.abs().powf(p)).powf(1.0 / p)
    }

    pub fn process_block(&self, block: &mut [f32]) {
        // @TODO: SIMD accelerate??
        for x in block {
            *x = WaveShape::process_sample(*x, self.amount);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_gain_apply() {
        let mut ws = WaveShape::new();
        ws.set_drive(0.5);

        let mut vec = vec![-500.0, 0.0, 500.0];
        ws.process_block(&mut vec);

        assert_approx_eq!(vec[0], -1.0);
        assert_approx_eq!(vec[1], 0.0);
        assert_approx_eq!(vec[2], 1.0);
    }
}
