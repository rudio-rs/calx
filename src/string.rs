use core_foundation_sys::base::{Boolean, CFIndex, CFRange, CFRelease};
use core_foundation_sys::string::{
    kCFStringEncodingUTF8, CFStringGetBytes, CFStringGetLength, CFStringRef,
};

use std::ptr;

pub struct StringRef(CFStringRef);
impl StringRef {
    pub fn new(string_ref: CFStringRef) -> Self {
        Self(string_ref)
    }

    pub fn to_utf8(&self) -> Vec<u8> {
        if self.0.is_null() {
            return Vec::new();
        }

        let length: CFIndex = unsafe { CFStringGetLength(self.0) };
        if length == 0 {
            return Vec::new();
        }

        // Get the buffer size of the string.
        let range: CFRange = CFRange {
            location: 0,
            length,
        };
        let mut size: CFIndex = 0;
        let mut converted_chars: CFIndex = unsafe {
            CFStringGetBytes(
                self.0,
                range,
                kCFStringEncodingUTF8,
                0,
                false as Boolean,
                ptr::null_mut() as *mut u8,
                0,
                &mut size,
            )
        };
        assert_eq!(converted_chars, length);
        assert!(size > 0);

        // Then, allocate the buffer with the required size and actually copy data into it.
        let mut buffer = vec![b'\x00'; size as usize];
        converted_chars = unsafe {
            CFStringGetBytes(
                self.0,
                range,
                kCFStringEncodingUTF8,
                0,
                false as Boolean,
                buffer.as_mut_ptr(),
                size,
                ptr::null_mut() as *mut CFIndex,
            )
        };
        assert_eq!(converted_chars, length);

        buffer
    }
}

impl Drop for StringRef {
    fn drop(&mut self) {
        use std::os::raw::c_void;
        unsafe { CFRelease(self.0 as *mut c_void) };
    }
}

impl Default for StringRef {
    fn default() -> Self {
        Self(ptr::null())
    }
}

#[test]
fn test_create_cfstring_ref() {
    use coreaudio_sys::{kCFAllocatorDefault, CFStringCreateWithBytes};
    fn cfstringref_from_string(string: &str) -> CFStringRef {
        let cfstringref = unsafe {
            CFStringCreateWithBytes(
                kCFAllocatorDefault,
                string.as_ptr(),
                string.len() as i64,
                kCFStringEncodingUTF8,
                false as Boolean,
            )
        };
        cfstringref as CFStringRef
    }
    let expected1 = "Rustaceans 🦀";
    let stringref1 = StringRef::new(cfstringref_from_string(expected1) as CFStringRef);
    assert_eq!(expected1.as_bytes(), stringref1.to_utf8());

    let expected2 = "";
    let stringref2 = StringRef::new(cfstringref_from_string(expected2) as CFStringRef);
    assert_eq!(expected2.as_bytes(), stringref2.to_utf8());
}
