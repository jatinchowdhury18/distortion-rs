// convert from dB to linear gain value
pub fn db_2_gain(gain_db: f32) -> f32 {
    if gain_db > -100.0 { 10.0f32.powf(gain_db * 0.05) } else { 0.0 }
}

// map from 0-1 range to given range
pub fn jmap01(value01: f32, min: f32, max: f32) -> f32 {
    min + value01 * (max - min)
}

// map value from source range to target range
pub fn jmap(value: f32, source_min: f32, source_max: f32, target_min: f32, target_max: f32) -> f32 {
    // assert!(source_max != source_min); // avoid NaNs!
    target_min + ((target_max - target_min) * (value - source_min)) / (source_max - source_min)
}

// get exponent skew to apply to 0-1 range to get desired centre when jmapped
pub fn get_skew_for_centre(min: f32, max: f32, centre: f32) -> f32 {
    let centre_norm = jmap(centre, min, max, 0.0, 1.0);
    centre_norm.log(0.5)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_db_2_gain() {
        let should_be_zero = db_2_gain(-100.0);
        let should_also_be_zero = db_2_gain(-200.0);
        let should_be_one = db_2_gain(0.0);
        let should_be_half = db_2_gain(-6.0);
        let should_be_two = db_2_gain(6.0);

        const TOL: f32 = 0.01;
        assert_approx_eq!(should_be_zero, 0.0, TOL);
        assert_approx_eq!(should_also_be_zero, 0.0, TOL);
        assert_approx_eq!(should_be_one, 1.0, TOL);
        assert_approx_eq!(should_be_half, 0.5, TOL);
        assert_approx_eq!(should_be_two, 2.0, TOL);
    }

    #[test]
    fn test_jmap01() {
        let should_be_half = jmap01(0.5, 0.0, 1.0);
        assert_eq!(should_be_half, 0.5);

        let should_be_fifty = jmap01(0.5, 0.0, 100.0);
        assert_eq!(should_be_fifty, 50.0);
    }

    #[test]
    fn test_jmap() {
        let should_be_half = jmap(0.5, 0.0, 1.0, 0.0, 1.0);
        assert_eq!(should_be_half, 0.5);

        let should_be_fifty = jmap(0.5, 0.0, 1.0, 0.0, 100.0);
        assert_eq!(should_be_fifty, 50.0);
    }

    #[test]
    fn test_get_skew_for_centre() {
        let should_be_one = get_skew_for_centre(0.0, 1.0, 0.5);
        assert_eq!(should_be_one, 1.0);

        let should_be_two = get_skew_for_centre(0.0, 2.0, 0.5);
        assert_eq!(should_be_two, 2.0);

        let should_be_half = get_skew_for_centre(0.0, 2.0, 2.0f32.sqrt());
        assert_approx_eq!(should_be_half, 0.5);
    }
}
