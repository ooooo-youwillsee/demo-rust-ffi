use std::ffi::{c_char, CString};

extern {
    fn strlen(ptr: *const c_char) -> usize;
}

fn main() {
    let s = CString::new("123").unwrap();
    let len = unsafe { strlen(s.as_ptr()) };
    println!("len: {}", len);
}
