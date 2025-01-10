use std::fs::File;
use std::io::Read;

use super::cartridge_header::CartridgeHeader;

pub struct Cartridge {
    file_data: Vec<u8>,
    rom_bank: u8,
    header: CartridgeHeader,
}

impl Cartridge {
    pub fn new(filename: &str) -> Cartridge {
        let file_data = read_file(filename).unwrap();
        let header = CartridgeHeader::new(file_data.clone());

        Cartridge {
            file_data,
            header,
            rom_bank: 0,
        }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.file_data[address as usize],
            0x4000..=0x7FFF => {
                let rom_bank = self.rom_bank.saturating_sub(1);
                let offset = (rom_bank as usize) * 0x4000;
                // println!("{} -- {}", rom_bank, offset);
                self.file_data[(address as usize) + offset]
            }
            _ => panic!("Invalid Cartridge Read address: 0x{:04X}", address),
        }
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        match address {
            0x0000..=0x7FFF => {
                let low_byte = self.read_u8(address) as u16;
                let high_byte = self.read_u8(address + 1) as u16;

                (high_byte << 8) | low_byte
            }
            _ => panic!("Invalid Cartridge Read address: 0x{:04X}", address),
        }
    }

    pub fn write_u8(&mut self, address: u16, value: u8) {
        match address {
            0x2000..=0x3FFF => {
                // println!("Switching ROM Bank to {}", value & 0x1F);
                self.rom_bank = value & self.header.get_rom_bank_mask();
            }
            0x4000..=0x5FFF => panic!("Ram bank switching not implemented"),
            _ => panic!(
                "Invalid Cartridge Write address: 0x{:04X} -- 0x{:02X}",
                address, value
            ),
        }
    }
}

pub fn read_file(filename: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file: File = File::open(filename)?;
    let mut buffer: Vec<u8> = Vec::new();

    file.read_to_end(&mut buffer).unwrap();

    Ok(buffer)
}
