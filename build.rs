use std::path::PathBuf;
use std::{env, fs};

fn main() {
    // Nuspell:
    let mut config = cc::Build::new();

    let manifest_path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let src_path = manifest_path.join("vendor/nuspell/src");
    for entry in fs::read_dir(&src_path).unwrap() {
        let entry = entry.unwrap();
        let path = src_path.join(entry.file_name());
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
    }

    config
        .cpp(true)
        .include(&src_path)
        .file(src_path.join("aff_data.cxx"))
        .file(src_path.join("checker.cxx"))
        .file(src_path.join("dictionary.cxx"))
        .file(src_path.join("finder.cxx"))
        .file(src_path.join("suggester.cxx"))
        .file(src_path.join("utils.cxx"))
        // My stuff
        .file(src_path.join("interface.cxx"))
        .compile("nuspell");

    println!("cargo:rustc-link-lib=static=nuspell");
    // Link to libc++
    println!("cargo:rustc-link-lib=stdc++");
    // Link to ICU4C, specifically icu-uc which Nuspell mentions in
    // `nuspell.pc.in`.
    println!("cargo:rustc-link-lib=icuuc");

    // Hunspell:
    let mut config = cc::Build::new();

    let manifest_path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let src_path = manifest_path.join("vendor/hunspell/src");
    for entry in fs::read_dir(&src_path).unwrap() {
        let entry = entry.unwrap();
        let path = src_path.join(entry.file_name());
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
    }

    config
        .cpp(true)
        .include(&src_path)
        .file(src_path.join("affentry.cxx"))
        .file(src_path.join("affixmgr.cxx"))
        .file(src_path.join("csutil.cxx"))
        .file(src_path.join("filemgr.cxx"))
        .file(src_path.join("hashmgr.cxx"))
        .file(src_path.join("hunspell.cxx"))
        .file(src_path.join("hunzip.cxx"))
        .file(src_path.join("phonet.cxx"))
        .file(src_path.join("replist.cxx"))
        .file(src_path.join("suggestmgr.cxx"))
        .compile("hunspell");

    println!("cargo:rustc-link-lib=static=hunspell");
}
