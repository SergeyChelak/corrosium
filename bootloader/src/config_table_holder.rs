use core::fmt::Display;
use uefi::table::cfg::ConfigTableEntry;
use uefi::{Guid, guid};

pub struct ConfigTableEntryHolder<'a>(pub &'a ConfigTableEntry);

const UEFI_MEMORY_ATTRIBUTES_TABLE: Guid = guid!("dcfa911d-26eb-469f-a220-38b7dc461220");

const GUID: [(Guid, &'static str); 14] = [
    (ConfigTableEntry::ACPI2_GUID, "ACPI2"),
    (ConfigTableEntry::ACPI_GUID, "ACPI1"),
    (ConfigTableEntry::DEBUG_IMAGE_INFO_GUID, "Debug Image"),
    (ConfigTableEntry::DXE_SERVICES_GUID, "DXE Services"),
    (ConfigTableEntry::ESRT_GUID, "EFI System Resources"),
    (
        ConfigTableEntry::HAND_OFF_BLOCK_LIST_GUID,
        "Hand-off Block List",
    ),
    (
        ConfigTableEntry::LZMA_COMPRESS_GUID,
        "LZMA Compressed filesystem",
    ),
    (
        ConfigTableEntry::MEMORY_STATUS_CODE_RECORD_GUID,
        "Hand-off Status Code",
    ),
    (
        ConfigTableEntry::MEMORY_TYPE_INFORMATION_GUID,
        "Memory Type Information",
    ),
    (ConfigTableEntry::PROPERTIES_TABLE_GUID, "Properties Table"),
    (ConfigTableEntry::SMBIOS3_GUID, "SMBIOS3"),
    (ConfigTableEntry::SMBIOS_GUID, "SMBIOS1"),
    (
        ConfigTableEntry::TIANO_COMPRESS_GUID,
        "Tiano compressed filesystem",
    ),
    (UEFI_MEMORY_ATTRIBUTES_TABLE, "Memory Attributes"), // The new GUID
];

impl<'a> Display for ConfigTableEntryHolder<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for g in GUID.iter() {
            if g.0 == self.0.guid {
                return f.write_str(g.1);
            }
        }
        f.write_fmt(format_args!("{}", self.0.guid))
    }
}
