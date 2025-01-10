use std::io::Read;

use crate::bus::Bus;

use super::opcode::{Opcode, OpcodeData};

#[derive(Debug)]
enum LoadDestination {
    BC,
    DE,
    SP,
    HL,
    Address,
    AddressBC,
    AddressDE,
    AddressHL,
    Address16,
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    IoPort,
    IoPortC,
    HLI,
    HLD,
    Undetermined,
}

impl LoadDestination {
    fn from_opcode(value: u8) -> LoadDestination {
        match value {
            0x01 => LoadDestination::BC,
            0x11 => LoadDestination::DE,
            0x21 | 0xF8 => LoadDestination::HL,
            0x22 => LoadDestination::HLI,
            0x32 => LoadDestination::HLD,
            0x31 | 0xF9 => LoadDestination::SP,
            0x0A | 0x1A | 0x2A | 0x3A | 0x3E | 0x78..=0x7F | 0xF0 | 0xF2 | 0xFA => {
                LoadDestination::A
            }
            0x06 | 0x40..=0x47 => LoadDestination::B,
            0x0E | 0x48..=0x4F => LoadDestination::C,
            0x16 | 0x50..=0x57 => LoadDestination::D,
            0x1E | 0x58..=0x5F => LoadDestination::E,
            0x26 | 0x60..=0x67 => LoadDestination::H,
            0x2E | 0x68..=0x6F => LoadDestination::L,
            0xE0 => LoadDestination::IoPort,
            0xE2 => LoadDestination::IoPortC,
            0xEA => LoadDestination::Address,
            0x08 => LoadDestination::Address16,
            0x02 => LoadDestination::AddressBC,
            0x12 => LoadDestination::AddressDE,
            0x36 | 0x70..=0x75 | 0x77 => LoadDestination::AddressHL,
            _ => panic!("Unimplemented Load Destination: 0x{:02X}", value),
        }
    }
}

#[derive(Debug)]
enum ReadDestination {
    Address,
    AddressBC,
    AddressDE,
    AddressHL,
    Immediate,
    Immediate8,
    A,
    B,
    C,
    D,
    E,
    L,
    H,
    HLI,
    HLD,
    HL,
    SP,
    IoPort,
    IoPortC,
    RelativeSP,
}

impl ReadDestination {
    fn from_opcode(value: u8) -> ReadDestination {
        // TODO: Refactor this where possible to use bitwise operations
        match value {
            0x01 | 0x11 | 0x21 | 0x31 => ReadDestination::Immediate,
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => ReadDestination::Immediate8,
            0x02 | 0x12 | 0x22 | 0x32 | 0xE0 | 0xEA | 0x47 | 0x57 | 0x67 | 0x77 | 0x4F | 0x5F
            | 0x6F | 0x7F | 0xE2 => ReadDestination::A,
            0x40 | 0x50 | 0x60 | 0x70 | 0x48 | 0x58 | 0x68 | 0x78 => ReadDestination::B,
            0x41 | 0x51 | 0x61 | 0x71 | 0x49 | 0x59 | 0x69 | 0x79 => ReadDestination::C,
            0x42 | 0x52 | 0x62 | 0x72 | 0x4A | 0x5A | 0x6A | 0x7A => ReadDestination::D,
            0x43 | 0x53 | 0x63 | 0x73 | 0x4B | 0x5B | 0x6B | 0x7B => ReadDestination::E,
            0x44 | 0x54 | 0x64 | 0x74 | 0x4C | 0x5C | 0x6C | 0x7C => ReadDestination::H,
            0x45 | 0x55 | 0x65 | 0x75 | 0x4D | 0x5D | 0x6D | 0x7D => ReadDestination::L,
            0x2A => ReadDestination::HLI,
            0x3A => ReadDestination::HLD,
            0xF0 => ReadDestination::IoPort,
            0xF2 => ReadDestination::IoPortC,
            0xFA => ReadDestination::Address,
            0x1A => ReadDestination::AddressDE,
            0x0A => ReadDestination::AddressBC,
            0x46 | 0x56 | 0x66 | 0x4E | 0x5E | 0x6E | 0x7E => ReadDestination::AddressHL,
            0xF8 => ReadDestination::RelativeSP,
            0xF9 => ReadDestination::HL,
            0x08 => ReadDestination::SP,
            _ => panic!("Unimplemented Read Destination: 0x{:02X}", value),
        }
    }
}

