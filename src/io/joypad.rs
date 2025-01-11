pub struct JoyPadButtons {
    pub right: bool,
    pub left: bool,
    pub up: bool,
    pub down: bool,
    pub a: bool,
    pub b: bool,
    pub select: bool,
    pub start: bool,
}

impl JoyPadButtons {
    pub fn default() -> JoyPadButtons {
        JoyPadButtons {
            right: false,
            left: false,
            up: false,
            down: false,
            a: false,
            b: false,
            select: false,
            start: false,
        }
    }

    pub fn set_right(&mut self, pressed: bool) {
        self.right = pressed;
    }

    pub fn set_left(&mut self, pressed: bool) {
        self.left = pressed;
    }

    pub fn set_up(&mut self, pressed: bool) {
        self.up = pressed;
    }

    pub fn set_down(&mut self, pressed: bool) {
        self.down = pressed;
    }

    pub fn set_a(&mut self, pressed: bool) {
        self.a = pressed;
    }

    pub fn set_b(&mut self, pressed: bool) {
        self.b = pressed;
    }

    pub fn set_select(&mut self, pressed: bool) {
        self.select = pressed;
    }

    pub fn set_start(&mut self, pressed: bool) {
        self.start = pressed;
    }

    pub fn direction_button_mask(&self) -> u8 {
        let mut mode_mask = 0xF0;

        if self.right {
            mode_mask |= 0x01;
        }

        if self.left {
            mode_mask |= 0x02;
        }

        if self.up {
            mode_mask |= 0x04;
        }

        if self.down {
            mode_mask |= 0x08;
        }

        !mode_mask
    }

    pub fn action_button_mask(&self) -> u8 {
        let mut mode_mask = 0xF0;

        if self.a {
            mode_mask |= 0x01;
        }

        if self.b {
            mode_mask |= 0x02;
        }

        if self.select {
            mode_mask |= 0x04;
        }

        if self.start {
            mode_mask |= 0x08;
        }

        !mode_mask
    }
}

pub struct Joypad {
    pub joypad_state: u8, // FF00
    pub joypad_buttons: JoyPadButtons,
}

impl Joypad {
    pub fn default() -> Joypad {
        Joypad {
            joypad_state: 0x0F,
            joypad_buttons: JoyPadButtons::default(),
        }
    }

    fn is_action_button_mode(&self) -> bool {
        self.joypad_state & 0x20 == 0
    }

    fn is_direction_button_mode(&self) -> bool {
        self.joypad_state & 0x10 == 0
    }

    pub fn read_joypad_buttons(&self) -> u8 {
        let mut button_state = 0x0F;

        if self.is_action_button_mode() {
            button_state &= self.joypad_buttons.action_button_mask();
        }

        if self.is_direction_button_mode() {
            button_state &= self.joypad_buttons.direction_button_mask();
        }

        button_state
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            0xFF00 => self.read_joypad_buttons(),
            _ => panic!("Invalid Joypad Read address: 0x{:04X}", address),
        }
    }

    pub fn write_u8(&mut self, address: u16, value: u8) {
        match address {
            0xFF00 => self.joypad_state = (self.joypad_state & 0x0F) | (value & 0xF0),
            _ => panic!("Invalid Joypad Write address: 0x{:04X}", address),
        }
    }
}
