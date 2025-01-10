use crate::bus::Bus;

use super::opcode::{Opcode, OpcodeData};

pub struct JumpAddress {
    opcode_data: OpcodeData,
    jump_address: u16,
}

impl JumpAddress {
    pub fn default() -> JumpAddress {
        let opcode = 0xC3;
        let mnemonic = String::from("Jump to Address");
        let cycles = 4;

        JumpAddress {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            jump_address: 0,
        }
    }
}

impl Opcode for JumpAddress {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let pc = bus.cpu.get_pc();
        let address = bus.read_u16(pc);

        self.jump_address = address;
        bus.cpu.set_pc(address);
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, bus: &mut Bus) -> () {
        println!(":: Jumping to 0x{:04X}", self.jump_address);
    }
}

// ------------------------------------------------------------------------------------------------

pub struct CallAddress {
    opcode_data: OpcodeData,
    return_address: u16,
    call_address: u16,
}

impl CallAddress {
    pub fn default() -> CallAddress {
        let opcode = 0xCD;
        let mnemonic = String::from("Call to Address");
        let cycles = 6;

        CallAddress {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            call_address: 0,
            return_address: 0,
        }
    }
}

impl Opcode for CallAddress {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let pc = bus.cpu.get_pc();
        self.call_address = bus.read_u16(pc);

        self.return_address = pc + 2;
        bus.push_u16_to_stack(self.return_address);
        bus.cpu.set_pc(self.call_address);
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, bus: &mut Bus) -> () {
        println!(
            ":: Calling 0x{:04X} - Return Address 0x{:04X} - Stack Pointer 0x{:04X}",
            self.call_address,
            self.return_address,
            bus.cpu.get_sp()
        );
    }
}
//

pub struct CallRST {
    opcode_data: OpcodeData,
    return_address: u16,
    rst_vector: u16,
}

impl CallRST {
    pub fn default(opcode: u8) -> CallRST {
        let mnemonic = format!("Call to RST");
        let cycles = 4;

        CallRST {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            return_address: 0,
            rst_vector: 0,
        }
    }
}

impl Opcode for CallRST {
    fn execute(&mut self, bus: &mut Bus) -> () {
        self.return_address = bus.cpu.get_pc();
        bus.push_u16_to_stack(self.return_address);

        self.rst_vector = match self.opcode_data.opcode {
            0xC7 => 0x0000,
            0xCF => 0x0008,
            0xD7 => 0x0010,
            0xDF => 0x0018,
            0xE7 => 0x0020,
            0xEF => 0x0028,
            0xF7 => 0x0030,
            0xFF => 0x0038,
            _ => panic!("Unimplemented RST Call"),
        };

        bus.cpu.set_pc(self.rst_vector);
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, bus: &mut Bus) -> () {
        println!(
            ":: Calling Vector {:02X}h- Return Address 0x{:04X} - Stack Pointer 0x{:04X}",
            self.rst_vector,
            self.return_address,
            bus.cpu.get_sp()
        );
    }
}

// ------------------------------------------------------------------------------------------------

pub struct JumpRelative {
    opcode_data: OpcodeData,
    jump_offset: i8,
}

impl JumpRelative {
    pub fn default() -> JumpRelative {
        let opcode = 0x18;
        let mnemonic = String::from("Jump with Relative Offset");
        let cycles = 3;

        JumpRelative {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            jump_offset: 0,
        }
    }
}

impl Opcode for JumpRelative {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let pc = bus.cpu.get_pc();
        self.jump_offset = bus.read_u8(pc) as i8 + 1; // Relative to next instruction

        bus.cpu.add_i8_to_pc(self.jump_offset);
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, bus: &mut Bus) -> () {
        println!(
            ":: Jumping with Offset {:02X} to PC 0x{:04X}",
            self.jump_offset,
            bus.cpu.get_pc()
        );
    }
}

// ------------------------------------------------------------------------------------------------

pub struct CallReturn {
    opcode_data: OpcodeData,
    return_address: u16,
}

impl CallReturn {
    pub fn default() -> CallReturn {
        let opcode = 0xC9;
        let mnemonic = String::from("Return from Call");
        let cycles = 4;

        CallReturn {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            return_address: 0,
        }
    }
}

impl Opcode for CallReturn {
    fn execute(&mut self, bus: &mut Bus) -> () {
        self.return_address = bus.pop_u16_from_stack();
        bus.cpu.set_pc(self.return_address);
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, bus: &mut Bus) -> () {
        println!(
            ":: Returning to 0x{:04X} - Stack Pointer 0x{:04X}",
            self.return_address,
            bus.cpu.get_sp()
        );
    }
}

// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
enum JumpCondition {
    Zero,
    NotZero,
    Carry,
    NotCarry,
    Undetermined,
}

impl JumpCondition {
    fn from_opcode(opcode: u8) -> JumpCondition {
        match opcode {
            0x20 | 0xC4 | 0xC2 | 0xC0 => JumpCondition::NotZero,
            0x28 | 0xCC | 0xCA | 0xC8 => JumpCondition::Zero,
            0x30 | 0xD4 | 0xD2 | 0xD0 => JumpCondition::NotCarry,
            0x38 | 0xDC | 0xDA | 0xD8 => JumpCondition::Carry,
            _ => panic!("Unimplemented Jump Condition"),
        }
    }
}

pub struct JumpRelativeConditional {
    opcode_data: OpcodeData,
    jump_offset: i8,
    condition: bool,
    jump_condition: JumpCondition,
}

impl JumpRelativeConditional {
    pub fn default(opcode: u8) -> JumpRelativeConditional {
        let mnemonic = format!("Jump Conditional with Relative Offset");
        let cycles = 2;

        JumpRelativeConditional {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            jump_offset: 0,
            condition: false,
            jump_condition: JumpCondition::Undetermined,
        }
    }
}

impl Opcode for JumpRelativeConditional {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let pc = bus.cpu.get_pc();
        self.jump_offset = bus.read_u8(pc) as i8 + 1; // Relative to next instruction

        self.jump_condition = JumpCondition::from_opcode(self.opcode_data.opcode);
        self.condition = match self.jump_condition {
            JumpCondition::Zero => bus.cpu.get_zero_flag(),
            JumpCondition::NotZero => !bus.cpu.get_zero_flag(),
            JumpCondition::Carry => bus.cpu.get_carry_flag(),
            JumpCondition::NotCarry => !bus.cpu.get_carry_flag(),
            _ => panic!("Unimplemented Jump Condition"),
        };

        if self.condition {
            self.opcode_data.cycles = 3;
            bus.cpu.add_i8_to_pc(self.jump_offset);
        } else {
            self.opcode_data.post_execution_pc_increment = 1;
        }
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, bus: &mut Bus) -> () {
        if self.condition {
            println!(
                ":: Jumping with Offset {:02X} to PC 0x{:04X} - Condition: {:?}",
                self.jump_offset,
                bus.cpu.get_pc(),
                self.jump_condition,
            );
        } else {
            println!(":: Not Jumping - Condition: {:?}", self.jump_condition);
        }
    }
}

// ------------------------------------------------------------------------------------------------

pub struct CallConditional {
    opcode_data: OpcodeData,
    return_address: u16,
    call_address: u16,
    condition: bool,
    jump_condition: JumpCondition,
}

impl CallConditional {
    pub fn default(opcode: u8) -> CallConditional {
        let mnemonic = format!("Call Conditional to Address");
        let cycles = 3;

        CallConditional {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            call_address: 0,
            return_address: 0,
            condition: false,
            jump_condition: JumpCondition::Undetermined,
        }
    }
}

impl Opcode for CallConditional {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let pc = bus.cpu.get_pc();
        self.call_address = bus.read_u16(pc);

        self.jump_condition = JumpCondition::from_opcode(self.opcode_data.opcode);
        self.condition = match self.jump_condition {
            JumpCondition::Zero => bus.cpu.get_zero_flag(),
            JumpCondition::NotZero => !bus.cpu.get_zero_flag(),
            JumpCondition::Carry => bus.cpu.get_carry_flag(),
            JumpCondition::NotCarry => !bus.cpu.get_carry_flag(),
            _ => panic!("Unimplemented Jump Condition"),
        };

        if self.condition {
            self.opcode_data.cycles = 6;
            self.return_address = pc + 2;
            bus.push_u16_to_stack(self.return_address);
            bus.cpu.set_pc(self.call_address);
        } else {
            self.opcode_data.post_execution_pc_increment = 2;
        }
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, bus: &mut Bus) -> () {
        if self.condition {
            println!(
                ":: Calling 0x{:04X} - Return Address 0x{:04X} - Stack Pointer 0x{:04X} - Condition: {:?}",
                self.call_address,
                self.return_address,
                bus.cpu.get_sp(),
                self.jump_condition,
            );
        } else {
            println!(":: Not Calling - Condition: {:?}", self.jump_condition);
        }
    }
}

// ------------------------------------------------------------------------------------------------

pub struct JumpConditional {
    opcode_data: OpcodeData,
    jump_address: u16,
    condition: bool,
    jump_condition: JumpCondition,
}

