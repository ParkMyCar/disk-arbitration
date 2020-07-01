#![deny(missing_debug_implementations)]

use libc::{
    c_char,
    c_void,
};

#[derive(Debug)]
#[repr(C)]
pub struct _DADisk(c_void);
pub type DADiskRef = *const _DADisk;

#[link(name = "DiskArbitration", kind = "framework")]
extern "C" {
    pub fn DADiskGetBSDName(disk: DADiskRef) -> *const c_char;
}
