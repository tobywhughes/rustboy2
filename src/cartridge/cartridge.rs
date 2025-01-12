use std::fs::File;
use std::io::Read;

use super::cartridge_header::{CartridgeChipType, CartridgeHeader};

pub struct Cartridge {
    file_data: Vec<u8>,
    chip_type: CartridgeChipType,
    rom_bank: u8,
    header: CartridgeHeader,
    ram: Vec<Vec<u8>>,
    ram_bank: u8,
    ram_enabled: bool,
}

impl Cartridge {
    pub fn new(filename: &str) -> Cartridge {
        let file_data = read_file(filename).unwrap();
        let header = CartridgeHeader::new(file_data.clone());
        let chip_type = CartridgeChipType::from(header.cartridge_type);

        Cartridge {
            file_data,
            chip_type,
            header,
            rom_bank: 0,
            ram_bank: 0,
            ram_enabled: false,
            ram: vec![vec![0; 0x2000]; 16],
        }
    }

    pub fn read_u8_mbc1(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.file_data[address as usize],
            0x4000..=0x7FFF => {
                let rom_bank = self.rom_bank.saturating_sub(1);
                let offset = (rom_bank as usize) * 0x4000;
                self.file_data[(address as usize) + offset]
            }
            0xA000..=0xBFFF => {
                if self.ram_enabled {
                    let bank = self.ram_bank as usize;
                    let offset = (address as usize) - 0xA000;
                    self.ram[bank][offset]
                } else {
                    0xFF // Not guaranteed to be 0xFF but often is
                }
            }
            _ => panic!("[MBC1] Invalid Cartridge Read address: 0x{:04X}", address),
        }
    }

    pub fn read_u8_mbc3(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.file_data[address as usize],
            0x4000..=0x7FFF => {
                let rom_bank = self.rom_bank.saturating_sub(1);
                let offset = (rom_bank as usize) * 0x4000;
                self.file_data[(address as usize) + offset]
            }
            0xA000..=0xBFFF => {
                if self.ram_enabled {
                    let bank = self.ram_bank as usize;
                    let offset = (address as usize) - 0xA000;
                    self.ram[bank][offset]
                } else {
                    0xFF // Not guaranteed to be 0xFF but often is
                }
            }
            _ => panic!("[MBC3] Invalid Cartridge Read address: 0x{:04X}", address),
        }
    }

    pub fn read_u8_rom(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.file_data[address as usize],
            _ => panic!(
                "[ROM ONLY] Invalid Cartridge Read address: 0x{:04X}",
                address
            ),
        }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match self.chip_type {
            CartridgeChipType::MBC1 => self.read_u8_mbc1(address),
            CartridgeChipType::ROMOnly => self.read_u8_rom(address),
            CartridgeChipType::MBC3 => self.read_u8_mbc3(address),
            _ => panic!("Cartridge Chip Type not implemented: {:?}", self.chip_type),
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

    fn write_u8_mbc1(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram_enabled = value & 0x0F == 0x0A,
            0x2000..=0x3FFF => {
                self.rom_bank = value & self.header.get_rom_bank_mask();
            }
            0x4000..=0x5FFF => self.ram_bank = value & 0x03,
            0x6000..=0x7FFF => panic!("MBC1: Unimplemented Banking Mod Selectt: 0x{:02X}", value),
            0xA000..=0xBFFF => {
                if self.ram_enabled {
                    let bank = self.ram_bank as usize;
                    let offset = (address as usize) - 0xA000;
                    self.ram[bank][offset] = value;
                }
            }
            _ => panic!(
                "Invalid Cartridge Write address: 0x{:04X} -- 0x{:02X}",
                address, value
            ),
        }
    }

    // TODO: Implement RTC
    pub fn write_u8_mbc3(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram_enabled = value & 0x0F == 0x0A,
            0x2000..=0x3FFF => {
                self.rom_bank = value & self.header.get_rom_bank_mask();
            }
            0x4000..=0x5FFF => self.ram_bank = value & 0x03,
            0x6000..=0x7FFF => (),
            0xA000..=0xBFFF => {
                if self.ram_enabled {
                    let bank = self.ram_bank as usize;
                    let offset = (address as usize) - 0xA000;
                    self.ram[bank][offset] = value;
                }
            }
            _ => panic!(
                "Invalid Cartridge Write address: 0x{:04X} -- 0x{:02X}",
                address, value
            ),
        }
    }

    pub fn write_u8(&mut self, address: u16, value: u8) {
        match self.chip_type {
            CartridgeChipType::MBC1 => self.write_u8_mbc1(address, value),
            CartridgeChipType::ROMOnly => panic!("ROM Only Cartridge cannot be written to"),
            CartridgeChipType::MBC3 => self.write_u8_mbc3(address, value),
            _ => panic!("Cartridge Chip Type not implemented: {:?}", self.chip_type),
        }
    }
}

pub fn read_file(filename: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file: File = File::open(filename)?;
    let mut buffer: Vec<u8> = Vec::new();

    file.read_to_end(&mut buffer).unwrap();

    Ok(buffer)
}
