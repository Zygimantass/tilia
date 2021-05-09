use byteorder::{ReadBytesExt, LittleEndian};

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct BIOSParameterBlock {
    oem_name: [char; 8],
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reserved_sectors: u16,
    number_of_copies: u8,
    total_sector: u32,
    number_of_sectors_per_fat: u32,
    cluster_number_of_root_directory: u32,
    serial_number: u32,
    volume_name: [char; 11],
    fat_name: [char; 8],
}

impl BIOSParameterBlock {
    pub fn new(mut buf: [u8; 512]) -> BIOSParameterBlock {
        let mut oem_name = [char; 8];
        let mut volume_name = [char; 11];
        let mut fat_name = [char; 8];

        oem_name.copy_from_slice(&buf[0x3..0xb]);
        volume_label.copy_from_slice(&buf[0x47..0x52]);
        fat_name.copy_from_slice(&buf[0x52..0x5A]);

        BIOSParameterBlock {
            oem_name,
            bytes_per_sector: (&buf[0x0B..0x0D]).read_u16::<LittleEndian>().unwrap(),
            sectors_per_cluster: buf[0x0D],
            reserved_sectors: (&buf[0x0E..0x10]).read_u16::<LittleEndian>().unwrap(),
            number_of_copies: buf[0x10],
            total_sector: (&buf[0x20..0x24]).read_u32::<LittleEndian>().unwrap(),
            number_of_sectors_per_fat: (&buf[0x24..0x28]).read_u32::<LittleEndian>().unwrap(),
            cluster_number_of_root_directory: (&buf[0x2C..0x30]).read_u32::<LittleEndian>().unwrap(),
            serial_number: (&buf[0x43..0x47]).read_u32::<LittleEndian>().unwrap(),
            volume_name,
            fat_name
        }
    }
}