pub struct LoadRegister {
    opcode_data: OpcodeData,
    load_destination: LoadDestination,
    load_address: u16,
    read_value: u16,
    port_offset: u16,
}

impl LoadRegister {
    pub fn default(opcode: u8) -> LoadRegister {
        let mnemonic = format!("Load Register");
        let cycles = 3;

        LoadRegister {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            load_destination: LoadDestination::Undetermined,
            load_address: 0,
            read_value: 0,
            port_offset: 0,
        }
    }
}

// Think I was wrong about address endianess, keeping this here for now
// fn swap_address_endianess(address: u16) -> u16 {
//     let low = address & 0x00FF;
//     let high = address & 0xFF00;

//     (low << 8) | (high >> 8)
// }

impl Opcode for LoadRegister {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let opcode = self.opcode_data.opcode;

        self.load_destination = LoadDestination::from_opcode(opcode);
        let read_destination = ReadDestination::from_opcode(opcode);

        // TODO: Verify address endianess logic
        self.read_value = match read_destination {
            ReadDestination::Immediate => {
                let pc = bus.cpu.get_pc();
                self.opcode_data.post_execution_pc_increment = 2;

                bus.read_u16(pc)
            }
            ReadDestination::Immediate8 => {
                let pc = bus.cpu.get_pc();
                self.opcode_data.post_execution_pc_increment = 1;

                bus.read_u8(pc) as u16
            }
            ReadDestination::A => bus.cpu.get_a() as u16,
            ReadDestination::B => bus.cpu.get_b() as u16,
            ReadDestination::C => bus.cpu.get_c() as u16,
            ReadDestination::D => bus.cpu.get_d() as u16,
            ReadDestination::E => bus.cpu.get_e() as u16,
            ReadDestination::H => bus.cpu.get_h() as u16,
            ReadDestination::L => bus.cpu.get_l() as u16,
            ReadDestination::HL => bus.cpu.get_hl(),
            ReadDestination::SP => bus.cpu.get_sp(),
            ReadDestination::HLI => {
                let hl = bus.cpu.get_hl();
                bus.cpu.set_hl(hl.wrapping_add(1));

                bus.read_u8(hl) as u16
            }
            ReadDestination::HLD => {
                let hl = bus.cpu.get_hl();
                bus.cpu.set_hl(hl.wrapping_sub(1));

                bus.read_u8(hl) as u16
            }
            ReadDestination::AddressBC => {
                let bc = bus.cpu.get_bc();

                bus.read_u8(bc) as u16
            }
            ReadDestination::AddressDE => {
                let de = bus.cpu.get_de();

                bus.read_u8(de) as u16
            }
            ReadDestination::AddressHL => {
                let hl = bus.cpu.get_hl();
                bus.read_u8(hl) as u16
            }
            ReadDestination::IoPort => {
                let pc = bus.cpu.get_pc();
                self.opcode_data.post_execution_pc_increment = 1;

                self.port_offset = bus.read_u8(pc) as u16;
                // println!("Loading from IO Port {:02X}", self.port_offset);
                bus.read_u8(0xFF00 + self.port_offset) as u16
            }
            ReadDestination::IoPortC => bus.read_u8(0xFF00 + bus.cpu.get_c() as u16) as u16,
            ReadDestination::Address => {
                let pc = bus.cpu.get_pc();
                self.opcode_data.post_execution_pc_increment = 2;

                self.load_address = bus.read_u16(pc);
                let result = bus.read_u8(self.load_address) as u16;
                // println!(
                //     "Loading from address: 0x{:04X} -- {:04x}",
                //     self.load_address, result
                // );
                result
            }
            ReadDestination::RelativeSP => {
                let pc = bus.cpu.get_pc();
                self.opcode_data.post_execution_pc_increment = 1;

                let offset_u8 = bus.read_u8(pc);
                let offset = offset_u8 as i8;
                let sp = bus.cpu.get_sp();

                let result = sp.wrapping_add_signed(offset.into());

                // Relative SP Reads affects flags
                bus.cpu.set_zero_flag(false);
                bus.cpu.set_n_flag(false);

                // TODO: Verify this, HC and C flags have iffy info online
                bus.cpu
                    .set_half_carry_flag((sp as u8 & 0x0F) + (offset_u8 & 0x0F) > 0x0F);
                bus.cpu
                    .set_carry_flag((sp & 0xFF) + ((offset_u8 & 0xFF) as u16) > 0xFF);

                result
            }
            _ => panic!("Unimplemented Read Destination {:?}", read_destination),
        };

