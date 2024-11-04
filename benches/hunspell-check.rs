#![feature(test)]

extern crate test;

use std::path::PathBuf;

use ffi_dictionaries::Hunspell;
use once_cell::sync::Lazy;
use test::{black_box, Bencher};

static EN_US: Lazy<Hunspell> = Lazy::new(|| {
    let manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let aff_path = manifest_path.join("vendor/en_US/en_US.aff");
    let dic_path = manifest_path.join("vendor/en_US/en_US.dic");
    Hunspell::new(aff_path.as_os_str(), dic_path.as_os_str())
});

#[bench]
fn in_dictionary_word(b: &mut Bencher) {
    b.iter(|| EN_US.spell(black_box(c"earth")))
}

#[bench]
fn number(b: &mut Bencher) {
    b.iter(|| EN_US.spell(black_box(c"8675,309.0")))
}

#[bench]
fn word_with_suffix(b: &mut Bencher) {
    b.iter(|| EN_US.spell(black_box(c"earthly")))
}

#[bench]
fn word_with_prefix(b: &mut Bencher) {
    b.iter(|| EN_US.spell(black_box(c"unearth")))
}

#[bench]
fn word_with_prefix_and_suffix(b: &mut Bencher) {
    b.iter(|| EN_US.spell(black_box(c"unearthed")))
}

#[bench]
fn incorrect_prefix(b: &mut Bencher) {
    b.iter(|| EN_US.spell(black_box(c"reearth")))
}

#[bench]
fn uppercase_in_dictionary_word(b: &mut Bencher) {
    b.iter(|| EN_US.spell(black_box(c"EARTH")))
}

#[bench]
fn titlecase_in_dictionary_word(b: &mut Bencher) {
    b.iter(|| EN_US.spell(black_box(c"Earth")))
}

#[bench]
fn breaks(b: &mut Bencher) {
    b.iter(|| EN_US.spell(black_box(c"light-weight-like")))
}

#[bench]
fn compound_word(b: &mut Bencher) {
    b.iter(|| EN_US.spell(black_box(c"20000th")))
}
