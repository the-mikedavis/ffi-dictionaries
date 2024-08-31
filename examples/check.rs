use std::path::PathBuf;
use std::time::Instant;

use ffi_dictionaries::Nuspell;

fn main() {
    let mut args = std::env::args().skip(1);
    let word = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Usage: check WORD");
            std::process::exit(1);
        }
    };

    let manifest_path = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let aff_path = manifest_path.join("vendor/en_US/en_US.aff");

    let now = Instant::now();
    let dict = Nuspell::new(aff_path.as_os_str());
    println!("Compiled the dictionary in {}ms", now.elapsed().as_millis());

    let now = Instant::now();
    if dict.spell(&word) {
        println!(
            "\"{word}\" is in the dictionary (checked in {}µs)",
            now.elapsed().as_micros()
        );
    } else {
        eprintln!(
            "\"{word}\" is NOT in the dictionary (checked in {}µs)",
            now.elapsed().as_micros()
        );
        std::process::exit(1);
    }
}
