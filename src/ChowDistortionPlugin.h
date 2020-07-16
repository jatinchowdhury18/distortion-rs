#ifndef ChowDistortionPLUGIN_H_INCLUDED
#define ChowDistortionPLUGIN_H_INCLUDED

#include <JuceHeader.h>
#include <memory>

#include "PluginBase.h"
#include "distortion.h"

class ChowDistortion : public PluginBase<ChowDistortion>
{
public:
    ChowDistortion();

    static void addParameters (Parameters& params);
    void prepareToPlay (double sampleRate, int samplesPerBlock) override;
    void releaseResources() override;
    void processBlock (AudioBuffer<float>& buffer) override;

private:
    std::unique_ptr<distortion::Distortion, decltype(&distortion::destroy)> distProc[2] {{nullptr, &distortion::destroy}, {nullptr, &distortion::destroy}};

    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (ChowDistortion)
};

#endif // ChowDistortionPLUGIN_H_INCLUDED
