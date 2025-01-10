use crate::bus::Bus;

use super::opcode::{Opcode, OpcodeData};

#[derive(Debug, PartialEq)]
enum CBInstruction {
    RLC,
    RRC,
    RL,
    RR,
    SLA,
    SRA,
    SWAP,
    SRL,
    BIT,
    RES,
    SET,
    Undetermined,
}

impl CBInstruction {
    fn from_opcode(opcode: u8) -> CBInstruction {
        match opcode {
            0x00..=0x07 => CBInstruction::RLC,
            0x08..=0x0F => CBInstruction::RRC,
            0x10..=0x17 => CBInstruction::RL,
            0x18..=0x1F => CBInstruction::RR,
            0x20..=0x27 => CBInstruction::SLA,
            0x28..=0x2F => CBInstruction::SRA,
            0x30..=0x37 => CBInstruction::SWAP,
            0x38..=0x3F => CBInstruction::SRL,
            0x40..=0x7F => CBInstruction::BIT,
            0x80..=0xBF => CBInstruction::RES,
            0xC0..=0xFF => CBInstruction::SET,
            _ => panic!("Invalid CB Instruction"),
        }
    }
}

// ------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
enum SourceRegister {
    B,
    C,
    D,
    E,
    H,
    L,
    AddressHL,
    A,
    Undetermined,
}

impl SourceRegister {
    fn from_opcode(opcode: u8) -> SourceRegister {
        let masked_opcode = opcode & 0x07;

        match masked_opcode {
            0x00 => SourceRegister::B,
            0x01 => SourceRegister::C,
            0x02 => SourceRegister::D,
            0x03 => SourceRegister::E,
            0x04 => SourceRegister::H,
            0x05 => SourceRegister::L,
            0x06 => SourceRegister::AddressHL,
            0x07 => SourceRegister::A,
            _ => panic!("Invalid Source Register"),
        }
    }

    fn get_data(&self, bus: &mut Bus) -> u8 {
        match self {
            SourceRegister::B => bus.cpu.get_b(),
            SourceRegister::C => bus.cpu.get_c(),
            SourceRegister::D => bus.cpu.get_d(),
            SourceRegister::E => bus.cpu.get_e(),
            SourceRegister::H => bus.cpu.get_h(),
            SourceRegister::L => bus.cpu.get_l(),
            SourceRegister::AddressHL => bus.read_u8(bus.cpu.get_hl()),
            SourceRegister::A => bus.cpu.get_a(),
            _ => panic!("Invalid Source Register"),
        }
    }

    fn set_data(&self, bus: &mut Bus, value: u8) -> () {
        match self {
            SourceRegister::B => bus.cpu.set_b(value),
            SourceRegister::C => bus.cpu.set_c(value),
            SourceRegister::D => bus.cpu.set_d(value),
            SourceRegister::E => bus.cpu.set_e(value),
            SourceRegister::H => bus.cpu.set_h(value),
            SourceRegister::L => bus.cpu.set_l(value),
            SourceRegister::AddressHL => bus.write_u8(bus.cpu.get_hl(), value),
            SourceRegister::A => bus.cpu.set_a(value),
            _ => panic!("Invalid Source Register"),
        }
    }
}

// ------------------------------------------------------------------------------------------------

pub struct CB {
    opcode_data: OpcodeData,
    cb_instruction: CBInstruction,
    source_register: SourceRegister,
}

impl CB {
    pub fn default() -> CB {
        let opcode = 0xCB;
        let mnemonic = String::from("CB");
        let cycles = 2;

        CB {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            cb_instruction: CBInstruction::Undetermined,
            source_register: SourceRegister::Undetermined,
        }
    }
}

impl Opcode for CB {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let cb_pc = bus.cpu.get_pc_and_increment();
        let cb_opcode = bus.read_u8(cb_pc);

        self.cb_instruction = CBInstruction::from_opcode(cb_opcode);
        self.source_register = SourceRegister::from_opcode(cb_opcode);

        if self.source_register == SourceRegister::AddressHL {
            self.opcode_data.cycles = 4
        }

        let value = self.source_register.get_data(bus);

        let mut is_carry = false;

        let result: u8 = match self.cb_instruction {
            CBInstruction::RLC => {
                is_carry = value & 0x80 == 0x80;

                value.rotate_left(1)
            }
            CBInstruction::RRC => {
                is_carry = value & 0x01 == 0x01;

                value.rotate_right(1)
            }
            CBInstruction::RL => {
                let carry = bus.cpu.get_carry_flag() as u8;

                is_carry = value & 0x80 == 0x80;

                (value << 1) | carry
            }
            CBInstruction::RR => {
                let carry = bus.cpu.get_carry_flag() as u8;

                is_carry = value & 0x01 == 0x01;

                (value >> 1) | (carry << 7)
            }
            CBInstruction::SLA => {
                is_carry = value & 0x80 == 0x80;

                value << 1
            }
            CBInstruction::SRA => {
                is_carry = value & 0x01 == 0x01;

                (value & 0x80) | (value >> 1)
            }
            CBInstruction::SWAP => {
                let low = value & 0x0F;
                let high = value & 0xF0;

                (low << 4) | (high >> 4)
            }
            CBInstruction::SRL => {
                is_carry = value & 0x01 == 0x01;

                value >> 1
            }
            CBInstruction::BIT => {
                if self.source_register == SourceRegister::AddressHL {
                    self.opcode_data.cycles = 3; // BIT takes 3 cycles when accessing memory
                }

                let bit = (cb_opcode & 0x38) >> 3;
                let mask = 1 << bit;

                value & mask
            }
            CBInstruction::SET => {
                let bit = (cb_opcode & 0x38) >> 3;
                let mask = 1 << bit;

                value | mask
            }
            CBInstruction::RES => {
                let bit = (cb_opcode & 0x38) >> 3;
                let mask = 1 << bit;

                value & !mask
            }
            _ => panic!("Unimplemented CB Instruction, 0x{:?}", self.cb_instruction),
        };

        if self.cb_instruction != CBInstruction::BIT {
            // BIT does not write back to the source register
            self.source_register.set_data(bus, result);
        }

        if self.cb_instruction == CBInstruction::BIT {
            bus.cpu.set_zero_flag(result == 0);
            bus.cpu.set_n_flag(false);
            bus.cpu.set_half_carry_flag(true);
        } else if self.cb_instruction == CBInstruction::SET
            || self.cb_instruction == CBInstruction::RES
        {
            // These Instructions do not affect flags
        } else {
            bus.cpu.set_zero_flag(result == 0);
            bus.cpu.set_n_flag(false);
            bus.cpu.set_half_carry_flag(false);
            bus.cpu.set_carry_flag(is_carry);
        }
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, bus: &mut Bus) -> () {
        println!(
            ":: CB {:?} <{:?}>",
            self.cb_instruction, self.source_register
        );
    }
}
