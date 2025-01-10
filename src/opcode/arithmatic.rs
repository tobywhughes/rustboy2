use crate::bus::Bus;

use super::opcode::{Opcode, OpcodeData};

#[derive(Debug)]
enum SourceRegister16Bit {
    BC,
    DE,
    HL,
    SP,
    Undetermined,
}

impl SourceRegister16Bit {
    fn from_opcode(value: u8) -> SourceRegister16Bit {
        match value {
            0x03 | 0x09 | 0x0B => SourceRegister16Bit::BC,
            0x13 | 0x19 | 0x1B => SourceRegister16Bit::DE,
            0x23 | 0x29 | 0x2B => SourceRegister16Bit::HL,
            0x33 | 0x39 | 0x3B => SourceRegister16Bit::SP,
            _ => panic!("Unimplemented 16bit Source: 0x{:02X}", value),
        }
    }
}

#[derive(Debug, PartialEq)]
enum DestinationRegister8Bit {
    B,
    C,
    D,
    E,
    H,
    L,
    A,
    AddressedHL,
    Undetermined,
}

impl DestinationRegister8Bit {
    fn from_opcode(value: u8) -> DestinationRegister8Bit {
        match value {
            0x04 | 0x5 => DestinationRegister8Bit::B,
            0x0C | 0x0D => DestinationRegister8Bit::C,
            0x14 | 0x15 => DestinationRegister8Bit::D,
            0x1C | 0x1D => DestinationRegister8Bit::E,
            0x24 | 0x25 => DestinationRegister8Bit::H,
            0x2C | 0x2D => DestinationRegister8Bit::L,
            0x3C | 0x3D => DestinationRegister8Bit::A,
            0x34 | 0x35 => DestinationRegister8Bit::AddressedHL,
            _ => panic!("Unimplemented Destination Source: 0x{:02X}", value),
        }
    }

    fn get_value_from_register(destination_register: &DestinationRegister8Bit, bus: &Bus) -> u8 {
        match destination_register {
            DestinationRegister8Bit::B => bus.cpu.get_b(),
            DestinationRegister8Bit::C => bus.cpu.get_c(),
            DestinationRegister8Bit::D => bus.cpu.get_d(),
            DestinationRegister8Bit::E => bus.cpu.get_e(),
            DestinationRegister8Bit::H => bus.cpu.get_h(),
            DestinationRegister8Bit::L => bus.cpu.get_l(),
            DestinationRegister8Bit::A => bus.cpu.get_a(),
            DestinationRegister8Bit::AddressedHL => {
                let address = bus.cpu.get_hl();
                bus.read_u8(address)
            }
            _ => panic!("Unimplemented Destination Register"),
        }
    }

    fn set_value_to_register(
        destination_register: &DestinationRegister8Bit,
        bus: &mut Bus,
        value: u8,
    ) -> () {
        match destination_register {
            DestinationRegister8Bit::B => bus.cpu.set_b(value),
            DestinationRegister8Bit::C => bus.cpu.set_c(value),
            DestinationRegister8Bit::D => bus.cpu.set_d(value),
            DestinationRegister8Bit::E => bus.cpu.set_e(value),
            DestinationRegister8Bit::H => bus.cpu.set_h(value),
            DestinationRegister8Bit::L => bus.cpu.set_l(value),
            DestinationRegister8Bit::A => bus.cpu.set_a(value),
            DestinationRegister8Bit::AddressedHL => {
                let address = bus.cpu.get_hl();
                bus.write_u8(address, value);
            }
            _ => panic!("Unimplemented Destination Register"),
        }
    }
}

// ------------------------------------------------------------------------------------------------
pub struct IncrementRegister {
    opcode_data: OpcodeData,
    source_register: SourceRegister16Bit,
}

impl IncrementRegister {
    pub fn default(opcode: u8) -> IncrementRegister {
        let mnemonic = format!("Increment Register");
        let cycles = 2;

        IncrementRegister {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            source_register: SourceRegister16Bit::Undetermined,
        }
    }
}

impl Opcode for IncrementRegister {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let opcode = self.opcode_data.opcode;

        self.source_register = SourceRegister16Bit::from_opcode(opcode);

