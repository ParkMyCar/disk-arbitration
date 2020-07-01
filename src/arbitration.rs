#![deny(missing_debug_implementations)]

use core_foundation as cf;

use crate::{
    disk::DADiskRef,
    dissenter::DADissenterRef,
    session::DASessionRef,
    types::UnsafeMutableRawPointer,
};

pub type DADiskAppearedCallback = Option<extern "C" fn(
    disk: DADiskRef,
    context: UnsafeMutableRawPointer,
)>;

pub type DADiskMountApprovalCallback = Option<extern "C" fn(
    disk: DADiskRef,
    context: UnsafeMutableRawPointer,
) -> DADissenterRef>;

#[link(name = "DiskArbitration", kind = "framework")]
extern "C" {
    pub fn DARegisterDiskAppearedCallback(
        session: DASessionRef,
        r#match: cf::dictionary::CFDictionaryRef,
        callback: DADiskAppearedCallback,
        context: UnsafeMutableRawPointer,
    );

    pub fn DARegisterDiskMountApprovalCallback(
        session: DASessionRef,
        r#match: cf::dictionary::CFDictionaryRef,
        callback: DADiskMountApprovalCallback,
        context: UnsafeMutableRawPointer,
    );
}
