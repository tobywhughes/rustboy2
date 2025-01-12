// TODO: I think this only works for MBC1 cartridges, need to implement other types

use byteorder::{BigEndian, ByteOrder};

#[derive(Debug)]
pub enum CartridgeChipType {
    ROMOnly,
    MBC1,
    MBC2,
    MBC3,
    MBC5,
    MBC6,
    MBC7,
    MMM01,
    HuC1,
    HuC3,
    Unknown,
}

impl From<CartridgeType> for CartridgeChipType {
    fn from(value: CartridgeType) -> Self {
        match value {
            CartridgeType::ROM => CartridgeChipType::ROMOnly,
            CartridgeType::MBC1 | CartridgeType::MBC1_RAM | CartridgeType::MBC1_RAM_BATTERY => {
                CartridgeChipType::MBC1
            }
            CartridgeType::MBC3 | CartridgeType::MBC3_RAM | CartridgeType::MBC3_RAM_BATTERY => {
                CartridgeChipType::MBC3
            }
            _ => CartridgeChipType::Unknown,
        }
    }
}

#[derive(Copy, Clone)]
pub enum CartridgeType {
    ROM = 0x00,
    MBC1 = 0x01,
    MBC1_RAM = 0x02,
    MBC1_RAM_BATTERY = 0x03,
    MBC2 = 0x05,
    MBC2_BATTERY = 0x06,
    ROM_RAM = 0x08,
    ROM_RAM_BATTERY = 0x09,
    MMM01 = 0x0B,
    MMM01_RAM = 0x0C,
    MMM01_RAM_BATTERY = 0x0D,
    MBC3_TIMER_BATTERY = 0x0F,
    MBC3_TIMER_RAM_BATTERY = 0x10,
    MBC3 = 0x11,
    MBC3_RAM = 0x12,
    MBC3_RAM_BATTERY = 0x13,
}

impl From<u8> for CartridgeType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => CartridgeType::ROM,
            0x01 => CartridgeType::MBC1,
            0x02 => CartridgeType::MBC1_RAM,
            0x03 => CartridgeType::MBC1_RAM_BATTERY,
            0x05 => CartridgeType::MBC2,
            0x06 => CartridgeType::MBC2_BATTERY,
            0x08 => CartridgeType::ROM_RAM,
            0x09 => CartridgeType::ROM_RAM_BATTERY,
            0x0B => CartridgeType::MMM01,
            0x0C => CartridgeType::MMM01_RAM,
            0x0D => CartridgeType::MMM01_RAM_BATTERY,
            0x0F => CartridgeType::MBC3_TIMER_BATTERY,
            0x10 => CartridgeType::MBC3_TIMER_RAM_BATTERY,
            0x11 => CartridgeType::MBC3,
            0x12 => CartridgeType::MBC3_RAM,
            0x13 => CartridgeType::MBC3_RAM_BATTERY,
            _ => panic!("Invalid Cartridge Type: 0x{:02X}", value),
        }
    }
}

#[derive(Debug)]
enum RomSize {
    KB32 = 0x00,  // 2 banks - 1 bit mask
    KB64 = 0x01,  // 4 banks - 2 bit mask
    KB128 = 0x02, // 8 banks - 3 bit mask
    KB256 = 0x03, // 16 banks - 4 bit mask
    KB512 = 0x04, // 32 banks - 5 bit mask - Anything above has more complicated banking masking
    MB1 = 0x05,   // 64 banks - 6 bit mask
    MB2 = 0x06,   // 128 banks - 7 bit mask
    MB4 = 0x07,   // 256 banks - 8 bit mask
    MB1_1 = 0x52, // I'll figure these out later
    MB1_2 = 0x53,
    MB1_5 = 0x54,
}

impl From<u8> for RomSize {
    fn from(value: u8) -> Self {
        match value {
            0x00 => RomSize::KB32,
            0x01 => RomSize::KB64,
            0x02 => RomSize::KB128,
            0x03 => RomSize::KB256,
            0x04 => RomSize::KB512,
            0x05 => RomSize::MB1,
            0x06 => RomSize::MB2,
            0x07 => RomSize::MB4,
            0x52 => RomSize::MB1_1,
            0x53 => RomSize::MB1_2,
            0x54 => RomSize::MB1_5,
            _ => panic!("Invalid ROM Size"),
        }
    }
}

impl RomSize {
    fn get_rom_bank_mask(&self) -> u8 {
        match self {
            RomSize::KB32 => 0x01,
            RomSize::KB64 => 0x03,
            RomSize::KB128 => 0x07,
            RomSize::KB256 => 0x0F,
            RomSize::KB512 => 0x1F,
            RomSize::MB1 => 0x3F,
            RomSize::MB2 => 0x7F,
            _ => panic!("Unimplemented ROM Banking"),
        }
    }
}

#[derive(Debug)]
enum RamSize {
    None = 0x00,
    KB2 = 0x01,
    KB8 = 0x02,
    KB32 = 0x03,
}

impl From<u8> for RamSize {
    fn from(value: u8) -> Self {
        match value {
            0x00 => RamSize::None,
            0x01 => RamSize::KB2,
            0x02 => RamSize::KB8,
            0x03 => RamSize::KB32,
            _ => panic!("Invalid RAM Size"),
        }
    }
}

pub struct CartridgeHeader {
    entry_point: u32,
    title: Vec<u8>,
    cgb_flag: u8,
    sgb_flag: u8,
    pub cartridge_type: CartridgeType,
    rom_size: RomSize,
    ram_size: RamSize,
}

impl CartridgeHeader {
    pub fn new(file_data: Vec<u8>) -> CartridgeHeader {
        let entry_point: u32 = BigEndian::read_u32(&file_data[0x100..0x104]);
        let title: Vec<u8> = file_data[0x134..0x143].to_vec();
        let cgb_flag: u8 = file_data[0x143];
        let sgb_flag: u8 = file_data[0x146];

        let cartridge_type: CartridgeType = CartridgeType::from(file_data[0x147]);
        let rom_size: RomSize = RomSize::from(file_data[0x148]);
        let ram_size: RamSize = RamSize::from(file_data[0x149]);

        println!("Rom Size: {:?}", rom_size);
        println!("Ram Size: {:?}", ram_size);

        CartridgeHeader {
            entry_point,
            title,
            cgb_flag,
            sgb_flag,
            cartridge_type,
            rom_size,
            ram_size,
        }
    }

    pub fn get_rom_bank_mask(&self) -> u8 {
        self.rom_size.get_rom_bank_mask()
    }
}