        let value = match self.source_register {
            SourceRegister16Bit::BC => bus.cpu.get_bc(),
            SourceRegister16Bit::DE => bus.cpu.get_de(),
            SourceRegister16Bit::HL => bus.cpu.get_hl(),
            SourceRegister16Bit::SP => bus.cpu.get_sp(),
            _ => panic!("Unimplemented Destination Register"),
        };

        let incremented_value = value.wrapping_add(1);

        match self.source_register {
            SourceRegister16Bit::BC => bus.cpu.set_bc(incremented_value),
            SourceRegister16Bit::DE => bus.cpu.set_de(incremented_value),
            SourceRegister16Bit::HL => bus.cpu.set_hl(incremented_value),
            SourceRegister16Bit::SP => bus.cpu.set_sp(incremented_value),
            _ => panic!("Unimplemented Destination Register"),
        }
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, _bus: &mut Bus) -> () {
        println!(":: Incrementing Register: {:?}", self.source_register);
    }
}

// ------------------------------------------------------------------------------------------------

pub struct DecrementRegister {
    opcode_data: OpcodeData,
    source_register: SourceRegister16Bit,
}

impl DecrementRegister {
    pub fn default(opcode: u8) -> DecrementRegister {
        let mnemonic = format!("Decrement Register");
        let cycles = 2;

        DecrementRegister {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            source_register: SourceRegister16Bit::Undetermined,
        }
    }
}

impl Opcode for DecrementRegister {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let opcode = self.opcode_data.opcode;

        self.source_register = SourceRegister16Bit::from_opcode(opcode);

        let value = match self.source_register {
            SourceRegister16Bit::BC => bus.cpu.get_bc(),
            SourceRegister16Bit::DE => bus.cpu.get_de(),
            SourceRegister16Bit::HL => bus.cpu.get_hl(),
            SourceRegister16Bit::SP => bus.cpu.get_sp(),
            _ => panic!("Unimplemented Destination Register"),
        };

        let decremented_value = value.wrapping_sub(1);

        match self.source_register {
            SourceRegister16Bit::BC => bus.cpu.set_bc(decremented_value),
            SourceRegister16Bit::DE => bus.cpu.set_de(decremented_value),
            SourceRegister16Bit::HL => bus.cpu.set_hl(decremented_value),
            SourceRegister16Bit::SP => bus.cpu.set_sp(decremented_value),
            _ => panic!("Unimplemented Destination Register"),
        }
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, _bus: &mut Bus) -> () {
        println!(":: Decrementing Register: {:?}", self.source_register);
    }
}

// ------------------------------------------------------------------------------------------------

pub struct Add16BitRegister {
    opcode_data: OpcodeData,
    source_register: SourceRegister16Bit,
}

impl Add16BitRegister {
    pub fn default(opcode: u8) -> Add16BitRegister {
        let mnemonic = format!("Add 16-bit Register");
        let cycles = 2;

        Add16BitRegister {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            source_register: SourceRegister16Bit::from_opcode(opcode),
        }
    }
}

impl Opcode for Add16BitRegister {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let value = match self.source_register {
            SourceRegister16Bit::BC => bus.cpu.get_bc(),
            SourceRegister16Bit::DE => bus.cpu.get_de(),
            SourceRegister16Bit::HL => bus.cpu.get_hl(),
            SourceRegister16Bit::SP => bus.cpu.get_sp(),
            _ => panic!("Unimplemented Destination Register"),
        };

        let hl = bus.cpu.get_hl();
        let result = hl.wrapping_add(value);

        bus.cpu.set_hl(result);

        bus.cpu.set_n_flag(false);
        bus.cpu
            .set_half_carry_flag((hl & 0x0FFF) + (value & 0x0FFF) > 0x0FFF);
        bus.cpu.set_carry_flag(result < hl);
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, _bus: &mut Bus) -> () {
        println!(":: Adding 16-bit Register: {:?}", self.source_register);
    }
}

// ------------------------------------------------------------------------------------------------

pub struct IncrementRegister8Bit {
    opcode_data: OpcodeData,
    destination_register: DestinationRegister8Bit,
}

