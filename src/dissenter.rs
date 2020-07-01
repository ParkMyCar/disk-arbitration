#![deny(missing_debug_implementations)]

use core_foundation as cf;
use libc::{
    c_int,
    c_void,
};

#[derive(Debug)]
#[repr(C)]
pub struct _DADissenter(c_void);
pub type DADissenterRef = *const _DADissenter;

pub type DAReturn = c_int;

#[link(name = "DiskArbitration", kind = "framework")]
extern "C" {
    pub fn DADissenterCreate(
        allocator: cf::base::CFAllocatorRef,
        status: DAReturn,
        string: cf::string::CFStringRef,
    ) -> DADissenterRef;

    pub fn DADissenterGetStatus(
        dissenter: DADissenterRef,
    ) -> DAReturn;

    pub fn DADissenterGetStatusString(
        dissenter: DADissenterRef,
    ) -> cf::string::CFStringRef;
}
