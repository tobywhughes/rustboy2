pub struct CGBRegisters {
    key_1: u8,
}

impl CGBRegisters {
    pub fn default() -> CGBRegisters {
        CGBRegisters { key_1: 0 }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            0xFF4D => 0xFF, // TODO: Implement CGB Speed Switching -- This is a temp value because blargg's DMG guards are buggy
            _ => panic!("Unimplemented CGB Register Read address: 0x{:04X}", address),
        }
    }

    pub fn write_u8(&mut self, address: u16, value: u8) {
        match address {
            0xFF4D => panic!("CGB Speed Switching not implemented"),
            _ => panic!(
                "Unimplemented CGB Register Write address: 0x{:04X} -- 0x{:02X}",
                address, value
            ),
        }
    }
}