impl IncrementRegister8Bit {
    pub fn default(opcode: u8) -> IncrementRegister8Bit {
        let mnemonic = format!("Increment 8-bit Register");
        let cycles = 1;

        IncrementRegister8Bit {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            destination_register: DestinationRegister8Bit::Undetermined,
        }
    }
}

impl Opcode for IncrementRegister8Bit {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let opcode = self.opcode_data.opcode;

        self.destination_register = DestinationRegister8Bit::from_opcode(opcode);

        if self.destination_register == DestinationRegister8Bit::AddressedHL {
            self.opcode_data.cycles = 3;
        }

        let value =
            DestinationRegister8Bit::get_value_from_register(&self.destination_register, &bus);

        let incremented_value = value.wrapping_add(1);

        DestinationRegister8Bit::set_value_to_register(
            &self.destination_register,
            bus,
            incremented_value,
        );

        bus.cpu.set_zero_flag(incremented_value == 0);
        bus.cpu.set_n_flag(false);
        bus.cpu.set_half_carry_flag((value & 0x0F) == 0x0F);
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, _bus: &mut Bus) -> () {
        println!(
            ":: Incrementing 8-bit Register: {:?}",
            self.destination_register
        );
    }
}

// ------------------------------------------------------------------------------------------------

pub struct DecrementRegister8Bit {
    opcode_data: OpcodeData,
    destination_register: DestinationRegister8Bit,
    decremented_value: u8,
}

impl DecrementRegister8Bit {
    pub fn default(opcode: u8) -> DecrementRegister8Bit {
        let mnemonic = format!("Decrement 8-bit Register");
        let cycles = 1;

        DecrementRegister8Bit {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            destination_register: DestinationRegister8Bit::Undetermined,
            decremented_value: 0,
        }
    }
}

impl Opcode for DecrementRegister8Bit {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let opcode = self.opcode_data.opcode;

        self.destination_register = DestinationRegister8Bit::from_opcode(opcode);

        if self.destination_register == DestinationRegister8Bit::AddressedHL {
            self.opcode_data.cycles = 3;
        }

        let value =
            DestinationRegister8Bit::get_value_from_register(&self.destination_register, &bus);

        self.decremented_value = value.wrapping_sub(1);

        DestinationRegister8Bit::set_value_to_register(
            &self.destination_register,
            bus,
            self.decremented_value,
        );

        bus.cpu.set_zero_flag(self.decremented_value == 0);
        bus.cpu.set_n_flag(true);
        bus.cpu.set_half_carry_flag((value & 0x0F) == 0x00);
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, _bus: &mut Bus) -> () {
        println!(
            ":: Decrementing 8-bit Register: {:?} - Result 0x{:02X}",
            self.destination_register, self.decremented_value
        );
    }
}

// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
enum SourceRegister8Bit {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    Immediate,
    AddressedHL,
    Undetermined,
}

impl SourceRegister8Bit {
    fn from_opcode(value: u8) -> SourceRegister8Bit {
        let masked_value = value & 0x07;
        let upper_nibble_masked = value & 0xF0;

        match masked_value {
            0x00 => SourceRegister8Bit::B,
            0x01 => SourceRegister8Bit::C,
            0x02 => SourceRegister8Bit::D,
            0x03 => SourceRegister8Bit::E,
            0x04 => SourceRegister8Bit::H,
            0x05 => SourceRegister8Bit::L,
            0x07 => SourceRegister8Bit::A,
            0x06 => match upper_nibble_masked {
                0x80 | 0x90 | 0xA0 | 0xB0 => SourceRegister8Bit::AddressedHL,
                0xC0 | 0xD0 | 0xE0 | 0xF0 => SourceRegister8Bit::Immediate,
                _ => panic!("Unimplemented Source Register: 0x{:02X}", value),
            },
            _ => panic!("Unimplemented Source Register: 0x{:02X}", value),
        }
    }

    fn get_from_register(bus: &mut Bus, register: &SourceRegister8Bit) -> u8 {
        match register {
            SourceRegister8Bit::A => bus.cpu.get_a(),
            SourceRegister8Bit::B => bus.cpu.get_b(),
            SourceRegister8Bit::C => bus.cpu.get_c(),
            SourceRegister8Bit::D => bus.cpu.get_d(),
            SourceRegister8Bit::E => bus.cpu.get_e(),
            SourceRegister8Bit::H => bus.cpu.get_h(),
            SourceRegister8Bit::L => bus.cpu.get_l(),
            SourceRegister8Bit::Immediate => {
                let pc = bus.cpu.get_pc_and_increment();
                bus.read_u8(pc)
            }
            SourceRegister8Bit::AddressedHL => {
                let address = bus.cpu.get_hl();

                bus.read_u8(address)
            }
            _ => panic!("Unimplemented Source Register"),
        }
    }
}

