use super::interrupts::Interrupt;

pub struct Timer {
    m_cycles: u16,
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,
}

impl Timer {
    pub fn default() -> Timer {
        Timer {
            m_cycles: 0,
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
        }
    }

    pub fn write_u8(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.m_cycles = 0,
            0xFF05 => self.tima = value,
            0xFF06 => self.tma = value,
            0xFF07 => self.tac = value,
            _ => panic!("Invalid Timer Write address: 0x{:04X}", address),
        }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            0xFF04 => (self.m_cycles >> 6) as u8,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => panic!("Invalid Timer Read address: 0x{:04X}", address),
        }
    }

    pub fn update_timer(&mut self, cycle_buffer: &mut u8, interrupt: &mut Interrupt) {
        while *cycle_buffer > 0 {
            let prev_m_cycles = self.m_cycles;
            self.m_cycles = self.m_cycles.wrapping_add(1);

            let tac_enabled = self.tac & 0x04 != 0;
            let tac_freq = self.tac & 0x03;

            let tac_mask = match tac_freq {
                0 => 0x80,
                1 => 0x02,
                2 => 0x08,
                3 => 0x20,
                _ => panic!("Invalid TAC Frequency: {:02X}", tac_freq),
            };
            let prev_masked_cycles = prev_m_cycles & tac_mask;
            let masked_cycles = self.m_cycles & tac_mask;

            // Falling edge for enabled TAC
            // TODO: CBG Has tac_enabled after falling edge
            if tac_enabled && masked_cycles == 0 && prev_masked_cycles > 0 {
                self.tima = self.tima.wrapping_add(1);

                if self.tima == 0 {
                    self.tima = self.tma;
                    interrupt.set_timer_interrupt(); // TODO: Technically happens after another m-cycle, will fix later
                }
            }

            *cycle_buffer -= 1;
        }
    }
}