        match self.load_destination {
            LoadDestination::BC => {
                bus.cpu.set_bc(self.read_value);
            }
            LoadDestination::DE => {
                bus.cpu.set_de(self.read_value);
            }
            LoadDestination::HL => {
                bus.cpu.set_hl(self.read_value);
            }
            LoadDestination::SP => {
                // println!("Loading SP: 0x{:04X}", self.read_value);
                bus.cpu.set_sp(self.read_value);
            }
            LoadDestination::A => {
                bus.cpu.set_a(self.read_value as u8);
            }
            LoadDestination::B => {
                bus.cpu.set_b(self.read_value as u8);
            }
            LoadDestination::C => {
                bus.cpu.set_c(self.read_value as u8);
            }
            LoadDestination::D => {
                bus.cpu.set_d(self.read_value as u8);
            }
            LoadDestination::E => {
                bus.cpu.set_e(self.read_value as u8);
            }
            LoadDestination::H => {
                bus.cpu.set_h(self.read_value as u8);
            }
            LoadDestination::L => {
                bus.cpu.set_l(self.read_value as u8);
            }
            LoadDestination::Address => {
                let pc = bus.cpu.get_pc();
                self.opcode_data.post_execution_pc_increment = 2;

                self.load_address = bus.read_u16(pc);

                bus.write_u8(self.load_address, self.read_value as u8);
            }
            LoadDestination::Address16 => {
                let pc = bus.cpu.get_pc();
                self.opcode_data.post_execution_pc_increment = 2;

                self.load_address = bus.read_u16(pc);

                bus.write_u16(self.load_address, self.read_value);
            }
            LoadDestination::IoPort => {
                let pc = bus.cpu.get_pc();
                self.opcode_data.post_execution_pc_increment = 1;

                self.port_offset = bus.read_u8(pc) as u16;
                bus.write_u8(0xFF00 + self.port_offset, self.read_value as u8);
            }
            LoadDestination::IoPortC => {
                bus.write_u8(0xFF00 + bus.cpu.get_c() as u16, self.read_value as u8);
            }
            LoadDestination::AddressBC => {
                let bc = bus.cpu.get_bc();
                bus.write_u8(bc, self.read_value as u8);
            }
            LoadDestination::AddressDE => {
                let de = bus.cpu.get_de();
                bus.write_u8(de, self.read_value as u8);
            }
            LoadDestination::AddressHL => {
                let hl = bus.cpu.get_hl();
                bus.write_u8(hl, self.read_value as u8);
            }
            LoadDestination::HLI => {
                let hl = bus.cpu.get_hl();
                bus.write_u8(hl, self.read_value as u8);

                bus.cpu.set_hl(hl.wrapping_add(1));
            }
            LoadDestination::HLD => {
                let hl = bus.cpu.get_hl();
                bus.write_u8(hl, self.read_value as u8);

                bus.cpu.set_hl(hl.wrapping_sub(1));
            }
            _ => panic!("Unimplemented Load Destination {:?}", self.load_destination),
        }
    }

    // TODO: I'll be real, this is inadaquate
    // Should be determining cycles based on the actual operation
    // But this at least gets things up and running
    fn update_cycles(&self, bus: &mut Bus) {
        let cycles = match self.opcode_data.opcode {
            0x40..=0x45
            | 0x50..=0x55
            | 0x60..=0x65
            | 0x47..=0x4D
            | 0x57..=0x5D
            | 0x67..=0x6D
            | 0x78..=0x7D
            | 0x4F
            | 0x5F
            | 0x6F
            | 0x7F => 1,
            0x02
            | 0x12
            | 0x22
            | 0x32
            | 0x06
            | 0x0E
            | 0x16
            | 0x1E
            | 0x26
            | 0x2E
            | 0x3E
            | 0x0A
            | 0x1A
            | 0x2A
            | 0x3A
            | 0x46
            | 0x56
            | 0x66
            | 0x4E
            | 0x5E
            | 0x6E
            | 0x7E
            | 0x70..=0x77
            | 0xF9
            | 0xE2
            | 0xF2 => 2,
            0x01 | 0x11 | 0x21 | 0x31 | 0x36 | 0xE0 | 0xF0 | 0xF8 => 3,
            0xEA | 0xFA => 4,
            0x08 => 5,
            _ => panic!(
                "Opcode Cycles Unimplemented: 0x{:02X}",
                self.opcode_data.opcode
            ),
        };

        bus.cpu.add_m_cycles(cycles);
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, _bus: &mut Bus) -> () {
        match self.load_destination {
            LoadDestination::IoPort => {
                if self.port_offset >= 0x80 {
                    println!(
                        ":: Loading 0x{:04X} into HRAM 0x{:02X}",
                        self.read_value, self.port_offset
                    );
                } else {
                    println!(
                        ":: Loading 0x{:04X} into IO Port 0x{:02X}",
                        self.read_value, self.port_offset
                    );
                }
            }
            LoadDestination::Address => {
                println!(
                    ":: Loading 0x{:04X} into address 0x{:04X}",
                    self.read_value, self.load_address
                );
            }
            _ => {
                println!(
                    ":: Loading 0x{:04X} into destination: {:?}",
                    self.read_value, self.load_destination
                );
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
enum StackRegisterSource {
    BC,
    DE,
    HL,
    AF,
    Undetermined,
}

impl StackRegisterSource {
    fn from_opcode(value: u8) -> StackRegisterSource {
        match value {
            0xC1 | 0xC5 => StackRegisterSource::BC,
            0xD1 | 0xD5 => StackRegisterSource::DE,
            0xE1 | 0xE5 => StackRegisterSource::HL,
            0xF1 | 0xF5 => StackRegisterSource::AF,
            _ => panic!("Unimplemented Push Source: 0x{:02X}", value),
        }
    }
}

pub struct PushRegister {
    opcode_data: OpcodeData,
    stack_register_source: StackRegisterSource,
    register_value: u16,
}

impl PushRegister {
    pub fn default(opcode: u8) -> PushRegister {
        let mnemonic = format!("Push Register");
        let cycles = 4;

        PushRegister {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            stack_register_source: StackRegisterSource::Undetermined,
            register_value: 0,
        }
    }
}

impl Opcode for PushRegister {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let opcode = self.opcode_data.opcode;

        self.stack_register_source = StackRegisterSource::from_opcode(opcode);

        self.register_value = match self.stack_register_source {
            StackRegisterSource::BC => bus.cpu.get_bc(),
            StackRegisterSource::DE => bus.cpu.get_de(),
            StackRegisterSource::HL => bus.cpu.get_hl(),
            StackRegisterSource::AF => bus.cpu.get_af(),
            _ => panic!("Unimplemented Push Source"),
        };

        // if (self.stack_register_source == StackRegisterSource::BC) {
        //     println!(
        //         "PUSH {:?} {}  -- SP: {:04x} PC: 0x{:04x}",
        //         self.stack_register_source,
        //         self.register_value,
        //         bus.cpu.get_sp(),
        //         bus.cpu.get_previous_pc()
        //     );
        // }

        bus.push_u16_to_stack(self.register_value);
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, bus: &mut Bus) -> () {
        println!(
            ":: Pushing 0x{:04X} onto the stack from {:?}",
            self.register_value, self.stack_register_source
        );
    }
}

// ------------------------------------------------------------------------------------------------

pub struct PopRegister {
    opcode_data: OpcodeData,
    stack_register_source: StackRegisterSource,
    register_value: u16,
}

impl PopRegister {
    pub fn default(opcode: u8) -> PopRegister {
        let mnemonic = format!("Pop Register");
        let cycles = 3;

        PopRegister {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            stack_register_source: StackRegisterSource::Undetermined,
            register_value: 0,
        }
    }
}

impl Opcode for PopRegister {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let opcode = self.opcode_data.opcode;

        self.stack_register_source = StackRegisterSource::from_opcode(opcode);

        self.register_value = bus.pop_u16_from_stack();

        // if (self.stack_register_source == StackRegisterSource::BC) {
        //     println!(
        //         "POP {:?} {}  -- SP: {:04x} PC: 0x{:04x}",
        //         self.stack_register_source,
        //         self.register_value,
        //         bus.cpu.get_sp(),
        //         bus.cpu.get_previous_pc()
        //     );
        // }

        match self.stack_register_source {
            StackRegisterSource::BC => bus.cpu.set_bc(self.register_value),
            StackRegisterSource::DE => bus.cpu.set_de(self.register_value),
            StackRegisterSource::HL => bus.cpu.set_hl(self.register_value),
            StackRegisterSource::AF => bus.cpu.set_af(self.register_value),
            _ => panic!("Unimplemented Pop Source"),
        };
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, bus: &mut Bus) -> () {
        println!(
            ":: Popping 0x{:04X} from the stack into {:?}",
            self.register_value, self.stack_register_source
        );
    }
}
