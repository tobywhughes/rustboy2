struct Registers {
    AF: u16,
    BC: u16,
    DE: u16,
    HL: u16,
    SP: u16,
    PC: u16,
}

impl Registers {
    fn default() -> Registers {
        Registers {
            AF: 0x01B0,
            BC: 0x0013,
            DE: 0x00D8,
            HL: 0x014D,
            SP: 0xFFFE,
            PC: 0x0100,
        }
    }
}

pub struct CPU {
    registers: Registers,
    pub cycles: u16, // TODO: Figure out how to handle cycles
    pub cycle_buffer: u8,
    pub is_halted: bool,
}

impl CPU {
    pub fn default() -> CPU {
        CPU {
            registers: Registers::default(),
            cycle_buffer: 0,
            cycles: 0,
            is_halted: false,
        }
    }

    pub fn get_pc_and_increment(&mut self) -> u16 {
        let pc = self.registers.PC;
        self.registers.PC += 1;
        pc
    }

    pub fn get_pc(&self) -> u16 {
        self.registers.PC
    }

    pub fn set_pc(&mut self, value: u16) {
        self.registers.PC = value;
    }

    pub fn add_to_pc(&mut self, value: u16) {
        self.registers.PC += value;
    }

    pub fn add_i8_to_pc(&mut self, value: i8) {
        self.registers.PC = self.registers.PC.wrapping_add_signed(value.into());
    }

    pub fn get_previous_pc(&self) -> u16 {
        self.registers.PC - 1
    }

    pub fn set_sp(&mut self, value: u16) {
        self.registers.SP = value;
    }

    pub fn get_sp(&self) -> u16 {
        self.registers.SP
    }

    pub fn push_sp(&mut self) -> u16 {
        self.registers.SP -= 2;
        self.registers.SP
    }

    pub fn pop_sp(&mut self) -> u16 {
        self.registers.SP += 2;
        self.registers.SP
    }

    // 16 bit register accessors

    pub fn set_bc(&mut self, value: u16) {
        self.registers.BC = value;
    }

    pub fn get_bc(&self) -> u16 {
        self.registers.BC
    }

    pub fn set_de(&mut self, value: u16) {
        self.registers.DE = value;
    }

    pub fn get_de(&self) -> u16 {
        self.registers.DE
    }

    pub fn set_hl(&mut self, value: u16) {
        self.registers.HL = value;
    }

    pub fn get_hl(&self) -> u16 {
        self.registers.HL
    }

    pub fn set_af(&mut self, value: u16) {
        self.registers.AF = value & 0xFFF0;
    }

    pub fn get_af(&self) -> u16 {
        self.registers.AF & 0xFFF0
    }

    // 8 bit register accessors

    pub fn get_a(&self) -> u8 {
        (self.registers.AF >> 8) as u8
    }

    pub fn get_b(&self) -> u8 {
        (self.registers.BC >> 8) as u8
    }

    pub fn get_c(&self) -> u8 {
        (self.registers.BC & 0x00FF) as u8
    }

    pub fn get_d(&self) -> u8 {
        (self.registers.DE >> 8) as u8
    }

    pub fn get_e(&self) -> u8 {
        (self.registers.DE & 0x00FF) as u8
    }

    pub fn set_a(&mut self, value: u8) {
        self.registers.AF = (self.registers.AF & 0x00FF) | ((value as u16) << 8);
    }

    pub fn set_b(&mut self, value: u8) {
        self.registers.BC = (self.registers.BC & 0x00FF) | ((value as u16) << 8);
    }

    pub fn set_c(&mut self, value: u8) {
        self.registers.BC = (self.registers.BC & 0xFF00) | (value as u16);
    }

    pub fn set_d(&mut self, value: u8) {
        self.registers.DE = (self.registers.DE & 0x00FF) | ((value as u16) << 8);
    }

    pub fn set_e(&mut self, value: u8) {
        self.registers.DE = (self.registers.DE & 0xFF00) | (value as u16);
    }

    pub fn get_h(&self) -> u8 {
        (self.registers.HL >> 8) as u8
    }

    pub fn set_h(&mut self, value: u8) {
        self.registers.HL = (self.registers.HL & 0x00FF) | ((value as u16) << 8);
    }

    pub fn get_l(&self) -> u8 {
        (self.registers.HL & 0x00FF) as u8
    }

    pub fn set_l(&mut self, value: u8) {
        self.registers.HL = (self.registers.HL & 0xFF00) | (value as u16);
    }

    pub fn get_f(&self) -> u8 {
        (self.registers.AF & 0x00FF) as u8
    }
    // Flags

    pub fn set_zero_flag(&mut self, value: bool) {
        if value {
            self.registers.AF |= 0x0080;
        } else {
            self.registers.AF &= 0xFF7F;
        }
    }

    pub fn set_n_flag(&mut self, value: bool) {
        if value {
            self.registers.AF |= 0x0040;
        } else {
            self.registers.AF &= 0xFFBF;
        }
    }

    pub fn set_half_carry_flag(&mut self, value: bool) {
        if value {
            self.registers.AF |= 0x0020;
        } else {
            self.registers.AF &= 0xFFDF;
        }
    }

    pub fn set_carry_flag(&mut self, value: bool) {
        if value {
            self.registers.AF |= 0x0010;
        } else {
            self.registers.AF &= 0xFFEF;
        }
    }

    pub fn get_zero_flag(&self) -> bool {
        (self.registers.AF & 0x0080) != 0
    }

    pub fn get_n_flag(&self) -> bool {
        (self.registers.AF & 0x0040) != 0
    }

    pub fn get_half_carry_flag(&self) -> bool {
        (self.registers.AF & 0x0020) != 0
    }

    pub fn get_carry_flag(&self) -> bool {
        (self.registers.AF & 0x0010) != 0
    }

    // -------------------------

    pub fn add_m_cycles(&mut self, cycles: u16) {
        self.cycle_buffer = cycles as u8;
        // self.cycles = self.cycles.wrapping_add(cycles);
    }

    pub fn halt(&mut self) {
        self.is_halted = true;
    }

    pub fn resume(&mut self) {
        self.is_halted = false;
    }
}