// ------------------------------------------------------------------------------------------------

pub struct Or8BitRegister {
    opcode_data: OpcodeData,
    compare_register: SourceRegister8Bit,
}

impl Or8BitRegister {
    pub fn default(opcode: u8) -> Or8BitRegister {
        let mnemonic = format!("Or 8-bit Register");
        let cycles = 1;

        Or8BitRegister {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            compare_register: SourceRegister8Bit::Undetermined,
        }
    }
}

impl Opcode for Or8BitRegister {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let opcode = self.opcode_data.opcode;

        self.compare_register = SourceRegister8Bit::from_opcode(opcode);

        let value = SourceRegister8Bit::get_from_register(bus, &self.compare_register);

        match self.compare_register {
            SourceRegister8Bit::Immediate | SourceRegister8Bit::AddressedHL => {
                self.opcode_data.cycles = 2
            }
            _ => (),
        }

        let a = bus.cpu.get_a();
        let result = a | value;

        bus.cpu.set_a(result);
    }

    fn update_flags(&self, bus: &mut Bus) -> () {
        let value = bus.cpu.get_a();

        bus.cpu.set_zero_flag(value == 0);
        bus.cpu.set_n_flag(false);
        bus.cpu.set_half_carry_flag(false);
        bus.cpu.set_carry_flag(false);
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, bus: &mut Bus) -> () {
        println!(
            ":: Or Register: {:?} - Result 0x{:02X} - Flags 0b{:08b}",
            self.compare_register,
            bus.cpu.get_a(),
            bus.cpu.get_f()
        );
    }
}

// ------------------------------------------------------------------------------------------------

pub struct Compare8BitRegister {
    opcode_data: OpcodeData,
    compare_register: SourceRegister8Bit,
}

impl Compare8BitRegister {
    pub fn default(opcode: u8) -> Compare8BitRegister {
        let mnemonic = format!("Compare 8-bit Register");
        let cycles = 1;

        Compare8BitRegister {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            compare_register: SourceRegister8Bit::Undetermined,
        }
    }
}

impl Opcode for Compare8BitRegister {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let opcode = self.opcode_data.opcode;

        self.compare_register = SourceRegister8Bit::from_opcode(opcode);

        let value = SourceRegister8Bit::get_from_register(bus, &self.compare_register);

        match self.compare_register {
            SourceRegister8Bit::Immediate | SourceRegister8Bit::AddressedHL => {
                self.opcode_data.cycles = 2
            }
            _ => (),
        }

        let a = bus.cpu.get_a();
        let result = a.wrapping_sub(value);

        bus.cpu.set_zero_flag(result == 0);
        bus.cpu.set_n_flag(true);
        bus.cpu.set_half_carry_flag((a & 0x0F) < (value & 0x0F));
        bus.cpu.set_carry_flag(a < value);
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, bus: &mut Bus) -> () {
        println!(
            ":: Compare Register: {:?} - Flags 0b{:08b}",
            self.compare_register,
            bus.cpu.get_f()
        );
    }
}

// ------------------------------------------------------------------------------------------------

pub struct And8BitRegister {
    opcode_data: OpcodeData,
    compare_register: SourceRegister8Bit,
}

impl And8BitRegister {
    pub fn default(opcode: u8) -> And8BitRegister {
        let mnemonic = format!("And 8-bit Register");
        let cycles = 1;

        And8BitRegister {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            compare_register: SourceRegister8Bit::Undetermined,
        }
    }
}

impl Opcode for And8BitRegister {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let opcode = self.opcode_data.opcode;

        self.compare_register = SourceRegister8Bit::from_opcode(opcode);

        let value = SourceRegister8Bit::get_from_register(bus, &self.compare_register);

