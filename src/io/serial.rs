pub struct Serial {
    transfer_data: u8,    // FF01
    transfer_control: u8, // FF02
}

impl Serial {
    pub fn default() -> Serial {
        Serial {
            transfer_data: 0,
            transfer_control: 0,
        }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            0xFF01 => self.transfer_data,
            0xFF02 => self.transfer_control,
            _ => panic!("Invalid Serial Read address: 0x{:04X}", address),
        }
    }

    pub fn write_u8(&mut self, address: u16, value: u8) {
        match address {
            0xFF01 => self.transfer_data = value,
            0xFF02 => {
                self.transfer_control = value;

                // Temp Blargg output
                // if value == 0x81 {
                //     print!("{}", self.transfer_data as char);
                // }
            }
            _ => panic!("Invalid Serial Write address: 0x{:04X}", address),
        }
    }
}
