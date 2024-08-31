use std::{hint::black_box, path::PathBuf};

use brunch::Bench;
use ffi_dictionaries::Nuspell;
use once_cell::sync::OnceCell;

const SAMPLES: u32 = 500_000;

fn en_us() -> &'static Nuspell {
    static EN_US: OnceCell<Nuspell> = OnceCell::new();
    EN_US.get_or_init(|| {
        let manifest_path = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
        let aff_path = manifest_path.join("vendor/en_US/en_US.aff");

        Nuspell::new(aff_path.as_os_str())
    })
}

brunch::benches!(
    // Compilation
    Bench::new("Compile en_US").run(|| {
        let manifest_path = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
        let aff_path = manifest_path.join("vendor/en_US/en_US.aff");

        Nuspell::new(aff_path.as_os_str())
    }),
    Bench::spacer(),
    // Checking
    Bench::new("In-dictionary word (\"earth\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us, |dict| dict.spell(black_box("earth"))),
    Bench::new("Number (\"8675,309.0\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us, |dict| dict.spell(black_box("8675,309.0"))),
    Bench::new("Word with suffix (\"earthly\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us, |dict| dict.spell(black_box("earthly"))),
    Bench::new("Word with prefix (\"unearth\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us, |dict| dict.spell(black_box("unearth"))),
    Bench::new("Word with prefix and suffix (\"unearthed\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us, |dict| dict.spell(black_box("unearthed"))),
    Bench::new("Incorrect prefix (\"reearth\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us, |dict| dict.spell(black_box("reearth"))),
    Bench::new("UPPERCASE in-dictionary word (\"EARTH\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us, |dict| dict.spell(black_box("EARTH"))),
    Bench::new("Titlecase in-dictionary word (\"Earth\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us, |dict| dict.spell(black_box("Earth"))),
    Bench::new("Breaks (\"light-weight-like\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us, |dict| dict.spell(black_box("light-weight-like"))),
    Bench::new("Compound word (\"20000th\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us, |dict| dict.spell(black_box("20000th"))),
);
