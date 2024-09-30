# `liquidizers-src`
> Source code and logic to build `libliquid` from source

[![](https://img.shields.io/crates/v/liquidizers-src.svg)][crates-io]
[![](https://docs.rs/liquidizers-src/badge.svg)][api-docs]
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)

This crate is intended to be consumed by a `sys` crate.

# Dependencies
* `gcc` or any other C compiler.

# Env Vars
* `DEP_LIQUID_DSP_INCLUDE` is path to the include directory.
* `DEP_LIQUID_DSP_LIB` is the path to the lib directory.
* `DEP_LIQUID_DSP_OUT` is the path to the out directory (root).

# Versioning
* The `master` branch uses the [`liquid-dsp`] `latest_release` branch and is
    considered a stable branch. When a stable release is published, the version
    will take the form of `VERSION+BUILD_METADATA`.

# License
* This project is licensed under the MIT license.

### Acknowledgments
* Based on [`zeromq-src-rs`]

[`zeromq-src-rs`]: https://github.com/jean-airoldie/zeromq-src-rs
[`liquid-dsp`]: https://github.com/jgaeddert/liquid-dsp
[crates-io]: https://crates.io/crates/liquidizers-src
[api-docs]: https://docs.rs/liquidizers-src