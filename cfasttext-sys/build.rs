use std::{fs, str};

fn fail_on_empty_directory(name: &str) {
    if fs::read_dir(name).unwrap().count() == 0 {
        println!(
            "The `{}` directory is empty, did you forget to pull the submodules?",
            name
        );
        println!("Try `git submodule update --init --recursive`");
        panic!();
    }
}

fn build_cfasttext() {
    let mut build = cc::Build::new();
    let compiler = build.get_compiler();
    if compiler.is_like_msvc() {
        // Enable exception for clang-cl
        // https://learn.microsoft.com/en-us/cpp/build/reference/eh-exception-handling-model?redirectedfrom=MSDN&view=msvc-170
        build.flag("/EHsc");
    }
    build
        .cpp(true)
        .files([
            "cfasttext/fasterText/src/args.cc",
            "cfasttext/fasterText/src/autotune.cc",
            "cfasttext/fasterText/src/matrix.cc",
            "cfasttext/fasterText/src/dictionary.cc",
            "cfasttext/fasterText/src/loss.cc",
            "cfasttext/fasterText/src/productquantizer.cc",
            "cfasttext/fasterText/src/densematrix.cc",
            "cfasttext/fasterText/src/quantmatrix.cc",
            "cfasttext/fasterText/src/vector.cc",
            "cfasttext/fasterText/src/model.cc",
            "cfasttext/fasterText/src/utils.cc",
            "cfasttext/fasterText/src/meter.cc",
            "cfasttext/fasterText/src/fasttext.cc",
            "cfasttext/lib/cfasttext.cc",
        ])
        .includes(["cfasttext/fasterText/src", "cfasttext/include"])
        .flag("-std=c++11")
        .flag_if_supported("-pthread")
        .flag_if_supported("-funroll-loops")
        .compile("cfasttext_static");
}

fn main() {
    fail_on_empty_directory("cfasttext");
    fail_on_empty_directory("cfasttext/fasterText");
    build_cfasttext();
}
