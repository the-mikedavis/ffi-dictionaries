use std::ffi::{c_char, c_int, CString};

enum NuspellDictionary {}

#[link(name = "nuspell")]
extern "C" {
    fn Dictionary_create(aff_path: *const c_char) -> *mut NuspellDictionary;

    fn Dictionary_destroy(dict: *mut NuspellDictionary);

    fn Dictionary_spell(dict: *mut NuspellDictionary, word: *const c_char) -> c_int;

}

pub struct Dictionary {
    inner: *mut NuspellDictionary,
}

impl Dictionary {
    pub fn new(affpath: &str) -> Self {
        let affpath = CString::new(affpath).unwrap();
        unsafe {
            Self {
                inner: Dictionary_create(affpath.as_ptr()),
            }
        }
    }

    pub fn spell(&self, word: &str) -> bool {
        let word = CString::new(word).unwrap();
        unsafe { Dictionary_spell(self.inner, word.as_ptr()) != 0 }
    }
}

impl Drop for Dictionary {
    fn drop(&mut self) {
        unsafe { Dictionary_destroy(self.inner) };
    }
}