impl JumpConditional {
    pub fn default(opcode: u8) -> JumpConditional {
        let mnemonic = format!("Jump Conditional to Address");
        let cycles = 3;

        JumpConditional {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            jump_address: 0,
            condition: false,
            jump_condition: JumpCondition::Undetermined,
        }
    }
}

impl Opcode for JumpConditional {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let pc = bus.cpu.get_pc();
        self.jump_address = bus.read_u16(pc);

        self.jump_condition = JumpCondition::from_opcode(self.opcode_data.opcode);
        self.condition = match self.jump_condition {
            JumpCondition::Zero => bus.cpu.get_zero_flag(),
            JumpCondition::NotZero => !bus.cpu.get_zero_flag(),
            JumpCondition::Carry => bus.cpu.get_carry_flag(),
            JumpCondition::NotCarry => !bus.cpu.get_carry_flag(),
            _ => panic!("Unimplemented Jump Condition"),
        };

        if self.condition {
            self.opcode_data.cycles = 4;
            bus.cpu.set_pc(self.jump_address);
        } else {
            self.opcode_data.post_execution_pc_increment = 2;
        }
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, _bus: &mut Bus) -> () {
        if self.condition {
            println!(
                ":: Jumping to 0x{:04X} - Condition: {:?}",
                self.jump_address, self.jump_condition
            );
        } else {
            println!(":: Not Jumping - Condition: {:?}", self.jump_condition);
        }
    }
}

// ------------------------------------------------------------------------------------------------

pub struct ReturnConditional {
    opcode_data: OpcodeData,
    return_address: u16,
    condition: bool,
    jump_condition: JumpCondition,
}

impl ReturnConditional {
    pub fn default(opcode: u8) -> ReturnConditional {
        let mnemonic = format!("Return Conditional from Call");
        let cycles = 2;

        ReturnConditional {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            return_address: 0,
            condition: false,
            jump_condition: JumpCondition::Undetermined,
        }
    }
}

impl Opcode for ReturnConditional {
    fn execute(&mut self, bus: &mut Bus) -> () {
        self.jump_condition = JumpCondition::from_opcode(self.opcode_data.opcode);
        self.condition = match self.jump_condition {
            JumpCondition::Zero => bus.cpu.get_zero_flag(),
            JumpCondition::NotZero => !bus.cpu.get_zero_flag(),
            JumpCondition::Carry => bus.cpu.get_carry_flag(),
            JumpCondition::NotCarry => !bus.cpu.get_carry_flag(),
            _ => panic!("Unimplemented Jump Condition"),
        };

        if self.condition {
            self.opcode_data.cycles = 5;
            self.return_address = bus.pop_u16_from_stack();
            bus.cpu.set_pc(self.return_address);
        }
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, bus: &mut Bus) -> () {
        if self.condition {
            println!(
                ":: Returning to 0x{:04X} - Stack Pointer 0x{:04X} - Condition: {:?}",
                self.return_address,
                bus.cpu.get_sp(),
                self.jump_condition,
            );
        } else {
            println!(":: Not Returning - Condition: {:?}", self.jump_condition);
        }
    }
}

//------------------------------------------------------------------------------------------------

pub struct JumpHL {
    opcode_data: OpcodeData,
}

impl JumpHL {
    pub fn default() -> JumpHL {
        let opcode = 0xE9;
        let mnemonic = String::from("Jump to HL");
        let cycles = 1;

        JumpHL {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
        }
    }
}

impl Opcode for JumpHL {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let hl = bus.cpu.get_hl();
        bus.cpu.set_pc(hl);
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, bus: &mut Bus) -> () {
        println!(":: Jumping to HL 0x{:04X}", bus.cpu.get_hl());
    }
}

// ------------------------------------------------------------------------------------------------

pub struct ReturnEnableInterrupts {
    opcode_data: OpcodeData,
    return_address: u16,
}

impl ReturnEnableInterrupts {
    pub fn default() -> ReturnEnableInterrupts {
        let opcode = 0xD9;
        let mnemonic = String::from("Return from Call and Enable Interrupts");
        let cycles = 4;

        ReturnEnableInterrupts {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            return_address: 0,
        }
    }
}

impl Opcode for ReturnEnableInterrupts {
    fn execute(&mut self, bus: &mut Bus) -> () {
        bus.io.interrupt.enable_interrupts();

        self.return_address = bus.pop_u16_from_stack();
        bus.cpu.set_pc(self.return_address);
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, bus: &mut Bus) -> () {
        println!(
            ":: Returning to 0x{:04X} - Stack Pointer 0x{:04X} - Interrupts Enabled",
            self.return_address,
            bus.cpu.get_sp()
        );
    }
}
