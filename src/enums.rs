use core_foundation as cf;

pub use cf::base::kCFAllocatorDefault;

#[link(name = "DiskArbitration", kind = "framework")]
extern {
    pub static kDADiskDescriptionMatchVolumeMountable: cf::dictionary::CFDictionaryRef;
}
