use core_foundation as cf;

pub use cf::base::kCFAllocatorDefault;

#[link(name = "DiskArbitration", kind = "framework")]
extern {
    pub static kDADiskDescriptionMatchVolumeMountable: cf::dictionary::CFDictionaryRef;

    pub static kDADiskDescriptionVolumeKindKey: cf::string::CFStringRef;            /* -> CFString      */
    pub static kDADiskDescriptionVolumeMountableKey: cf::string::CFStringRef;       /* -> CFBoolean     */
    pub static kDADiskDescriptionVolumeNameKey: cf::string::CFStringRef;            /* -> CFString      */
    pub static kDADiskDescriptionVolumeNetworkKey: cf::string::CFStringRef;         /* -> CFBoolean     */
    pub static kDADiskDescriptionVolumePathKey: cf::string::CFStringRef;            /* -> CFURL         */
    pub static kDADiskDescriptionVolumeTypeKey: cf::string::CFStringRef;            /* -> CFString      */
    pub static kDADiskDescriptionVolumeUUIDKey: cf::string::CFStringRef;            /* -> CFUUID        */

    pub static kDADiskDescriptionMediaBlockSizeKey: cf::string::CFStringRef;        /* -> CFNumber      */
    pub static kDADiskDescriptionMediaBSDMajorKey: cf::string::CFStringRef;         /* -> CFNumber      */
    pub static kDADiskDescriptionMediaBSDMinorKey: cf::string::CFStringRef;         /* -> CFNumber      */
    pub static kDADiskDescriptionMediaBSDNameKey: cf::string::CFStringRef;          /* -> CFString      */
    pub static kDADiskDescriptionMediaBSDUnitKey: cf::string::CFStringRef;          /* -> CFNumber      */
    pub static kDADiskDescriptionMediaContentKey: cf::string::CFStringRef;          /* -> CFString      */
    pub static kDADiskDescriptionMediaEjectableKey: cf::string::CFStringRef;        /* -> CFBoolean     */
    pub static kDADiskDescriptionMediaIconKey: cf::string::CFStringRef;             /* -> CFDictionary  */
    pub static kDADiskDescriptionMediaKindKey: cf::string::CFStringRef;             /* -> CFString      */
    pub static kDADiskDescriptionMediaLeafKey: cf::string::CFStringRef;             /* -> CFBoolean     */
    pub static kDADiskDescriptionMediaNameKey: cf::string::CFStringRef;             /* -> CFString      */
    pub static kDADiskDescriptionMediaPathKey: cf::string::CFStringRef;             /* -> CFString      */
    pub static kDADiskDescriptionMediaRemovableKey: cf::string::CFStringRef;        /* -> CFBoolean     */
    pub static kDADiskDescriptionMediaSizeKey: cf::string::CFStringRef;             /* -> CFNumber      */
    pub static kDADiskDescriptionMediaTypeKey: cf::string::CFStringRef;             /* -> CFString      */
    pub static kDADiskDescriptionMediaUUIDKey: cf::string::CFStringRef;             /* -> CFUUID        */
    pub static kDADiskDescriptionMediaWholeKey: cf::string::CFStringRef;            /* -> CFBoolean     */
    pub static kDADiskDescriptionMediaWritableKey: cf::string::CFStringRef;         /* -> CFBoolean     */
    pub static kDADiskDescriptionMediaEncryptedKey: cf::string::CFStringRef;        /* -> CFBoolean     */
    pub static kDADiskDescriptionMediaEncryptionDetailKey: cf::string::CFStringRef; /* -> CFNumber      */

    pub static kDADiskDescriptionDeviceGUIDKey: cf::string::CFStringRef;            /* -> CFData        */
    pub static kDADiskDescriptionDeviceInternalKey: cf::string::CFStringRef;        /* -> CFBoolean     */
    pub static kDADiskDescriptionDeviceModelKey: cf::string::CFStringRef;           /* -> CFString      */
    pub static kDADiskDescriptionDevicePathKey: cf::string::CFStringRef;            /* -> CFString      */
    pub static kDADiskDescriptionDeviceProtocolKey: cf::string::CFStringRef;        /* -> CFString      */
    pub static kDADiskDescriptionDeviceRevisionKey: cf::string::CFStringRef;        /* -> CFString      */
    pub static kDADiskDescriptionDeviceUnitKey: cf::string::CFStringRef;            /* -> CFNumber      */
    pub static kDADiskDescriptionDeviceVendorKey: cf::string::CFStringRef;          /* -> CFString      */
    pub static kDADiskDescriptionDeviceTDMLockedKey: cf::string::CFStringRef;       /* -> CFBoolean     */

    pub static kDADiskDescriptionBusNameKey: cf::string::CFStringRef;               /* -> CFString      */
    pub static kDADiskDescriptionBusPathKey: cf::string::CFStringRef;               /* -> CFString      */
}
