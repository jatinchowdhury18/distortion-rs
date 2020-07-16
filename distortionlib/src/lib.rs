mod distortion;
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
    input: *const f32,
    output: *mut f32,
    num_samples: usize
) {
    for i in 0..num_samples as isize {
        let x = distortion.process_sample(*input.offset (i));
        *output.offset(i) = x;
    }
}
