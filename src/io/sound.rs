pub struct Sound {
    channel_control: u8,
    output_terminal: u8,
    sound_on: u8,
}

impl Sound {
    pub fn default() -> Sound {
        Sound {
            channel_control: 0,
            output_terminal: 0,
            sound_on: 0xF1, // TODO: Alternate FF26 Initial Values [$FF26] = $F1-GB, $F0-SGB ; NR52
        }
    }

    pub fn write_u8(&mut self, address: u16, value: u8) {
        match address {
            0xFF24 => self.channel_control = value,
            0xFF25 => self.output_terminal = value,
            0xFF26 => {
                if value & 0x80 == 0 {
                    self.sound_on |= 0x80;
                } else {
                    self.sound_on &= 0x7F;
                }
            }
            _ => (), // Currently just a noop because implementing sound later
        }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            0xFF24 => self.channel_control,
            0xFF25 => self.output_terminal,
            0xFF26 => self.sound_on,
            _ => 0xFF,
        }
    }
}
