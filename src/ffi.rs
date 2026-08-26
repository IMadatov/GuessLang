use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use crate::analyze_text;

/// Analyzes text and returns a JSON serialized string pointer.
/// The caller must free the returned pointer using `free_c_string`.
#[no_mangle]
pub extern "C" fn analyze_text_c(input: *const c_char) -> *mut c_char {
    if input.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(input) };
    let r_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let result = analyze_text(r_str);
    
    // Serialize result to JSON
    let json_result = serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string());

    match CString::new(json_result) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Frees a string previously allocated by `analyze_text_c`.
#[no_mangle]
pub extern "C" fn free_c_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(s);
    }
}
