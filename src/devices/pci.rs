use crate::println;
use x86_64::instructions::port::Port;
use alloc::vec::Vec;
use alloc::fmt;
use core::fmt::Formatter;

pub struct PCIDevice {
    bus: u8,
    device: u8,
    vendor_id: u16,
    device_id: u16,
    function: u8,
    class_code: u8,
    subclass_code: u8,
    rev_id: u8
}

impl fmt::Display for PCIDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}.{:02}.{:01} {:04x}:{:04x} {:02x} {:02x} (rev id {:02x})", self.bus, self.device, self.function, self.vendor_id, self.device_id, self.class_code, self.subclass_code, self.rev_id)
    }
}

pub struct PCIEnumerator;

impl PCIEnumerator {
    fn config_read_dword(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
        let lbus = bus as u32;
        let lslot = slot as u32;
        let lfunc = func as u32;
        let loffset = offset as u32;

        let address =
            (lbus << 16) | (lslot << 11) | (lfunc << 8) | (loffset & 0xff) | (0x80000000 as u32);

        unsafe {
            let mut write_port = Port::<u32>::new(0xcf8);
            write_port.write(address);

            let mut read_port = Port::<u32>::new(0xcfc);
            read_port.read()
        }
    }

    fn config_get_vendor_id(bus: u8, device: u8, function: u8) -> u16 {
        (Self::config_read_dword(bus, device, function, 0) & 0xffff) as u16
    }

    fn config_get_device_id(bus: u8, device: u8, function: u8) -> u16 {
        (Self::config_read_dword(bus, device, function, 0x00) >> 16) as u16
    }

    fn config_get_class_code(bus: u8, device: u8, function: u8) -> u8 {
        (Self::config_read_dword(bus, device, function, 0x08) >> 24) as u8
    }

    fn config_get_sub_class_code(bus: u8, device: u8, function: u8) -> u8 {
        ((Self::config_read_dword(bus, device, function, 0x08) >> 16) & 0xff) as u8
    }

    fn config_get_header_type(bus: u8, device: u8) -> u8 {
        ((Self::config_read_dword(bus, device, 0, 0x0c) >> 16) & 0xff) as u8
    }

    fn config_get_revision_id(bus: u8, device: u8, function: u8) -> u8 {
        (Self::config_read_dword(bus, device, function, 0x08) & 0xff) as u8
    }

    fn get_pci_device(bus: u8, device: u8, function: u8) -> PCIDevice {
        let vendor_id = Self::config_get_vendor_id(bus, device, function);
        let device_id = Self::config_get_device_id(bus, device, function);
        let class_code = Self::config_get_class_code(bus, device, function);
        let subclass_code = Self::config_get_sub_class_code(bus, device, function);
        let rev_id = Self::config_get_revision_id(bus, device, function);

        PCIDevice {
            bus,
            device,
            vendor_id,
            device_id,
            function,
            class_code,
            subclass_code,
            rev_id,
        }
    }

    pub fn enumerate() -> Vec<PCIDevice> {
        let mut devices = Vec::<PCIDevice>::new();

        for bus in 0..=255 {
            for device in 0..=31 {
                let vendor_id = Self::config_get_vendor_id(bus, device, 0);

                if vendor_id == 0xffff {
                    continue
                }

                if (Self::config_get_header_type(bus, device) & 0x80) != 0 {
                    for function in 0..=7 {
                        let vendor_id = Self::config_get_vendor_id(bus, device, function);

                        if vendor_id == 0xffff {
                            continue
                        }
                        let class_code = Self::config_get_class_code(bus, device, function);

                        if class_code & 0xf0 != 0 {
                            continue
                        }
                        let pcidevice = Self::get_pci_device(bus, device, function);

                        devices.push(pcidevice);
                    }
                } else {
                    let pcidevice = Self::get_pci_device(bus, device, 0);
                    devices.push(pcidevice);
                }
            }
        }

        for device in &devices {
            println!("{}", device);
        }

        devices
    }
}
