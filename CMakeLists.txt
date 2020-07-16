cmake_minimum_required(VERSION 3.15)
project(distortion-rs VERSION 1.0.0)

set(CMAKE_CXX_STANDARD 17)
add_subdirectory(modules)

juce_add_plugin(ChowDistortionRust
    COMPANY_NAME chowdsp
    PLUGIN_MANUFACTURER_CODE Chow
    PLUGIN_CODE Cdrs
    FORMATS AU VST3 Standalone #VST
    ProductName "distortion-rs")

set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${CMAKE_SOURCE_DIR}/cmake/")

enable_language(Rust)
include(CMakeCargo)

add_subdirectory(distortionlib)
include_directories(distortionlib)

add_custom_command(TARGET ChowDistortionRust PRE_BUILD
    COMMAND cbindgen -d --lang c++ -o distortion.h .
    WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}/distortionlib/"
    COMMENT "Generating Rust to C++ bindings..."
)

juce_generate_juce_header(ChowDistortionRust)
add_subdirectory(src)

target_compile_definitions(ChowDistortionRust
    PUBLIC
    JUCE_DISPLAY_SPLASH_SCREEN=0
    JUCE_REPORT_APP_USAGE=0
    JUCE_WEB_BROWSER=0
    JUCE_USE_CURL=0
    JUCE_VST3_CAN_REPLACE_VST2=0
    FOLEYS_SHOW_GUI_EDITOR_PALLETTE=0
    # FOLEYS_ENABLE_BINARY_DATA=1
    )

target_link_libraries(ChowDistortionRust PRIVATE
    # BinaryData
    juce::juce_audio_utils
    foleys_gui_magic
    distortionlib
    )

if(WIN32)
    target_link_libraries(ChowDistortionRust PRIVATE userenv)
endif()