        match self.compare_register {
            SourceRegister8Bit::Immediate | SourceRegister8Bit::AddressedHL => {
                self.opcode_data.cycles = 2
            }
            _ => (),
        }

        let a = bus.cpu.get_a();
        let result = a & value;

        bus.cpu.set_a(result);
    }

    fn update_flags(&self, bus: &mut Bus) -> () {
        let value = bus.cpu.get_a();

        bus.cpu.set_zero_flag(value == 0);
        bus.cpu.set_n_flag(false);
        bus.cpu.set_half_carry_flag(true);
        bus.cpu.set_carry_flag(false);
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, bus: &mut Bus) -> () {
        println!(
            ":: And Register: {:?} - Result 0x{:02X} - Flags 0b{:08b}",
            self.compare_register,
            bus.cpu.get_a(),
            bus.cpu.get_f()
        );
    }
}

// ------------------------------------------------------------------------------------------------

fn subtract8_bit_register_operation(
    opcode: u8,
    cycles: &mut u16,
    compare_register: &mut SourceRegister8Bit,
    bus: &mut Bus,
    with_carry: bool,
) {
    *compare_register = SourceRegister8Bit::from_opcode(opcode);

    let value = SourceRegister8Bit::get_from_register(bus, &compare_register);

    match compare_register {
        SourceRegister8Bit::Immediate | SourceRegister8Bit::AddressedHL => *cycles = 2,
        _ => (),
    }

    let a: u8 = bus.cpu.get_a();

    let result = if with_carry {
        let carry = if bus.cpu.get_carry_flag() { 1 } else { 0 };

        bus.cpu
            .set_half_carry_flag((a & 0x0F) < ((value & 0x0F) + carry));
        bus.cpu
            .set_carry_flag((a as u16) < (value as u16 + carry as u16));

        a.wrapping_sub(value).wrapping_sub(carry)
    } else {
        bus.cpu.set_half_carry_flag((a & 0x0F) < (value & 0x0F));
        bus.cpu.set_carry_flag(a < value);
        a.wrapping_sub(value)
    };

    bus.cpu.set_a(result);

    bus.cpu.set_zero_flag(result == 0);
    bus.cpu.set_n_flag(true);
}

pub struct Subtract8BitRegisterWithCarry {
    opcode_data: OpcodeData,
    compare_register: SourceRegister8Bit,
}

impl Subtract8BitRegisterWithCarry {
    pub fn default(opcode: u8) -> Subtract8BitRegisterWithCarry {
        let mnemonic = format!("Subtract 8-bit Register with Carry");
        let cycles = 1;

        Subtract8BitRegisterWithCarry {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            compare_register: SourceRegister8Bit::Undetermined,
        }
    }
}

impl Opcode for Subtract8BitRegisterWithCarry {
    fn execute(&mut self, bus: &mut Bus) -> () {
        subtract8_bit_register_operation(
            self.opcode_data.opcode,
            &mut self.opcode_data.cycles,
            &mut self.compare_register,
            bus,
            true,
        )
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, bus: &mut Bus) -> () {
        println!(
            ":: Subtract Register With Carry: {:?} - Flags 0b{:08b}",
            self.compare_register,
            bus.cpu.get_f()
        );
    }
}

pub struct Subtract8BitRegister {
    opcode_data: OpcodeData,
    compare_register: SourceRegister8Bit,
}

impl Subtract8BitRegister {
    pub fn default(opcode: u8) -> Subtract8BitRegister {
        let mnemonic = format!("Subtract 8-bit Register");
        let cycles = 1;

        Subtract8BitRegister {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            compare_register: SourceRegister8Bit::Undetermined,
        }
    }
}

impl Opcode for Subtract8BitRegister {
    fn execute(&mut self, bus: &mut Bus) -> () {
        subtract8_bit_register_operation(
            self.opcode_data.opcode,
            &mut self.opcode_data.cycles,
            &mut self.compare_register,
            bus,
            false,
        )
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, bus: &mut Bus) -> () {
        println!(
            ":: Subtract Register: {:?} - Flags 0b{:08b}",
            self.compare_register,
            bus.cpu.get_f()
        );
    }
}

// ------------------------------------------------------------------------------------------------

pub struct Xor8BitRegister {
    opcode_data: OpcodeData,
    compare_register: SourceRegister8Bit,
}

