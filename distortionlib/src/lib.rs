mod distortion;
mod gain;
mod waveshape;
mod filter;
mod utils;

pub use distortion::Distortion;

#[no_mangle]
pub extern "C" fn create(sample_rate: usize) -> *mut Distortion {
    Box::into_raw(Box::new(Distortion::new(sample_rate)))
}

#[no_mangle]
pub unsafe extern "C" fn destroy(distortion: *mut Distortion) {
    assert!(!distortion.is_null());
    Box::from_raw(distortion);
}

#[no_mangle]
pub unsafe extern "C" fn process(
    distortion: &mut Distortion,
    block: *mut f32,
    num_samples: usize
) {
    let block = std::slice::from_raw_parts_mut(block, num_samples);
    distortion.process_block(block);
}

#[no_mangle]
pub extern "C" fn set_gain(distortion: &mut Distortion, gain_db: f32) {
    distortion.set_gain_db(gain_db);
}

#[no_mangle]
pub extern "C" fn set_drive(distortion: &mut Distortion, drive: f32) {
    distortion.set_drive(drive);
}

#[no_mangle]
pub extern "C" fn set_filter_params(distortion: &mut Distortion, freq_hz: f32, q: f32) {
    distortion.set_filter_params(freq_hz, q);
}
