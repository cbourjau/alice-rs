#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::os::raw::c_void;

    #[test]
    fn init_esd_object() {
        let local_path = CString::new("/home/christian/Downloads/AliESDs.root").unwrap();
        let mut esd = unsafe { esd_new(local_path.as_ptr()) };
        let mut sum = 0;
        for i in 0..10 {
            unsafe { esd_load_next(esd, i); }
            sum += unsafe { (*esd).Tracks_ };
        }
        assert!(sum >= 0, "No tracks loaded?!");
    }
}