impl Xor8BitRegister {
    pub fn default(opcode: u8) -> Xor8BitRegister {
        let mnemonic = format!("Xor 8-bit Register");
        let cycles = 1;

        Xor8BitRegister {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            compare_register: SourceRegister8Bit::Undetermined,
        }
    }
}

impl Opcode for Xor8BitRegister {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let opcode = self.opcode_data.opcode;

        self.compare_register = SourceRegister8Bit::from_opcode(opcode);

        let value = SourceRegister8Bit::get_from_register(bus, &self.compare_register);

        match self.compare_register {
            SourceRegister8Bit::Immediate | SourceRegister8Bit::AddressedHL => {
                self.opcode_data.cycles = 2
            }
            _ => (),
        }

        let a = bus.cpu.get_a();
        let result = a ^ value;

        bus.cpu.set_a(result);

        bus.cpu.set_zero_flag(result == 0);
        bus.cpu.set_n_flag(false);
        bus.cpu.set_half_carry_flag(false);
        bus.cpu.set_carry_flag(false);
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, bus: &mut Bus) -> () {
        println!(
            ":: Xor Register: {:?} - Result 0x{:02X} - Flags 0b{:08b}",
            self.compare_register,
            bus.cpu.get_a(),
            bus.cpu.get_f()
        );
    }
}

// ------------------------------------------------------------------------------------------------

fn add_8_bit_register_operation(
    opcode: u8,
    cycles: &mut u16,
    compare_register: &mut SourceRegister8Bit,
    bus: &mut Bus,
    with_carry: bool,
) {
    *compare_register = SourceRegister8Bit::from_opcode(opcode);

    let value = SourceRegister8Bit::get_from_register(bus, &compare_register);

    match compare_register {
        SourceRegister8Bit::Immediate | SourceRegister8Bit::AddressedHL => *cycles = 2,
        _ => (),
    }

    let a: u8 = bus.cpu.get_a();

    let result = if with_carry {
        let carry = bus.cpu.get_carry_flag() as u8;

        bus.cpu
            .set_half_carry_flag((a & 0x0F) + (value & 0x0F) + carry > 0x0F);
        bus.cpu
            .set_carry_flag(a as u16 + value as u16 + carry as u16 > 0xFF);

        a.wrapping_add(value).wrapping_add(carry)
    } else {
        bus.cpu
            .set_half_carry_flag((a & 0x0F) + (value & 0x0F) > 0x0F);
        bus.cpu.set_carry_flag(a as u16 + value as u16 > 0xFF);

        a.wrapping_add(value)
    };

    bus.cpu.set_a(result);

    bus.cpu.set_zero_flag(result == 0);
    bus.cpu.set_n_flag(false);
}
pub struct Add8BitRegister {
    opcode_data: OpcodeData,
    compare_register: SourceRegister8Bit,
}

impl Add8BitRegister {
    pub fn default(opcode: u8) -> Add8BitRegister {
        let mnemonic = format!("Add 8-bit Register");
        let cycles = 1;

        Add8BitRegister {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            compare_register: SourceRegister8Bit::Undetermined,
        }
    }
}

impl Opcode for Add8BitRegister {
    fn execute(&mut self, bus: &mut Bus) -> () {
        add_8_bit_register_operation(
            self.opcode_data.opcode,
            &mut self.opcode_data.cycles,
            &mut self.compare_register,
            bus,
            false,
        );
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, bus: &mut Bus) -> () {
        println!(
            ":: Add Register: {:?} - Result 0x{:02X} - Flags 0b{:08b}",
            self.compare_register,
            bus.cpu.get_a(),
            bus.cpu.get_f()
        );
    }
}

pub struct Add8BitRegisterWithCarry {
    opcode_data: OpcodeData,
    compare_register: SourceRegister8Bit,
}

impl Add8BitRegisterWithCarry {
    pub fn default(opcode: u8) -> Add8BitRegisterWithCarry {
        let mnemonic = format!("Add 8-bit Register With Carry");
        let cycles = 1;

        Add8BitRegisterWithCarry {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            compare_register: SourceRegister8Bit::Undetermined,
        }
    }
}

