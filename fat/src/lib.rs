#[repr(packed)]
struct BootSector {
    boot_jump_instructions: [u8; 3],
    oem_identifier: [u8; 8],
    byte_per_sector: u16,
    sectors_per_cluster: u8,
    reserver_sectors: u16,
    fat_count: u8,
    dir_entry_count: u16,
    total_sectors: u16,
    media_descriptor_type: u8,
    sectors_per_fat: u16,
    sectors_per_track: u16,
    heads: u16,
    hidden_sectors: u32,
    large_sector_count: u32,
    // extended boot record
    drive_number: u8,
    _reserved: u8,
    signature: u8,
    volume_id: u32,
    volume_label: [u8; 11],
    system_id: [u8; 8]
}

#[repr(packed)]
struct DirectoryEntry {
    name: [u8; 11],
    attributes: u8,
    _reserved: u8,
    created_time_tenths: u8,
    creation_time: u16,
    created_date: u16,
    accessed_date: u16,
    first_cluster_high: u16,
    modified_time: u16,
    modified_data: u16,
    first_cluster_low: u16,
    size: u32
}