pub struct WRam {
    bank0: [u8; 0x1000],
    bankn: [[u8; 0x1000]; 7],
    ram_bank: u8, // Always 1 for non-CGB
}

impl WRam {
    pub fn default() -> WRam {
        WRam {
            bank0: [0; 0x1000],
            bankn: [[0; 0x1000]; 7],
            ram_bank: 1,
        }
    }

    pub fn write_u8(&mut self, address: u16, value: u8) {
        let ram_bank = self.ram_bank.saturating_sub(1);

        match address {
            0xC000..=0xCFFF => self.bank0[(address - 0xC000) as usize] = value,
            0xD000..=0xDFFF => {
                // if address < 0xDD50 && address >= 0xDD00 {
                //     println!("WRam Write: 0x{:04X} -- 0x{:02X}", address, value);
                // }
                self.bankn[ram_bank as usize][(address - 0xD000) as usize] = value
            }
            0xE000..=0xEFFF => self.bank0[(address - 0xE000) as usize] = value,
            0xF000..=0xFDFF => self.bankn[ram_bank as usize][(address - 0xF000) as usize] = value,
            _ => panic!("Invalid WRam Write address: 0x{:04X}", address),
        }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        let ram_bank = self.ram_bank.saturating_sub(1);

        match address {
            0xC000..=0xCFFF => self.bank0[(address - 0xC000) as usize],
            0xD000..=0xDFFF => {
                let result = self.bankn[ram_bank as usize][(address - 0xD000) as usize];
                // if (address < 0xD601 && address >= 0xD600) {
                //     println!("WRam Read: 0x{:04X} -- 0x{:02X}", address, result);
                // }
                result
            }
            0xE000..=0xEFFF => self.bank0[(address - 0xE000) as usize],
            0xF000..=0xFDFF => self.bankn[ram_bank as usize][(address - 0xF000) as usize],
            _ => panic!("Invalid WRam Read address: 0x{:04X}", address),
        }
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        let low_byte = self.read_u8(address) as u16;
        let high_byte = self.read_u8(address + 1) as u16;

        // println!(
        //     "WRam Read: 0x{:04X} -- 0x{:04X}",
        //     address,
        //     (high_byte << 8) | low_byte
        // );

        (high_byte << 8) | low_byte
    }
}
