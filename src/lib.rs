use std::ffi::{c_char, c_int, CString, OsStr};

enum NuspellHandle {}

#[link(name = "nuspell")]
extern "C" {
    fn Nuspell_create(aff_path: *const c_char) -> *mut NuspellHandle;

    fn Nuspell_destroy(dict: *mut NuspellHandle);

    fn Nuspell_spell(dict: *const NuspellHandle, word: *const c_char) -> c_int;

}

pub struct Nuspell {
    inner: *mut NuspellDictionary,
}

impl Nuspell {
    pub fn new(affpath: &OsStr) -> Nuspell {
        let affpath = CString::new(affpath.as_encoded_bytes()).unwrap();
        unsafe {
            Nuspell {
                inner: Nuspell_create(affpath.as_ptr()),
            }
        }
    }

    pub fn spell(&self, word: &str) -> bool {
        let word = CString::new(word).unwrap();
        unsafe { Nuspell_spell(self.inner.cast_const(), word.as_ptr()) != 0 }
    }
}

impl Drop for Nuspell {
    fn drop(&mut self) {
        unsafe { Nuspell_destroy(self.inner) };
    }
}

unsafe impl Send for Nuspell {}
unsafe impl Sync for Nuspell {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn kick_the_tires() {
        let manifest_path = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
        let aff_path = manifest_path.join("vendor/en_US/en_US.aff");
        let dict = Nuspell::new(aff_path.as_os_str());

        assert!(dict.spell("hello"));
        assert!(dict.spell("world"));

        assert!(!dict.spell("exmaple"));
    }
}
