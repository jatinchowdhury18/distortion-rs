# Distortion-rs

[![Build Status](https://travis-ci.com/jatinchowdhury18/distortion-rs.svg?token=Ub9niJrqG1Br1qaaxp7E&branch=master)](https://travis-ci.com/jatinchowdhury18/distortion-rs)

This repository contains an example audio plugin built using JUCE
and Rust.

## Building
To build the plugin, you must have Rust abd CMake installed. You
also need [`cbindgen`](https://github.com/eqrion/cbindgen), which
can be installed with the command:
```bash
$ cargo install --force cbindgen
```

Next, clone the repository and initialize submodules:
```bash
$ git clone https://github.com/jatinchowdhury18/distortion-rs.git
$ cd distortion-rs
$ git submodule update --init --recursive
```

Finally, build the project using CMake:
```
$ cmake -Bbuild
$ cmake --build build --config Release
```

## About
I was inspired by Ian Hobson's
[freeverb-rs](https://github.com/irh/freeverb-rs) project to start
learning Rust for audio signal processing. The wonderful
[Rust audio](https://github.com/RustAudio) community has developed
the [vst-rs](https://github.com/RustAudio/vst-rs) crate to build
audio plugins entirely in Rust, but unfortunately plugin GUI support
in Rust is limited. 

The `distortionlib` folder is a Rust crate which builds to a static
library containing all necessary DSP functionality. The `src/` folder
contains JUCE/C++ code that builds the plugin GUI, manages the plugin
parameters, and connects to the Rust library. Since JUCE6 supports
building with CMake, this project uses Devolution's
[CMakeRust](https://github.com/Devolutions/CMakeRust) tool to integrate
the Rust toolchain into the CMake build process.

## License
@TODO
