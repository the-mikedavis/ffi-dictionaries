use std::ffi::CString;
use std::time::Instant;
use std::{ffi::CStr, path::PathBuf};

use ffi_dictionaries::{Hunspell, Nuspell};

enum Dictionary {
    Nuspell(Nuspell),
    Hunspell(Hunspell),
}

impl Dictionary {
    fn new(provider: &str) -> Self {
        match provider {
            "nuspell" => {
                let manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                let aff_path = manifest_path.join("vendor/en_US/en_US.aff");

                let now = Instant::now();
                let dict = Nuspell::new(aff_path.as_os_str());
                println!("Compiled the dictionary in {}ms", now.elapsed().as_millis());
                Self::Nuspell(dict)
            }
            "hunspell" => {
                let manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                let aff_path = manifest_path.join("vendor/en_US/en_US.aff");
                let dic_path = manifest_path.join("vendor/en_US/en_US.dic");

                let now = Instant::now();
                let dict = Hunspell::new(aff_path.as_os_str(), dic_path.as_os_str());
                println!("Compiled the dictionary in {}ms", now.elapsed().as_millis());
                Self::Hunspell(dict)
            }
            _ => unreachable!(),
        }
    }

    fn spell(&self, word: &CStr) -> bool {
        match self {
            Self::Nuspell(dict) => dict.spell(word),
            Self::Hunspell(dict) => dict.spell(word),
        }
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let provider = match args.next() {
        Some(arg) if arg == "hunspell" || arg == "nuspell" => arg,
        _ => {
            eprintln!("Usage: check {{nuspell|hunspell}} WORD");
            std::process::exit(1);
        }
    };
    let word = match args.next() {
        Some(word) => CString::new(word).unwrap(),
        None => {
            eprintln!("Usage: check {{nuspell|hunspell}} WORD");
            std::process::exit(1);
        }
    };

    let dict = Dictionary::new(&provider);

    let now = Instant::now();
    if dict.spell(&word) {
        println!(
            "{word:?} is in the dictionary (checked in {}µs)",
            now.elapsed().as_micros()
        );
    } else {
        eprintln!(
            "{word:?} is NOT in the dictionary (checked in {}µs)",
            now.elapsed().as_micros()
        );
        std::process::exit(1);
    }
}
