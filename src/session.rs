#![deny(missing_debug_implementations)]

use core_foundation as cf;
use libc::c_void;

#[derive(Debug)]
#[repr(C)]
pub struct _DASession(c_void);
pub type DASessionRef = *const _DASession;

#[link(name = "DiskArbitration", kind = "framework")]
extern "C" {
    pub fn DASessionCreate(
        allocator: cf::base::CFAllocatorRef,
    ) -> DASessionRef;

    pub fn DASessionGetTypeID() -> cf::base::CFTypeID;

    pub fn DASessionScheduleWithRunLoop(
        session: DASessionRef,
        runLoop: cf::runloop::CFRunLoopRef,
        runLoopMode: cf::string::CFStringRef,
    );

    pub fn DASessionUnscheduleFromRunLoop(
        session: DASessionRef,
        runLoop: cf::runloop::CFRunLoopRef,
        runLoopMode: cf::string::CFStringRef,
    );
}
