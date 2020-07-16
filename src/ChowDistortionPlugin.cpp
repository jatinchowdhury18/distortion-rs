#include "ChowDistortionPlugin.h"

ChowDistortion::ChowDistortion()
{
}

void ChowDistortion::addParameters (Parameters& params)
{
}

void ChowDistortion::prepareToPlay (double sampleRate, int samplesPerBlock)
{
    for (int ch = 0; ch < 2; ++ch)
        distProc[ch].reset (distortion::create (sampleRate));
}

void ChowDistortion::releaseResources()
{
}

void ChowDistortion::processBlock (AudioBuffer<float>& buffer)
{
    ScopedNoDenormals noDenormals;

    for (int ch = 0; ch < buffer.getNumChannels(); ++ch)
        distortion::process (distProc[ch].get(), buffer.getReadPointer (ch),
            buffer.getWritePointer (ch), buffer.getNumSamples());
}

// This creates new instances of the plugin...
AudioProcessor* JUCE_CALLTYPE createPluginFilter()
{
    return new ChowDistortion();
}
