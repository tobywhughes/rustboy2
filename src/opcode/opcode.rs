use crate::bus::{self, Bus};

use super::{
    arithmatic::{
        Add16BitRegister, Add8BitRegister, Add8BitRegisterWithCarry, AddSignedImmediateToSP,
        And8BitRegister, Compare8BitRegister, ComplementAccumulator, ComplementCarryFlag,
        DecimalAdjustAccumulator, DecrementRegister, DecrementRegister8Bit, IncrementRegister,
        IncrementRegister8Bit, Or8BitRegister, RotateAccumulator, SetCarryFlag,
        Subtract8BitRegister, Subtract8BitRegisterWithCarry, Xor8BitRegister,
    },
    cb::CB,
    control::{DisableInterrupts, EnableInterrupts, Halt, Noop},
    jump::{
        CallAddress, CallConditional, CallRST, CallReturn, JumpAddress, JumpConditional, JumpHL,
        JumpRelative, JumpRelativeConditional, ReturnConditional, ReturnEnableInterrupts,
    },
    load::{LoadRegister, PopRegister, PushRegister},
};

const BREAKPOINT: u16 = 0xC2B6;
const ENABLE_BREAKPOINT: bool = false;
static mut BREAKPOINT_HIT: bool = false;
const DEBUG_PRINT_OVERRIDE: bool = false;

pub fn execute_opcode(bus: &mut Bus, opcode: u8) -> () {
    // TODO: Consider an enum dispatch
    let mut opcode: Box<dyn Opcode> = match opcode {
        0x00 => Box::new(Noop::default()),
        0x03 | 0x13 | 0x23 | 0x33 => Box::new(IncrementRegister::default(opcode)),
        0x0B | 0x1B | 0x2B | 0x3B => Box::new(DecrementRegister::default(opcode)),
        0x09 | 0x19 | 0x29 | 0x39 => Box::new(Add16BitRegister::default(opcode)),
        0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C => {
            Box::new(IncrementRegister8Bit::default(opcode))
        }
        0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D => {
            Box::new(DecrementRegister8Bit::default(opcode))
        }
        0x18 => Box::new(JumpRelative::default()),
        0x01
        | 0x02
        | 0x08
        | 0x0A
        | 0x1A
        | 0x2A
        | 0x3A
        | 0x12
        | 0x22
        | 0x32
        | 0x06
        | 0x0E
        | 0x16
        | 0x1E
        | 0x26
        | 0x2E
        | 0x36
        | 0x3E
        | 0x11
        | 0x21
        | 0x31
        | 0x40..=0x75
        | 0x77..=0x7F
        | 0xE0
        | 0xE2
        | 0xEA
        | 0xF0
        | 0xF2
        | 0xF8
        | 0xF9
        | 0xFA => Box::new(LoadRegister::default(opcode)),
        0x20 | 0x30 | 0x28 | 0x38 => Box::new(JumpRelativeConditional::default(opcode)),
        0xC4 | 0xD4 | 0xCC | 0xDC => Box::new(CallConditional::default(opcode)),
        0xC2 | 0xCA | 0xD2 | 0xDA => Box::new(JumpConditional::default(opcode)),
        0xC0 | 0xC8 | 0xD0 | 0xD8 => Box::new(ReturnConditional::default(opcode)),
        0xE9 => Box::new(JumpHL::default()),
        0xC5 | 0xD5 | 0xE5 | 0xF5 => Box::new(PushRegister::default(opcode)),
        0xC1 | 0xD1 | 0xE1 | 0xF1 => Box::new(PopRegister::default(opcode)),
        0xC3 => Box::new(JumpAddress::default()),
        0xC9 => Box::new(CallReturn::default()),
        0xCD => Box::new(CallAddress::default()),
        0xF3 => Box::new(DisableInterrupts::default()),
        0x80..=0x87 | 0xC6 => Box::new(Add8BitRegister::default(opcode)),
        0x88..=0x8F | 0xCE => Box::new(Add8BitRegisterWithCarry::default(opcode)),
        0xB0..=0xB7 | 0xF6 => Box::new(Or8BitRegister::default(opcode)),
        0xB8..=0xBF | 0xFE => Box::new(Compare8BitRegister::default(opcode)),
        0xA0..=0xA7 | 0xE6 => Box::new(And8BitRegister::default(opcode)),
        0x90..=0x97 | 0xD6 => Box::new(Subtract8BitRegister::default(opcode)),
        0x98..=0x9F | 0xDE => Box::new(Subtract8BitRegisterWithCarry::default(opcode)),
        0xA8..=0xAF | 0xEE => Box::new(Xor8BitRegister::default(opcode)),
        0x07 | 0x0F | 0x17 | 0x1F => Box::new(RotateAccumulator::default(opcode)),
        0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => Box::new(CallRST::default(opcode)),
        0xCB => Box::new(CB::default()),
        0xFB => Box::new(EnableInterrupts::default()),
        0x27 => Box::new(DecimalAdjustAccumulator::default(opcode)),
        0x2F => Box::new(ComplementAccumulator::default(opcode)),
        0x76 => Box::new(Halt::default()),
        0x37 => Box::new(SetCarryFlag::default(opcode)),
        0x3F => Box::new(ComplementCarryFlag::default(opcode)),
        0xE8 => Box::new(AddSignedImmediateToSP::default(opcode)),
        0xD9 => Box::new(ReturnEnableInterrupts::default()),
        _ => panic!("Opcode Unimplemented: 0x{:02X}", opcode),
    };

    if (ENABLE_BREAKPOINT && (bus.cpu.get_previous_pc() == BREAKPOINT)) {
        unsafe {
            BREAKPOINT_HIT = true;
        }
    }

    let mut DEBUG_PRINT = false;

    unsafe {
        DEBUG_PRINT = DEBUG_PRINT_OVERRIDE || BREAKPOINT_HIT;
    }

    if DEBUG_PRINT {
        opcode.debug_print(bus)
    };
    opcode.execute(bus);
    opcode.update_cycles(bus);
    opcode.update_flags(bus);
    if DEBUG_PRINT {
        opcode.post_execute_debug_print(bus);
    }
    opcode.post_execution_pc_increment(bus);
}

pub trait Opcode {
    fn execute(&mut self, bus: &mut Bus) -> () {
        panic!("Unimplemented Opcode")
    }

    fn update_cycles(&self, bus: &mut Bus) -> () {
        bus.cpu.add_m_cycles(self.opcode_data().cycles);
    }
    fn update_flags(&self, bus: &mut Bus) -> () {
        // Default Noop - No Flags to Update
    }

    fn debug_print(&self, bus: &mut Bus) -> () {
        let opcode_data = self.opcode_data();
        let pc = bus.cpu.get_previous_pc();
        println!(
            "[0x{:04X}] {} <{:02X}>",
            pc, opcode_data.mnemonic, opcode_data.opcode
        );
    }

    fn post_execute_debug_print(&self, bus: &mut Bus) -> () {
        // Default Nothing Printed
    }

    fn post_execution_pc_increment(&self, bus: &mut Bus) -> () {
        bus.cpu
            .add_to_pc(self.opcode_data().post_execution_pc_increment);
    }

    fn opcode_data(&self) -> &OpcodeData;
}

pub struct OpcodeData {
    pub opcode: u8,
    pub cycles: u16,
    mnemonic: String,
    pub post_execution_pc_increment: u16,
}

impl OpcodeData {
    pub fn new(opcode: u8, cycles: u16, mnemonic: String) -> OpcodeData {
        OpcodeData {
            opcode,
            cycles,
            mnemonic,
            post_execution_pc_increment: 0,
        }
    }
}
