use std::ffi::CStr;
use std::os::raw::c_char;

extern "C" {
    fn liquid_libversion() -> *const c_char;
}

pub fn version() -> (i32, i32, i32) {
    let mut major = 0;
    let mut minor = 0;
    let mut patch = 0;
    unsafe {
        let liquid_version = CStr::from_ptr(liquid_libversion()).to_str().unwrap();
        let mut iter = liquid_version.split('.');
        major = iter.next().unwrap().parse().unwrap();
        minor = iter.next().unwrap().parse().unwrap();
        patch = iter.next().unwrap().parse().unwrap();
    }

    (major, minor, patch)
}
