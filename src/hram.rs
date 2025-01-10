pub struct HRam {
    data: [u8; 0x7F],
}

impl HRam {
    pub fn default() -> HRam {
        HRam { data: [0; 0x7F] }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        self.data[address as usize - 0xFF80]
    }

    pub fn write_u8(&mut self, address: u16, value: u8) {
        self.data[address as usize - 0xFF80] = value;
    }
}
