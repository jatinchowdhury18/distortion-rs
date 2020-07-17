#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

namespace distortion {

struct Distortion;

extern "C" {

Distortion *create(uintptr_t sample_rate);

void destroy(Distortion *distortion);

void process(Distortion *distortion, float *block, uintptr_t num_samples);

void set_drive(Distortion *distortion, float drive);

void set_filter_params(Distortion *distortion, float freq_hz, float q);

void set_gain(Distortion *distortion, float gain_db);

} // extern "C"

} // namespace distortion
