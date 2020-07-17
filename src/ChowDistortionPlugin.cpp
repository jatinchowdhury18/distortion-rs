#include "ChowDistortionPlugin.h"

namespace
{
    const String gainTag = "gain";
    const String freqTag = "freq";
    const String qTag = "q";
    const String driveTag = "drive";
}

ChowDistortion::ChowDistortion()
{
    gainDBParam = vts.getRawParameterValue (gainTag);
    freqHzParam = vts.getRawParameterValue (freqTag);
    qParam = vts.getRawParameterValue (qTag);
    driveParam = vts.getRawParameterValue (driveTag);
}

void ChowDistortion::addParameters (Parameters& params)
{
    NormalisableRange<float> freqRange (20.0f, 20000.0f);
    freqRange.setSkewForCentre (1000.0f);

    NormalisableRange<float> qRange (0.1f, 18.0f);
    qRange.setSkewForCentre (0.707f);

    params.push_back (std::make_unique<AudioParameterFloat> (gainTag, "Gain [dB]", -30.0f, 30.0f, 0.0f));
    params.push_back (std::make_unique<AudioParameterFloat> (freqTag, "Freq", freqRange, 1000.0f));
    params.push_back (std::make_unique<AudioParameterFloat> (qTag, "Q", qRange, 0.7071f));
    params.push_back (std::make_unique<AudioParameterFloat> (driveTag, "Drive", 0.0f, 1.0f, 0.5f));
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
    {
        distortion::set_gain (distProc[ch].get(), *gainDBParam);
        distortion::set_filter_params (distProc[ch].get(), *freqHzParam, *qParam);
        distortion::set_drive (distProc[ch].get(), *driveParam);

        distortion::process (distProc[ch].get(), buffer.getWritePointer (ch), buffer.getNumSamples());
    }
}

// This creates new instances of the plugin...
AudioProcessor* JUCE_CALLTYPE createPluginFilter()
{
    return new ChowDistortion();
}
