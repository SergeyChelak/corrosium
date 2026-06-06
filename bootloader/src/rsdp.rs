// use bootinfo::ConfigTable;
// use uefi::table::cfg::ConfigTableEntry;

// pub struct RSDP {
//     pub acpi: ConfigTable,
//     pub smbios: ConfigTable,
// }

// impl RSDP {
//     pub fn setup() -> uefi::Result<Self> {
//         let mut acpi_table: Option<ConfigTable> = None;
//         let mut smbios_table: Option<ConfigTable> = None;

//         uefi::system::with_config_table(|entry| {
//             for cfg in entry {
//                 if let Some(acpi) = acpi_config_table(cfg) {
//                     if is_newer_table(&acpi, &acpi_table) {
//                         acpi_table = Some(acpi);
//                     }
//                     continue;
//                 }

//                 if let Some(smbios) = smbios_config_table(cfg) {
//                     if is_newer_table(&smbios, &smbios_table) {
//                         smbios_table = Some(smbios);
//                     }
//                     continue;
//                 }
//             }
//         });

//         let (Some(acpi), Some(smbios)) = (acpi_table, smbios_table) else {
//             return Err(uefi::Status::NOT_FOUND.into());
//         };

//         Ok(Self { acpi, smbios })
//     }
// }

// fn acpi_config_table(entry: &ConfigTableEntry) -> Option<ConfigTable> {
//     let Some((address, version)) = (match entry.guid {
//         ConfigTableEntry::ACPI2_GUID => Some((entry.address, 2)),
//         ConfigTableEntry::ACPI_GUID => Some((entry.address, 1)),
//         _ => None,
//     }) else {
//         return None;
//     };
//     let acpi = ConfigTable { address, version };
//     Some(acpi)
// }

// fn smbios_config_table(entry: &ConfigTableEntry) -> Option<ConfigTable> {
//     let Some((address, version)) = (match entry.guid {
//         ConfigTableEntry::SMBIOS3_GUID => Some((entry.address, 3)),
//         ConfigTableEntry::SMBIOS_GUID => Some((entry.address, 1)),
//         _ => None,
//     }) else {
//         return None;
//     };
//     let smbios = ConfigTable { address, version };
//     Some(smbios)
// }

// fn is_newer_table(candidate: &ConfigTable, other: &Option<ConfigTable>) -> bool {
//     let Some(other) = other else {
//         return true;
//     };
//     candidate.version > other.version
// }
