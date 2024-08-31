use std::{hint::black_box, path::PathBuf};

use brunch::Bench;
use ffi_dictionaries::{Hunspell, Nuspell};
use once_cell::sync::OnceCell;

const SAMPLES: u32 = 500_000;

fn en_us_nuspell() -> &'static Nuspell {
    static EN_US: OnceCell<Nuspell> = OnceCell::new();
    EN_US.get_or_init(|| {
        let manifest_path = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
        let aff_path = manifest_path.join("vendor/en_US/en_US.aff");

        Nuspell::new(aff_path.as_os_str())
    })
}

fn en_us_hunspell() -> &'static Hunspell {
    static EN_US: OnceCell<Hunspell> = OnceCell::new();
    EN_US.get_or_init(|| {
        let manifest_path = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
        let aff_path = manifest_path.join("vendor/en_US/en_US.aff");
        let dic_path = manifest_path.join("vendor/en_US/en_US.dic");

        Hunspell::new(aff_path.as_os_str(), dic_path.as_os_str())
    })
}

brunch::benches!(
    // Compilation
    Bench::new("Nuspell: compile en_US").run(|| {
        let manifest_path = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
        let aff_path = manifest_path.join("vendor/en_US/en_US.aff");

        Nuspell::new(aff_path.as_os_str())
    }),
    Bench::new("Hunspell: compile en_US").run(|| {
        let manifest_path = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
        let aff_path = manifest_path.join("vendor/en_US/en_US.aff");
        let dic_path = manifest_path.join("vendor/en_US/en_US.dic");

        Hunspell::new(aff_path.as_os_str(), dic_path.as_os_str())
    }),
    Bench::spacer(),
    // Nuspell
    Bench::new("Nuspell: In-dictionary word (\"earth\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us_nuspell, |dict| dict.spell(black_box("earth"))),
    Bench::new("Nuspell: Number (\"8675,309.0\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us_nuspell, |dict| dict.spell(black_box("8675,309.0"))),
    Bench::new("Nuspell: Word with suffix (\"earthly\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us_nuspell, |dict| dict.spell(black_box("earthly"))),
    Bench::new("Nuspell: Word with prefix (\"unearth\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us_nuspell, |dict| dict.spell(black_box("unearth"))),
    Bench::new("Nuspell: Word with prefix and suffix (\"unearthed\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us_nuspell, |dict| dict.spell(black_box("unearthed"))),
    Bench::new("Nuspell: Incorrect prefix (\"reearth\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us_nuspell, |dict| dict.spell(black_box("reearth"))),
    Bench::new("Nuspell: UPPERCASE in-dictionary word (\"EARTH\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us_nuspell, |dict| dict.spell(black_box("EARTH"))),
    Bench::new("Nuspell: Titlecase in-dictionary word (\"Earth\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us_nuspell, |dict| dict.spell(black_box("Earth"))),
    Bench::new("Nuspell: Breaks (\"light-weight-like\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us_nuspell, |dict| dict
            .spell(black_box("light-weight-like"))),
    Bench::new("Nuspell: Compound word (\"20000th\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us_nuspell, |dict| dict.spell(black_box("20000th"))),
    Bench::spacer(),
    Bench::new("Hunspell: In-dictionary word (\"earth\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us_hunspell, |dict| dict.spell(black_box("earth"))),
    Bench::new("Hunspell: Number (\"8675,309.0\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us_hunspell, |dict| dict.spell(black_box("8675,309.0"))),
    Bench::new("Hunspell: Word with suffix (\"earthly\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us_hunspell, |dict| dict.spell(black_box("earthly"))),
    Bench::new("Hunspell: Word with prefix (\"unearth\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us_hunspell, |dict| dict.spell(black_box("unearth"))),
    Bench::new("Hunspell: Word with prefix and suffix (\"unearthed\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us_hunspell, |dict| dict.spell(black_box("unearthed"))),
    Bench::new("Hunspell: Incorrect prefix (\"reearth\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us_hunspell, |dict| dict.spell(black_box("reearth"))),
    Bench::new("Hunspell: UPPERCASE in-dictionary word (\"EARTH\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us_hunspell, |dict| dict.spell(black_box("EARTH"))),
    Bench::new("Hunspell: Titlecase in-dictionary word (\"Earth\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us_hunspell, |dict| dict.spell(black_box("Earth"))),
    Bench::new("Hunspell: Breaks (\"light-weight-like\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us_hunspell, |dict| dict
            .spell(black_box("light-weight-like"))),
    Bench::new("Hunspell: Compound word (\"20000th\")")
        .with_samples(SAMPLES)
        .run_seeded_with(en_us_hunspell, |dict| dict.spell(black_box("20000th"))),
);
