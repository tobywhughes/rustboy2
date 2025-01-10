pub struct Interrupt {
    pub interrupt_master_enable: bool,
    interrupt_enable: u8, // 0xFFFF
    interrupt_flag: u8,   // 0xFF0F
}

impl Interrupt {
    pub fn default() -> Interrupt {
        Interrupt {
            interrupt_master_enable: true,
            interrupt_enable: 0,
            interrupt_flag: 0,
        }
    }

    pub fn disable_interrupts(&mut self) {
        self.interrupt_master_enable = false;
    }

    pub fn enable_interrupts(&mut self) {
        self.interrupt_master_enable = true;
    }

    pub fn write_u8(&mut self, address: u16, value: u8) {
        match address {
            0xFFFF => self.interrupt_enable = value,
            0xFF0F => self.interrupt_flag = value,
            _ => panic!("Invalid Interrupt Write address: 0x{:04X}", address),
        }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            0xFFFF => self.interrupt_enable,
            0xFF0F => self.interrupt_flag,
            _ => panic!("Invalid Interrupt Read address: 0x{:04X}", address),
        }
    }

    pub fn check_interrupts(&self) -> u8 {
        if self.interrupt_master_enable {
            self.interrupt_enable & self.interrupt_flag
        } else {
            0
        }
    }

    pub fn has_interrupts(&self) -> bool {
        self.interrupt_enable & self.interrupt_flag > 0
    }

    // Interrupt Checks

    pub fn is_timer_interrupt(&self) -> bool {
        (self.interrupt_enable & self.interrupt_flag) & 0x04 != 0
    }

    pub fn is_vblank_interrupt(&self) -> bool {
        (self.interrupt_enable & self.interrupt_flag) & 0x01 != 0
    }

    pub fn is_lcd_interrupt(&self) -> bool {
        (self.interrupt_enable & self.interrupt_flag) & 0x02 != 0
    }

    // Interrupt Setters

    pub fn disable_timer_interrupt(&mut self) {
        self.interrupt_flag &= !0x04;
    }

    pub fn set_timer_interrupt(&mut self) {
        self.interrupt_flag |= 0x04;
    }

    pub fn disable_vblank_interrupt(&mut self) {
        self.interrupt_flag &= !0x01;
    }

    pub fn set_vblank_interrupt(&mut self) {
        self.interrupt_flag |= 0x01;
    }

    pub fn set_lcd_interrupt(&mut self) {
        self.interrupt_flag |= 0x02;
    }

    pub fn disable_lcd_interrupt(&mut self) {
        self.interrupt_flag &= !0x02;
    }
}
