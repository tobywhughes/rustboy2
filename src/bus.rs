use crate::{
    cartridge::cartridge::Cartridge, hram::HRam, io::io::IO, memory::MemoryLocation,
    opcode::opcode::execute_opcode, wram::WRam,
};

use super::cpu::CPU;

type ProcessedTile = [[u8; 8]; 8];
type ProcessedTiles = [[ProcessedTile; 32]; 32];
pub struct Bus {
    cartridge: Cartridge,
    pub cpu: CPU,
    wram: WRam,
    pub io: IO,
    hram: HRam,
    render_cycles: u32,
}

impl Bus {
    pub fn new(filename: &str) -> Bus {
        Bus {
            cartridge: Cartridge::new(filename),
            cpu: CPU::default(),
            wram: WRam::default(),
            hram: HRam::default(),
            io: IO::default(),
            render_cycles: 0,
        }
    }

    fn read_instruction(&mut self) -> u8 {
        let pc = self.cpu.get_pc_and_increment();

        let opcode = self.read_u8(pc);

        opcode
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        let memory_location = MemoryLocation::parse_address(address);
        let u16_value = match memory_location {
            MemoryLocation::Bank0 | MemoryLocation::BankN => self.cartridge.read_u16(address),
            MemoryLocation::WorkRamBank0
            | MemoryLocation::WorkRamBankN
            | MemoryLocation::EchoRam => self.wram.read_u16(address),
            _ => panic!("[Read u16] Unimplemented Memory Location 0x{:04X}", address),
        };

        u16_value
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        let memory_location = MemoryLocation::parse_address(address);
        let u8_value = match memory_location {
            MemoryLocation::Bank0 | MemoryLocation::BankN => self.cartridge.read_u8(address),
            MemoryLocation::WorkRamBank0
            | MemoryLocation::WorkRamBankN
            | MemoryLocation::EchoRam => self.wram.read_u8(address),
            MemoryLocation::HRam => self.hram.read_u8(address),
            MemoryLocation::IO
            | MemoryLocation::InterruptEnableRegister
            | MemoryLocation::VRam
            | MemoryLocation::Oam => self.io.read_u8(address),
            _ => panic!("[Read u8] Unimplemented Memory Location 0x{:04X}", address),
        };

        u8_value
    }

    pub fn write_u8(&mut self, address: u16, value: u8) {
        let memory_location = MemoryLocation::parse_address(address);
        match memory_location {
            MemoryLocation::Bank0 | MemoryLocation::BankN => {
                self.cartridge.write_u8(address, value)
            }
            MemoryLocation::WorkRamBank0
            | MemoryLocation::WorkRamBankN
            | MemoryLocation::EchoRam => self.wram.write_u8(address, value),
            MemoryLocation::IO
            | MemoryLocation::InterruptEnableRegister
            | MemoryLocation::VRam
            | MemoryLocation::Oam => self.io.write_u8(address, value),
            MemoryLocation::HRam => self.hram.write_u8(address, value),
            MemoryLocation::NotUsed => {} //No-op
            _ => panic!(
                "[Write u8] Unimplemented Memory Location 0x{:04X} for value {:02x}",
                address, value
            ),
        };
    }

    pub fn write_u16(&mut self, address: u16, value: u16) {
        let low_byte = (value & 0xFF) as u8;
        let high_byte = (value >> 8) as u8;

        self.write_u8(address, low_byte);
        self.write_u8(address + 1, high_byte);
    }

    pub fn run_cycle(&mut self) -> bool {
        let temp = self.io.interrupt.check_interrupts();
        if self.io.interrupt.check_interrupts() > 0 {
            self.cpu.add_m_cycles(5);

            self.cpu.resume();
            self.io.interrupt.disable_interrupts();
            self.push_u16_to_stack(self.cpu.get_pc());

            if self.io.interrupt.is_vblank_interrupt() {
                // println!("VBlank Interrupt");
                self.cpu.set_pc(0x0040);
                self.io.interrupt.disable_vblank_interrupt();
            } else if self.io.interrupt.is_lcd_interrupt() {
                self.cpu.set_pc(0x0048);
                self.io.interrupt.disable_lcd_interrupt();
            } else if self.io.interrupt.is_timer_interrupt() {
                self.cpu.set_pc(0x0050);
                self.io.interrupt.disable_timer_interrupt();
            } else {
                panic!("Unimplemented Interrupt {:08b}", temp);
            }
        } else {
            if self.cpu.is_halted {
                // Handle case where master interrupt is disabled but individual interrupts are enabled
                // TODO: May need extra handling if interrupt was already pending, so called "halt bug"
                if self.io.interrupt.has_interrupts() {
                    self.cpu.resume();
                }

                self.cpu.add_m_cycles(1);
            } else {
                let opcode = self.read_instruction();
                execute_opcode(self, opcode);
            }
        }

        // TODO: Figure out cycle timing
        if self.io.oam.dma_transfer {
            let start_address = (self.io.oam.dma as u16) << 8;
            for i in 0..0xA0 {
                let value = self.read_u8(start_address + i);
                self.io.oam.oam[i as usize] = value;
            }
            self.io.oam.dma_transfer = false;
        }

        self.render_cycles += self.cpu.cycle_buffer as u32;

        self.io
            .lcd
            .update_ly(self.cpu.cycle_buffer, &mut self.io.interrupt);
        self.io
            .timer
            .update_timer(&mut self.cpu.cycle_buffer, &mut self.io.interrupt);

        if self.render_cycles > 69905 {
            self.render_cycles -= 69905;
            true
        } else {
            false
        }
    }

    pub fn push_u16_to_stack(&mut self, value: u16) {
        let sp = self.cpu.push_sp();
        let high_byte = (value >> 8) as u8;
        let low_byte = (value & 0xFF) as u8;

        self.write_u8(sp, low_byte);
        self.write_u8(sp + 1, high_byte);
    }

    pub fn pop_u16_from_stack(&mut self) -> u16 {
        let sp = self.cpu.pop_sp();
        let low_byte = self.read_u8(sp - 2);
        let high_byte = self.read_u8(sp - 1);

        ((high_byte as u16) << 8) | low_byte as u16
    }
}
