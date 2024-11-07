use std::{ffi::{CStr, CString}, os::raw::c_char};

// Trait definition
pub trait PointersExport<T> {
    fn to_pointer_mut(&self) -> *mut c_char;
    fn to_pointer(&self) -> *const c_char;
}

pub trait PointerImport<T> {
    fn from_pointer(raw: *const c_char) -> T;
    fn from_pointer_mut(raw: *mut c_char) -> T;
}

// Implementation for `&str`
impl PointersExport<&str> for &str {
    fn to_pointer_mut(&self) -> *mut c_char {
        let c_string = CString::new(*self).expect("Failed to create CString");
        c_string.into_raw() // Convert to raw pointer
    }

    fn to_pointer(&self) -> *const c_char {
        let c_string = CString::new(*self).expect("Failed to create CString");
        c_string.as_ptr() // Convert to raw pointer
    }
}

// Implementation for `String`
impl PointersExport<String> for String {
    fn to_pointer_mut(&self) -> *mut c_char {
        let c_string = CString::new(self.as_str()).expect("Failed to create CString");
        c_string.into_raw() // Convert to raw pointer
    }

    fn to_pointer(&self) -> *const c_char {
        let c_string = CString::new(self.as_str()).expect("Failed to create CString");
        c_string.as_ptr() // Convert to raw pointer
    }
}



impl PointerImport<String> for String {
    fn from_pointer(raw: *const c_char) -> String {
        unsafe {
            // SAFETY: We assume the raw pointer is a valid pointer to a C string
            CStr::from_ptr(raw)
                .to_str()
                .expect("Failed to convert C string to &str")
                .to_string() // Convert to `String`
        }
    }

    fn from_pointer_mut(raw: *mut c_char) -> String {
        unsafe {
            // SAFETY: We assume the raw pointer is a valid pointer to a C string
            CStr::from_ptr(raw)
                .to_str()
                .expect("Failed to convert C string to &str")
                .to_string() // Convert to `String`
        }
    }
}
