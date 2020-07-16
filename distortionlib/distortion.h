#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

namespace distortion {

struct Distortion;

extern "C" {

Distortion *create(uintptr_t sample_rate);

void destroy(Distortion *distortion);

void process(Distortion *distortion, const float *input, float *output, uintptr_t num_samples);

} // extern "C"

} // namespace distortion
