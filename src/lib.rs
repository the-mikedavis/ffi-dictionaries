use std::ffi::{c_char, c_int, CStr, CString, OsStr};

enum NuspellHandle {}

extern "C" {
    fn Nuspell_create(aff_path: *const c_char) -> *mut NuspellHandle;

    fn Nuspell_destroy(dict: *mut NuspellHandle);

    fn Nuspell_spell(dict: *const NuspellHandle, word: *const c_char) -> c_int;

    fn Nuspell_suggest(
        dict: *const NuspellHandle,
        slst: *mut *mut *mut c_char,
        word: *const c_char,
    ) -> c_int;

    fn Nuspell_free_list(slst: *mut *mut *mut c_char, n: c_int);
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

    pub fn spell(&self, word: &CStr) -> bool {
        unsafe { Nuspell_spell(self.inner.cast_const(), word.as_ptr()) != 0 }
    }

    pub fn suggest(&self, word: &CStr, out: &mut Vec<String>) {
        out.clear();
        unsafe {
            let mut list = std::ptr::null_mut();
            let n = Nuspell_suggest(self.inner.cast_const(), &mut list, word.as_ptr());
            if n != 0 {
                for i in 0..n {
                    let item = CStr::from_ptr(*list.offset(i.try_into().unwrap()));
                    out.push(String::from(item.to_str().unwrap()));
                }
                Nuspell_free_list(&mut list, n);
            }
        }
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

extern "C" {
    fn Hunspell_create(aff_path: *const c_char, dic_path: *const c_char) -> *mut HunspellHandle;

    fn Hunspell_destroy(dict: *mut HunspellHandle);

    fn Hunspell_spell(dict: *const HunspellHandle, word: *const c_char) -> c_int;

    fn Hunspell_suggest(
        dict: *const HunspellHandle,
        slst: *mut *mut *mut c_char,
        word: *const c_char,
    ) -> c_int;

    fn Hunspell_free_list(dict: *const HunspellHandle, slst: *mut *mut *mut c_char, n: c_int);
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

    pub fn spell(&self, word: &CStr) -> bool {
        unsafe { Hunspell_spell(self.inner.cast_const(), word.as_ptr()) != 0 }
    }

    pub fn suggest(&self, word: &CStr, out: &mut Vec<String>) {
        out.clear();
        unsafe {
            let mut list = std::ptr::null_mut();
            let n = Hunspell_suggest(self.inner.cast_const(), &mut list, word.as_ptr()) as c_int;
            if n != 0 {
                for i in 0..n {
                    let item = CStr::from_ptr(*list.offset(i.try_into().unwrap()));
                    out.push(String::from(item.to_str().unwrap()));
                }
                Hunspell_free_list(self.inner.cast_const(), &mut list, n as c_int);
            }
        }
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

        assert!(dict.spell(c"hello"));
        assert!(dict.spell(c"world"));

        assert!(!dict.spell(c"exmaple"));

        let mut suggestions = Vec::new();
        dict.suggest(c"adveenture", &mut suggestions);
        assert!(suggestions.contains(&"adventure".to_string()));
    }

    #[test]
    fn kick_the_tires_hunspell() {
        let manifest_path = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
        let aff_path = manifest_path.join("vendor/en_US/en_US.aff");
        let dic_path = manifest_path.join("vendor/en_US/en_US.dic");
        let dict = Hunspell::new(aff_path.as_os_str(), dic_path.as_os_str());

        assert!(dict.spell(c"hello"));
        assert!(dict.spell(c"world"));

        assert!(!dict.spell(c"exmaple"));

        let mut suggestions = Vec::new();
        dict.suggest(c"adveenture", &mut suggestions);
        assert!(suggestions.contains(&"adventure".to_string()));
    }
}