impl Opcode for Add8BitRegisterWithCarry {
    fn execute(&mut self, bus: &mut Bus) -> () {
        add_8_bit_register_operation(
            self.opcode_data.opcode,
            &mut self.opcode_data.cycles,
            &mut self.compare_register,
            bus,
            true,
        );
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, bus: &mut Bus) -> () {
        println!(
            ":: Add Register With Carry: {:?} - Result 0x{:02X} - Flags 0b{:08b}",
            self.compare_register,
            bus.cpu.get_a(),
            bus.cpu.get_f()
        );
    }
}

// ------------------------------------------------------------------------------------------------

// TODO: Implement rotate commands

#[derive(Debug)]
enum RotateCommand {
    RotateLeft,
    RotateRight,
    RotateLeftCarry,
    RotateRightCarry,
}

impl RotateCommand {
    fn from_opcode(value: u8) -> RotateCommand {
        match value {
            0x07 => RotateCommand::RotateLeft,
            0x0F => RotateCommand::RotateRight,
            0x17 => RotateCommand::RotateLeftCarry,
            0x1F => RotateCommand::RotateRightCarry,
            _ => panic!("Unimplemented Rotate Command: 0x{:02X}", value),
        }
    }
}

pub struct RotateAccumulator {
    opcode_data: OpcodeData,
    rotate_command: RotateCommand,
}

impl RotateAccumulator {
    pub fn default(opcode: u8) -> RotateAccumulator {
        let mnemonic = format!("Rotate Accumulator");
        let cycles = 1;

        RotateAccumulator {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
            rotate_command: RotateCommand::from_opcode(opcode),
        }
    }
}

impl Opcode for RotateAccumulator {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let a = bus.cpu.get_a();

        let mut is_carry = false;

        let result: u8 = match self.rotate_command {
            RotateCommand::RotateLeft => {
                is_carry = a & 0x80 == 0x80;

                a.rotate_left(1)
            }
            RotateCommand::RotateRight => {
                is_carry = a & 0x01 == 0x01;

                a.rotate_right(1)
            }
            RotateCommand::RotateLeftCarry => {
                let carry = bus.cpu.get_carry_flag() as u8;

                is_carry = a & 0x80 == 0x80;

                (a << 1) | carry
            }
            RotateCommand::RotateRightCarry => {
                let carry = bus.cpu.get_carry_flag() as u8;

                is_carry = a & 0x01 == 0x01;

                (a >> 1) | (carry << 7)
            }
        };

        bus.cpu.set_a(result);

        bus.cpu.set_zero_flag(false);
        bus.cpu.set_n_flag(false);
        bus.cpu.set_half_carry_flag(false);
        bus.cpu.set_carry_flag(is_carry);
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, bus: &mut Bus) -> () {
        println!(
            ":: Rotate Accumulator: {:?} - Result 0x{:02X} - Flags 0b{:08b}",
            self.rotate_command,
            bus.cpu.get_a(),
            bus.cpu.get_f()
        );
    }
}

// ------------------------------------------------------------------------------------------------

pub struct DecimalAdjustAccumulator {
    opcode_data: OpcodeData,
}

impl DecimalAdjustAccumulator {
    pub fn default(opcode: u8) -> DecimalAdjustAccumulator {
        let mnemonic = format!("Decimal Adjust Accumulator");
        let cycles = 1;

        DecimalAdjustAccumulator {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
        }
    }
}

// static mut total_carries: u32 = 0;
// static mut total_half_carries: u32 = 0;

// Confusing lol
impl Opcode for DecimalAdjustAccumulator {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let mut a = bus.cpu.get_a();

        let is_addition = !bus.cpu.get_n_flag();
        let incoming_carry = bus.cpu.get_carry_flag();
        let incoming_half_carry = bus.cpu.get_half_carry_flag();
        // unsafe {
        //     total_carries += incoming_carry as u32;
        //     total_half_carries += incoming_half_carry as u32;
        // }

        // Addition
        if is_addition {
            if incoming_carry || a > 0x99 {
                a = a.wrapping_add(0x60);
                bus.cpu.set_carry_flag(true);
            }
            if incoming_half_carry || (a & 0xF) > 0x09 {
                a = a.wrapping_add(0x06);
            }
        } else {
            if incoming_carry {
                a = a.wrapping_sub(0x60);
                // bus.cpu.set_carry_flag(true);
            }
            if incoming_half_carry {
                a = a.wrapping_sub(0x06);
            }
        }

