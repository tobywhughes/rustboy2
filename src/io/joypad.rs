pub struct Joypad {
    pub joypad_state: u8, // FF00
}

impl Joypad {
    pub fn default() -> Joypad {
        Joypad { joypad_state: 0x0F }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            0xFF00 => self.joypad_state,
            _ => panic!("Invalid Joypad Read address: 0x{:04X}", address),
        }
    }

    pub fn write_u8(&mut self, address: u16, value: u8) {
        match address {
            0xFF00 => self.joypad_state = (self.joypad_state & 0x0F) | (value & 0xF0),
            _ => panic!("Invalid Joypad Write address: 0x{:04X}", address),
        }
    }
}
