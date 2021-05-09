use alloc::vec::Vec;
use x86_64::instructions::port::Port;

#[repr(u8)]
enum StatusFlags {
    Busy = 1 << 7,
    Error = 1,
    Ready = 1 << 3,
}

const PORT_DATA: u16 = 0x1f0;
const PORT_LBA0: u16 = 0x1f3;
const PORT_LBA1: u16 = 0x1f4;
const PORT_LBA2: u16 = 0x1f5;
const PORT_SELECT_DRIVE: u16 = 0x1f6;
const PORT_COMMAND: u16 = 0x1f7;
const PORT_DEV_CTRL: u16 = 0x3f6;

#[derive(Debug, Clone)]
pub struct DriveProperties {
    lba28_sectors: u32,
    lba48_sectors: Option<u64>,
}

pub struct AtaPioDriver;

impl AtaPioDriver {
    pub fn init() -> Vec<DriveProperties> {
        unsafe {
            Self::reset_drives();
        };

        let mut drives = Vec::<DriveProperties>::new();

        for drive_no in 0..=1 {
            if let Some(drive) = unsafe { Self::identify(drive_no) } {
                drives.push(drive);
            }
        }

        drives
    }

    #[inline]
    unsafe fn send_command(cmd: u8) {
        let mut command_port = Port::<u8>::new(PORT_COMMAND);
        command_port.write(cmd);
    }

    #[inline]
    unsafe fn read_status() -> u8 {
        let mut status_port = Port::<u8>::new(PORT_COMMAND);
        status_port.read()
    }

    unsafe fn reset_drives() {
        let mut control_port = Port::<u8>::new(PORT_DEV_CTRL);

        control_port.write(0);

        for _ in 0..4 {
            let _ = control_port.read();
        }

        loop {
            let v = control_port.read();
            if (v & 0xc0) == 0x40 {
                break;
            }
        }
    }

    #[inline]
    unsafe fn is_ready() -> bool {
        for _ in 0..4 {
            let _ = Self::read_status();
        }

        let data: u8 = Self::read_status();
        (data & 0xc0) == 0x40
    }

    unsafe fn wait_ready() {
        while !Self::is_ready() {}
    }

    unsafe fn clear_lba() {
        let mut port_lba0 = Port::<u8>::new(PORT_LBA0);
        let mut port_lba1 = Port::<u8>::new(PORT_LBA0);
        let mut port_lba2 = Port::<u8>::new(PORT_LBA0);

        port_lba0.write(0);
        port_lba1.write(0);
        port_lba2.write(0);
    }

    unsafe fn identify(drive: usize) -> Option<DriveProperties> {
        assert!(drive <= 1);

        let mut drive_selector = {
            if drive == 0 {
                0xa0
            } else {
                0xb0
            }
        };

        Self::clear_lba();
        // Self::send_command(0xEC);
        //
        // loop {
        //     let status: u8 = Self::read_status();
        //
        //     if status == 0 {
        //         return None;
        //     }
        //
        //     if (status & 1 != 0) {
        //         panic!("ATA: Drive error on IDENTIFY")
        //     }
        //
        //     if (status & (1 << 7)) != 0 {
        //         // STATUS_BUSY
        //         continue;
        //     }
        //
        //     // check lba's
        //
        //     if (status & (1 << 3) != 0) {
        //         break; // STATUS_DRQ, ready to receive/send data
        //     }
        // }
        //
        // let mut data_port = Port::<u16>::new(PORT_DATA);
        //
        // let mut data: [u16; 256] = [0; 256];
        //
        // for i in 0..256 {
        //     data[i] = data_port.read();
        // }
        //
        // let lba48_supported = (data[83] & (1 << 10)) != 0;
        // let lba28_sectors = (data[60] as u32) | ((data[61] as u32) << 0x10);
        // let lba48_sectors: Option<u64> = {
        //     if (lba48_supported) {
        //         Some(
        //             (data[100] as u64)
        //                 | (data[101] as u64) << 0x10
        //                 | (data[102] as u64) << 0x20
        //                 | (data[103] as u64) << 0x30,
        //         )
        //     } else {
        //         None
        //     }
        // };
        //
        // if lba28_sectors == 0 && (lba48_sectors.is_none() || lba48_sectors == Some(0)) {
        //     panic!("ATA: The drive controller does not support LBA :(")
        // }
        //
        // Some(DriveProperties {
        //     lba28_sectors,
        //     lba48_sectors,
        // })
        None
    }
}
