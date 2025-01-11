use super::interrupts::{self, Interrupt};

pub enum ScanLineEvent {
    OAMScanEntered,
    PixelTransferEntered,
    HBlankEntered,
    VBlankEntered,
    None,
}

#[derive(Debug)]
pub struct PaletteData {
    pub color_0: u8,
    pub color_1: u8,
    pub color_2: u8,
    pub color_3: u8,
}

impl PaletteData {
    pub fn default() -> PaletteData {
        PaletteData {
            color_0: 0,
            color_1: 0,
            color_2: 0,
            color_3: 0,
        }
    }

    pub fn get_color(&self, color: u8) -> u8 {
        match color {
            0 => self.color_0,
            1 => self.color_1,
            2 => self.color_2,
            3 => self.color_3,
            _ => panic!("Invalid color: {}", color),
        }
    }

    pub fn get_object_color(&self, color: u8) -> u8 {
        match color {
            0 => 4,
            1 => self.color_1,
            2 => self.color_2,
            3 => self.color_3,
            _ => panic!("Invalid color: {}", color),
        }
    }

    pub fn from_u8(byte: u8) -> PaletteData {
        let color_0 = byte & 0x03;
        let color_1 = (byte & 0x0C) >> 2;
        let color_2 = (byte & 0x30) >> 4;
        let color_3 = (byte & 0xC0) >> 6;

        PaletteData {
            color_0,
            color_1,
            color_2,
            color_3,
        }
    }
}

pub struct LCD {
    pub lcd_control: u8,      // 0xFF40
    pub lcd_status: u8,       // 0xFF41
    scroll_y: u8,             // 0xFF42
    scroll_x: u8,             // 0xFF43
    pub lcd_y_coordinate: u8, // 0xFF44
    lcd_y_cycles: u16,
    ly_compare: u8,    // 0xFF45
    bg_palette: u8,    // 0xFF47
    obj_palette_0: u8, // 0xFF48
    obj_palette_1: u8, // 0xFF49
    window_x: u8,      // 0xFF4A
    window_y: u8,      // 0xFF4B
}

