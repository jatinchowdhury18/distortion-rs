use super::utils;

pub struct Gain {
    cur_gain: f32,
    old_gain: f32,
}

impl Gain {
    pub fn new() -> Self {
        Gain {
            cur_gain: 1.0,
            old_gain: 1.0,
        }
    }

    pub fn reset(&mut self) {
        self.old_gain = self.cur_gain;
    }

    pub fn set_gain_db(&mut self, new_gain_db: f32) {
        self.cur_gain = utils::db_2_gain(new_gain_db);
    }

    pub fn process_block(&mut self, block: &mut [f32]) {
        // @TODO: speed up with SIMD ??

        if self.cur_gain == self.old_gain {
            for samp in block {
                *samp *= self.cur_gain;
            }
        } else {
            let num_samples = block.len() as f32;
            for (n, samp) in block.iter_mut().enumerate() {
                let factor = n as f32 /  num_samples;
                *samp *= self.cur_gain * factor + self.old_gain * (1.0 - factor);
            }

            self.old_gain = self.cur_gain;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_gain_apply() {
        let mut gain = Gain::new();
        gain.set_gain_db(6.0);
        gain.reset();

        let mut ones_vec = vec![1.0; 10];
        gain.process_block(&mut ones_vec);

        assert_approx_eq!(2.0, ones_vec[1], 0.01);
    }
}
