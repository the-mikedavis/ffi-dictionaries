use std::ffi::{c_char, c_int, CString, OsStr};

enum NuspellHandle {}

#[link(name = "nuspell")]
extern "C" {
    fn Nuspell_create(aff_path: *const c_char) -> *mut NuspellHandle;

    fn Nuspell_destroy(dict: *mut NuspellHandle);

    fn Nuspell_spell(dict: *const NuspellHandle, word: *const c_char) -> c_int;

}

pub struct Nuspell {
    inner: *mut NuspellHandle,
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

enum HunspellHandle {}

#[link(name = "nuspell")]
extern "C" {
    fn Hunspell_create(aff_path: *const c_char, dic_path: *const c_char) -> *mut HunspellHandle;

    fn Hunspell_destroy(dict: *mut HunspellHandle);

    fn Hunspell_spell(dict: *const HunspellHandle, word: *const c_char) -> c_int;

}

pub struct Hunspell {
    inner: *mut HunspellHandle,
}

impl Hunspell {
    pub fn new(aff_path: &OsStr, dic_path: &OsStr) -> Hunspell {
        let aff_path = CString::new(aff_path.as_encoded_bytes()).unwrap();
        let dic_path = CString::new(dic_path.as_encoded_bytes()).unwrap();
        unsafe {
            Hunspell {
                inner: Hunspell_create(aff_path.as_ptr(), dic_path.as_ptr()),
            }
        }
    }

    pub fn spell(&self, word: &str) -> bool {
        let word = CString::new(word).unwrap();
        unsafe { Hunspell_spell(self.inner.cast_const(), word.as_ptr()) != 0 }
    }
}

impl Drop for Hunspell {
    fn drop(&mut self) {
        unsafe { Hunspell_destroy(self.inner) };
    }
}

unsafe impl Send for Hunspell {}
unsafe impl Sync for Hunspell {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn kick_the_tires_nuspell() {
        let manifest_path = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
        let aff_path = manifest_path.join("vendor/en_US/en_US.aff");
        let dict = Nuspell::new(aff_path.as_os_str());

        assert!(dict.spell("hello"));
        assert!(dict.spell("world"));

        assert!(!dict.spell("exmaple"));
    }

    #[test]
    fn kick_the_tires_hunspell() {
        let manifest_path = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
        let aff_path = manifest_path.join("vendor/en_US/en_US.aff");
        let dic_path = manifest_path.join("vendor/en_US/en_US.dic");
        let dict = Hunspell::new(aff_path.as_os_str(), dic_path.as_os_str());

        assert!(dict.spell("hello"));
        assert!(dict.spell("world"));

        assert!(!dict.spell("exmaple"));
    }
}
