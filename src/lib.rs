use std::ffi::{c_char, c_int, CString, OsStr};

enum NuspellDictionary {}

#[link(name = "nuspell")]
extern "C" {
    fn Dictionary_create(aff_path: *const c_char) -> *mut NuspellDictionary;

    fn Dictionary_destroy(dict: *mut NuspellDictionary);

    fn Dictionary_spell(dict: *const NuspellDictionary, word: *const c_char) -> c_int;

}

pub struct Dictionary {
    inner: *mut NuspellDictionary,
}

impl Dictionary {
    pub fn new(affpath: &OsStr) -> Self {
        let affpath = CString::new(affpath.as_encoded_bytes()).unwrap();
        unsafe {
            Self {
                inner: Dictionary_create(affpath.as_ptr()),
            }
        }
    }

    pub fn spell(&self, word: &str) -> bool {
        let word = CString::new(word).unwrap();
        unsafe { Dictionary_spell(self.inner.cast_const(), word.as_ptr()) != 0 }
    }
}

impl Drop for Dictionary {
    fn drop(&mut self) {
        unsafe { Dictionary_destroy(self.inner) };
    }
}

unsafe impl Send for Dictionary {}
unsafe impl Sync for Dictionary {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn kick_the_tires() {
        let manifest_path = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
        let aff_path = manifest_path.join("vendor/en_US/en_US.aff");
        let dict = Dictionary::new(aff_path.as_os_str());

        assert!(dict.spell("hello"));
        assert!(dict.spell("world"));

        assert!(!dict.spell("exmaple"));
    }
}
