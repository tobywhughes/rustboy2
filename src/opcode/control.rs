use crate::bus::Bus;

use super::opcode::{Opcode, OpcodeData};

// ------------------------------------------------------------------------------------------------

pub struct Noop {
    opcode_data: OpcodeData,
}

impl Noop {
    pub fn default() -> Noop {
        let opcode = 0x00;
        let mnemonic = String::from("Noop");
        let cycles = 1;

        Noop {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
        }
    }
}

impl Opcode for Noop {
    fn execute(&mut self, _bus: &mut Bus) -> () {
        // Do Nothing
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }
}

// ------------------------------------------------------------------------------------------------

pub struct DisableInterrupts {
    opcode_data: OpcodeData,
}

impl DisableInterrupts {
    pub fn default() -> DisableInterrupts {
        let opcode = 0xF3;
        let mnemonic = String::from("Disable Interrupts");
        let cycles = 1;

        DisableInterrupts {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
        }
    }
}

impl Opcode for DisableInterrupts {
    fn execute(&mut self, bus: &mut Bus) -> () {
        bus.io.interrupt.disable_interrupts();
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }
}

pub struct EnableInterrupts {
    opcode_data: OpcodeData,
}

impl EnableInterrupts {
    pub fn default() -> EnableInterrupts {
        let opcode = 0xFB;
        let mnemonic = String::from("Enable Interrupts");
        let cycles = 1;

        EnableInterrupts {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
        }
    }
}

impl Opcode for EnableInterrupts {
    fn execute(&mut self, bus: &mut Bus) -> () {
        bus.io.interrupt.enable_interrupts();

        // let pc = bus.cpu.get_pc();
        // let next_instruction = bus.read_u8(pc);
        // let next_instruction1 = bus.read_u8(pc + 3);
        // let next_instruction2 = bus.read_u8(pc + 4);
        // let next_instruction3 = bus.read_u8(pc + 5);
        // let next_instruction4 = bus.read_u8(pc + 6);

        // println!(
        //     "Next Instruction: PC {:04X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} ",
        //     pc,
        //     next_instruction,
        //     next_instruction1,
        //     next_instruction2,
        //     next_instruction3,
        //     next_instruction4
        // );
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }
}

// ------------------------------------------------------------------------------------------------

pub struct Halt {
    opcode_data: OpcodeData,
}

impl Halt {
    pub fn default() -> Halt {
        let opcode = 0x76;
        let mnemonic = String::from("Halt");
        let cycles = 1;

        Halt {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
        }
    }
}

impl Opcode for Halt {
    fn execute(&mut self, bus: &mut Bus) -> () {
        bus.cpu.halt();
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }
}