impl LCD {
    pub fn default() -> LCD {
        LCD {
            lcd_control: 0x91, // 0b10010001 - Default value, LCD on, BG Display On
            lcd_status: 0x85,  // 0b10000101 - Default value
            scroll_y: 0,
            scroll_x: 0,
            lcd_y_coordinate: 0,
            lcd_y_cycles: 0,
            ly_compare: 0,
            bg_palette: 0xFC, // 0b11111100
            obj_palette_0: 0xFF,
            obj_palette_1: 0xFF,
            window_x: 0,
            window_y: 0,
        }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            0xFF40 => self.lcd_control,
            0xFF41 => self.lcd_status,
            0xFF42 => self.scroll_y,
            0xFF43 => self.scroll_x,
            0xFF44 => self.lcd_y_coordinate,
            0xFF45 => self.ly_compare,
            0xFF47 => self.bg_palette,
            0xFF48 => self.obj_palette_0,
            0xFF49 => self.obj_palette_1,
            0xFF4A => self.window_y,
            0xFF4B => self.window_x,
            _ => panic!("Invalid LCD Read address: 0x{:04X}", address),
        }
    }

    pub fn write_u8(&mut self, address: u16, value: u8) {
        match address {
            0xFF40 => self.lcd_control = value,
            0xFF41 => self.lcd_status = value,
            0xFF42 => self.scroll_y = value,
            0xFF43 => self.scroll_x = value,
            0xFF44 => self.lcd_y_coordinate = 0, // Writing to LY resets the value
            0xFF45 => self.ly_compare = value,
            0xFF47 => self.bg_palette = value,
            0xFF48 => self.obj_palette_0 = value,
            0xFF49 => self.obj_palette_1 = value,
            0xFF4A => self.window_y = value,
            0xFF4B => self.window_x = value,
            _ => panic!("Invalid LCD Write address: 0x{:04X}", address),
        }
    }

    pub fn is_lcd_enabled(&self) -> bool {
        self.lcd_control & 0x80 != 0
    }

    pub fn is_background_enabled(&self) -> bool {
        self.lcd_control & 0x01 != 0
    }

    pub fn window_tile_map_area(&self) -> u16 {
        if self.lcd_control & 0x40 == 0 {
            0x9800
        } else {
            0x9C00
        }
    }

    pub fn is_window_enabled(&self) -> bool {
        self.lcd_control & 0x20 != 0
    }

    pub fn bg_tile_map_area(&self) -> u16 {
        if self.lcd_control & 0x08 == 0 {
            0x9800
        } else {
            0x9C00
        }
    }

    // TODO: This is wrong, offset works differently - see pandocs
    pub fn bg_window_tile_data_area(&self) -> u16 {
        if self.lcd_control & 0x10 == 0 {
            0x9000
        } else {
            0x8000
        }
    }

    pub fn object_height(&self) -> u8 {
        if self.lcd_control & 0x04 == 0 {
            8
        } else {
            16
        }
    }

    pub fn is_8_height(&self) -> bool {
        self.lcd_control & 0x04 == 0
    }

    pub fn is_object_enabled(&self) -> bool {
        self.lcd_control & 0x02 != 0
    }

    pub fn get_scroll_data(&self) -> (u8, u8) {
        (self.scroll_y, self.scroll_x)
    }

    pub fn get_window_scroll_data(&self) -> (u8, u8) {
        (self.window_y, self.window_x)
    }

    fn set_mode_2(&mut self) {
        self.lcd_status &= 0xFC;
        self.lcd_status |= 0x02;
    }

    fn set_mode_3(&mut self) {
        self.lcd_status &= 0xFC;
        self.lcd_status |= 0x03;
    }

    fn set_mode_0(&mut self) {
        self.lcd_status &= 0xFC;
    }

    fn mode_2_lcd_interrupt(&self, interrupt: &mut Interrupt) -> () {
        if self.lcd_status & 0x20 != 0 {
            interrupt.set_lcd_interrupt();
        }
    }

    fn mode_0_lcd_interrupt(&self, interrupt: &mut Interrupt) -> () {
        if self.lcd_status & 0x08 != 0 {
            interrupt.set_lcd_interrupt();
        }
    }

    fn is_before_vblank(&self) -> bool {
        self.lcd_y_coordinate < 144
    }

    fn enters_mode_3(&self, previous_cycles: u16) -> bool {
        self.lcd_y_cycles < 252 && previous_cycles < 80
    }

    fn enters_mode_0(&self, previous_cycles: u16) -> bool {
        self.lcd_y_cycles >= 252 && previous_cycles < 252
    }

    fn scanline_complete(&self) -> bool {
        self.lcd_y_cycles >= 456
    }

    fn handle_ly_eq_ly_compare(&mut self, interrupt: &mut Interrupt) {
        if self.lcd_y_coordinate == self.ly_compare {
            self.lcd_status |= 0x04;

            if self.lcd_status & 0x40 != 0 {
                interrupt.set_lcd_interrupt();
            }
        } else {
            self.lcd_status &= 0xFB;
        }
    }

    pub fn update_ly(&mut self, cycles: u8, interrupt: &mut Interrupt) -> ScanLineEvent {
        let mut scanline_event = ScanLineEvent::None;

        let previous_cycles = self.lcd_y_cycles;
        self.lcd_y_cycles += cycles as u16 * 4; // T cycles?

        if self.is_before_vblank() {
            if self.enters_mode_3(previous_cycles) {
                // Mode 3
                self.set_mode_3();
                scanline_event = ScanLineEvent::PixelTransferEntered;
            } else if self.enters_mode_0(previous_cycles) {
                // Mode 0
                self.set_mode_0();
                self.mode_0_lcd_interrupt(interrupt);
                scanline_event = ScanLineEvent::HBlankEntered;
            }
        }

        if self.scanline_complete() {
            self.lcd_y_coordinate += 1;
            self.lcd_y_cycles -= 456;

            self.handle_ly_eq_ly_compare(interrupt);

            if self.lcd_y_coordinate == 144 {
                self.lcd_status |= 0x01; // Set the VBlank flag
                interrupt.set_vblank_interrupt();
                if self.lcd_status & 0x10 != 0 {
                    interrupt.set_lcd_interrupt();
                }
                scanline_event = ScanLineEvent::VBlankEntered;
            } else if self.lcd_y_coordinate > 153 {
                self.lcd_y_coordinate = 0;
                self.lcd_status &= 0xFE; // Clear the VBlank flag
                interrupt.disable_vblank_interrupt();

                self.set_mode_2();
                self.mode_2_lcd_interrupt(interrupt);
                scanline_event = ScanLineEvent::OAMScanEntered;
            } else if self.lcd_y_coordinate < 144 {
                self.set_mode_2();
                self.mode_2_lcd_interrupt(interrupt);
                scanline_event = ScanLineEvent::OAMScanEntered;
            }
        }

        scanline_event
    }

    pub fn get_palette_data(&self) -> PaletteData {
        PaletteData::from_u8(self.bg_palette)
    }

    pub fn get_object_palette_0_data(&self) -> PaletteData {
        PaletteData::from_u8(self.obj_palette_0)
    }

    pub fn get_object_palette_1_data(&self) -> PaletteData {
        PaletteData::from_u8(self.obj_palette_1)
    }
}
