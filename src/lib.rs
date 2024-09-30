use std::{
    env,
    path::{Path, PathBuf},
};

pub fn add_files(build: &mut cc::Build, root: &Path, files: &[&str]) {
    build.files(files.iter().map(|src| {
        let mut p = root.join(src);
        p.set_extension("c");
        p
    }));

    build.include(root);
}

#[derive(Debug, Clone)]
pub struct Build {
    build_debug: bool,
}

impl Build {
    pub fn new() -> Self {
        Self { build_debug: false }
    }
}

impl Default for Build {
    fn default() -> Self {
        Self::new()
    }
}

impl Build {
    pub fn enable_debug(&mut self) {
        self.build_debug = true;
    }
    pub fn build(&self) {
        let vendor = Path::new(env!("CARGO_MANIFEST_DIR")).join("vendor");
        let vendor_src = vendor.join("src");
        let mut build = cc::Build::new();
        build.include(&vendor).include(vendor.join("include"));

        add_files(&mut build, &vendor_src, &["libliquid"]);

        add_files(
            &mut build,
            &vendor_src.join("agc").join("src"),
            &["agc_crcf", "agc_rrrf"],
        );

        add_files(&mut build, &vendor_src.join("audio").join("src"), &["cvsd"]);

        add_files(
            &mut build,
            &vendor_src.join("buffer").join("src"),
            &["buffercf", "bufferf"],
        );

        add_files(
            &mut build,
            &vendor_src.join("channel").join("src"),
            &["channel_cccf"],
        );

        add_files(
            &mut build,
            &vendor_src.join("dotprod").join("src"),
            &["dotprod_cccf", "dotprod_crcf", "dotprod_rrrf", "sumsq"],
        );

        add_files(
            &mut build,
            &vendor_src.join("equalization").join("src"),
            &["equalizer_cccf", "equalizer_rrrf"],
        );

        add_files(
            &mut build,
            &vendor_src.join("fec").join("src"),
            &[
                "c_ones_mod2",
                "crc",
                "fec_conv_pmatrix",
                "fec_conv_poly",
                "fec_conv_punctured",
                "fec_conv",
                "fec_golay2412",
                "fec_hamming74",
                "fec_hamming84",
                "fec_hamming128_gentab",
                "fec_hamming128",
                "fec_hamming1511",
                "fec_hamming3126",
                "fec_pass",
                "fec_rep3",
                "fec_rep5",
                "fec_rs",
                "fec_secded2216",
                "fec_secded3932",
                "fec_secded7264",
                "fec",
                "interleaver",
                "packetizer",
                "sumproduct",
            ],
        );

        add_files(
            &mut build,
            &vendor_src.join("fft").join("src"),
            &["fft_utilities", "fftf", "spgramcf", "spgramf"],
        );

        add_files(
            &mut build,
            &vendor_src.join("filter").join("src"),
            &[
                "bessel",
                "butter",
                "cheby1",
                "cheby2",
                "ellip",
                "filter_cccf",
                "filter_crcf",
                "filter_rrrf",
                "firdes",
                "firdespm_halfband",
                "firdespm",
                "fnyquist",
                "gmsk",
                "group_delay",
                "hM3",
                "iirdes",
                "lpc",
                "rcos",
                "rkaiser",
                "rrcos",
            ],
        );

        add_files(
            &mut build,
            &vendor_src.join("framing").join("src"),
            &[
                "bpacketgen",
                "bpacketsync",
                "detector_cccf",
                "dsssframe64gen",
                "dsssframe64sync",
                "dsssframegen",
                "dsssframesync",
                "flexframegen",
                "flexframesync",
                "framedatastats",
                "framegen64",
                "framesync64",
                "framesyncstats",
                "framing_cccf",
                "framingcf",
                "framing_crcf",
                "framing_rrrf",
                "fskframegen",
                "fskframesync",
                "gmskframegen",
                "gmskframesync",
                "ofdmflexframegen",
                "ofdmflexframesync",
                "qpilotgen",
                "qpilotsync",
            ],
        );

        add_files(
            &mut build,
            &vendor_src.join("math").join("src"),
            &[
                "math",
                "modular_arithmetic",
                "poly",
                "polyc",
                "polycf",
                "polyf",
                "windows",
            ],
        );

        add_files(
            &mut build,
            &vendor_src.join("matrix").join("src"),
            &[
                "matrix", "matrixc", "matrixcf", "matrixf", "smatrixb", "smatrixf", "smatrixi",
            ],
        );

        add_files(
            &mut build,
            &vendor_src.join("modem").join("src"),
            &[
                "ampmodem",
                "fskdem",
                "fskmod",
                "gmskdem",
                "gmskmod",
                "modem_apsk_const",
                "modem_arb_const",
                "modemcf",
                "modem_utilities",
            ],
        );

        add_files(
            &mut build,
            &vendor_src.join("multichannel").join("src"),
            &[
                "firpfbch_cccf",
                "firpfbch_crcf",
                "ofdmframegen",
                "ofdmframesync",
            ],
        );

        add_files(
            &mut build,
            &vendor_src.join("nco").join("src"),
            &["nco_crcf"],
        );

        build.include(&vendor_src.join("optim").join("src"));
        add_files(
            &mut build,
            &vendor_src.join("optim").join("src"),
            &[
                "chromosome",
                "gasearch",
                "gradsearch",
                "qnsearch",
                "qs1dsearch",
                "utilities",
            ],
        );

        add_files(
            &mut build,
            &vendor_src.join("quantization").join("src"),
            &["compand", "quantizercf", "quantizerf"],
        );

        add_files(
            &mut build,
            &vendor_src.join("random").join("src"),
            &[
                "rand",
                "randexp",
                "randgamma",
                "randn",
                "randnakm",
                "randricek",
                "randweib",
                "scramble",
            ],
        );

        add_files(
            &mut build,
            &vendor_src.join("sequence").join("src"),
            &["bsequence", "msequence"],
        );

        add_files(
            &mut build,
            &vendor_src.join("utility").join("src"),
            &[
                "bshift_array",
                "byte_utilities",
                "memory",
                "msb_index",
                "pack_bytes",
                "shift_array",
                "utility",
            ],
        );

        // add_files(
        //     &mut build,
        //     &vendor_src.join("vector").join("src"),
        //     &["audio_arsp"],
        // );

        // let target = std::env::var("TARGET").unwrap();

        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        let lib_dir = out_dir.join("lib");

        build.out_dir(&lib_dir);
        build.compile("liquid");
    }
}
