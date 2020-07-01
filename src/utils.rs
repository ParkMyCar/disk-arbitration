#![deny(missing_debug_implementations)]

use core_foundation as cf;
use libc::c_void;

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    pub fn CFDictionaryGetValue(
        theDict: cf::dictionary::CFDictionaryRef,
        key: cf::string::CFStringRef,
    ) -> *const c_void;
}