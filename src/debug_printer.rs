use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

pub struct DebugWriter;

impl DebugWriter {
    pub fn write_byte(&mut self, byte: u8) {
        use x86_64::instructions::port::Port;

        let mut out_port = Port::new(0xe9);

        unsafe { out_port.write(byte) };
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }
}

impl fmt::Write for DebugWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref DEBUG_WRITER: Mutex<DebugWriter> = Mutex::new(DebugWriter {});
}