        bus.cpu.set_a(a);

        bus.cpu.set_zero_flag(a == 0);
        bus.cpu.set_half_carry_flag(false);

        // unsafe {
        //     println!("{}{}", total_carries, total_half_carries);
        // }
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, bus: &mut Bus) -> () {
        println!(
            ":: Decimal Adjust Accumulator - Result 0x{:02X} - Flags 0b{:08b}",
            bus.cpu.get_a(),
            bus.cpu.get_f()
        );
    }
}

// ------------------------------------------------------------------------------------------------

pub struct ComplementAccumulator {
    opcode_data: OpcodeData,
}

impl ComplementAccumulator {
    pub fn default(opcode: u8) -> ComplementAccumulator {
        let mnemonic = format!("Complement Accumulator");
        let cycles = 1;

        ComplementAccumulator {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
        }
    }
}

impl Opcode for ComplementAccumulator {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let a = bus.cpu.get_a();

        bus.cpu.set_a(!a);

        bus.cpu.set_n_flag(true);
        bus.cpu.set_half_carry_flag(true);
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, _bus: &mut Bus) -> () {
        println!(":: Complement Accumulator");
    }
}

// ------------------------------------------------------------------------------------------------

pub struct SetCarryFlag {
    opcode_data: OpcodeData,
}

impl SetCarryFlag {
    pub fn default(opcode: u8) -> SetCarryFlag {
        let mnemonic = format!("Set Carry Flag");
        let cycles = 1;

        SetCarryFlag {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
        }
    }
}

impl Opcode for SetCarryFlag {
    fn execute(&mut self, bus: &mut Bus) -> () {
        bus.cpu.set_carry_flag(true);
        bus.cpu.set_n_flag(false);
        bus.cpu.set_half_carry_flag(false);
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, _bus: &mut Bus) -> () {
        println!(":: Set Carry Flag");
    }
}

pub struct ComplementCarryFlag {
    opcode_data: OpcodeData,
}

impl ComplementCarryFlag {
    pub fn default(opcode: u8) -> ComplementCarryFlag {
        let mnemonic = format!("Complement Carry Flag");
        let cycles = 1;

        ComplementCarryFlag {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
        }
    }
}

impl Opcode for ComplementCarryFlag {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let carry = bus.cpu.get_carry_flag();
        bus.cpu.set_carry_flag(!carry);
        bus.cpu.set_n_flag(false);
        bus.cpu.set_half_carry_flag(false);
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, _bus: &mut Bus) -> () {
        println!(":: Complement Carry Flag");
    }
}

//

pub struct AddSignedImmediateToSP {
    opcode_data: OpcodeData,
}

impl AddSignedImmediateToSP {
    pub fn default(opcode: u8) -> AddSignedImmediateToSP {
        let mnemonic = format!("Add Signed Immediate to SP");
        let cycles = 4;

        AddSignedImmediateToSP {
            opcode_data: OpcodeData::new(opcode, cycles, mnemonic),
        }
    }
}

impl Opcode for AddSignedImmediateToSP {
    fn execute(&mut self, bus: &mut Bus) -> () {
        let value: u8 = bus.read_u8(bus.cpu.get_pc());
        let value_signed = value as i8;

        self.opcode_data.post_execution_pc_increment = 1;

        let sp = bus.cpu.get_sp();

        let result = sp.wrapping_add_signed(value_signed.into());

        bus.cpu.set_sp(result);

        bus.cpu.set_zero_flag(false);
        bus.cpu.set_n_flag(false);
        bus.cpu
            .set_half_carry_flag((sp & 0x0F) + (value as u16 & 0x0F) > 0x0F);
        bus.cpu
            .set_carry_flag((sp & 0xFF) + (value as u16 & 0xFF) > 0xFF);
    }

    fn opcode_data(&self) -> &OpcodeData {
        &self.opcode_data
    }

    fn post_execute_debug_print(&self, _bus: &mut Bus) -> () {
        println!(":: Add Signed Immediate to SP");
    }
}
