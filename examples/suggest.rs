use std::ffi::{CStr, CString};
use std::path::PathBuf;
use std::time::Instant;

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

    fn suggest(&self, word: &CStr, out: &mut Vec<String>) {
        match self {
            Self::Nuspell(dict) => dict.suggest(word, out),
            Self::Hunspell(dict) => dict.suggest(word, out),
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

    let mut suggestions = Vec::with_capacity(5);
    let now = Instant::now();
    dict.suggest(&word, &mut suggestions);
    let time = now.elapsed();
    if suggestions.is_empty() {
        println!("No suggestions found for {word:?} (checked in {time:?})");
    } else {
        let suggestions = suggestions
            .into_iter()
            .fold(String::new(), |mut s, suggestion| {
                if !s.is_empty() {
                    s.push_str(", ");
                }
                s.push('"');
                s.push_str(&suggestion);
                s.push('"');
                s
            });
        println!("Suggestions for {word:?}: {suggestions} (checked in {time:?})");
    }
}
