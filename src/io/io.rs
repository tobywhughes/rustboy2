use super::{
    cgb_registers::CGBRegisters, interrupts::Interrupt, joypad::Joypad, lcd::LCD,
    oam::ObjectAttributeMemory, serial::Serial, sound::Sound, timer::Timer, vram::VRam,
};

enum IOMap {
    Timer,
    Interrupt,
    Sound,
    LCD,
    VRam,
    OAM,
    Serial,
    CGBRegisters,
    Joypad,
    Unused, // Illegal writes to unused memory
    Unimplemented,
}

impl IOMap {
    fn parse_address(address: u16) -> IOMap {
        match address {
            0xFF01 | 0xFF02 => IOMap::Serial,
            0xFF4F | 0x8000..=0x9FFF => IOMap::VRam,
            0xFF46 | 0xFE00..=0xFE9F => IOMap::OAM,
            0xFF04..=0xFF07 => IOMap::Timer,
            0xFF10..=0xFF26 | 0xFF30..=0xFF3F => IOMap::Sound,
            0xFF40..=0xFF4B => IOMap::LCD,
            0xFF4D => IOMap::CGBRegisters,
            0xFF0F => IOMap::Interrupt,
            0xFFFF => IOMap::Interrupt,
            0xFF00 => IOMap::Joypad,
            0xFF7F => IOMap::Unused,
            _ => IOMap::Unimplemented,
        }
    }
}

pub struct IO {
    pub timer: Timer,
    pub interrupt: Interrupt,
    pub sound: Sound,
    pub lcd: LCD,
    pub vram: VRam,
    pub serial: Serial,
    pub cgb_registers: CGBRegisters,
    pub oam: ObjectAttributeMemory,
    pub joypad: Joypad,
}

impl IO {
    pub fn default() -> IO {
        IO {
            timer: Timer::default(),
            interrupt: Interrupt::default(),
            sound: Sound::default(),
            lcd: LCD::default(),
            vram: VRam::default(),
            serial: Serial::default(),
            cgb_registers: CGBRegisters::default(),
            oam: ObjectAttributeMemory::default(),
            joypad: Joypad::default(),
        }
    }

    pub fn write_u8(&mut self, address: u16, value: u8) {
        let io_map = IOMap::parse_address(address);
        match io_map {
            IOMap::Serial => {
                self.serial.write_u8(address, value);
            }
            IOMap::Timer => {
                self.timer.write_u8(address, value);
            }
            IOMap::Interrupt => {
                self.interrupt.write_u8(address, value);
            }
            IOMap::Sound => {
                self.sound.write_u8(address, value);
            }
            IOMap::LCD => {
                self.lcd.write_u8(address, value);
            }
            IOMap::VRam => {
                self.vram.write_u8(address, value);
            }
            IOMap::OAM => {
                self.oam.write_u8(address, value);
            }
            IOMap::CGBRegisters => {
                self.cgb_registers.write_u8(address, value);
            }
            IOMap::Joypad => {
                self.joypad.write_u8(address, value);
            }
            IOMap::Unused => {
                // Do nothing
            }
            _ => panic!(
                "Unimplemented IO Write 0x{:04X} with value {:02X}",
                address, value
            ),
        }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        let io_map = IOMap::parse_address(address);
        match io_map {
            IOMap::Serial => self.serial.read_u8(address),
            IOMap::Timer => self.timer.read_u8(address),
            IOMap::LCD => self.lcd.read_u8(address),
            IOMap::VRam => self.vram.read_u8(address),
            IOMap::OAM => self.oam.read_u8(address),
            IOMap::Interrupt => self.interrupt.read_u8(address),
            IOMap::Sound => self.sound.read_u8(address),
            IOMap::CGBRegisters => self.cgb_registers.read_u8(address),
            IOMap::Joypad => self.joypad.read_u8(address),
            IOMap::Unused => 0xFF, // Unused memory returns 0xFF
            _ => panic!("Unimplemented IO Read 0x{:04X}", address),
        }
    }
}
