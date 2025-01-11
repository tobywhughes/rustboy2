use crate::{
    bus::{self, Bus},
    io::{io::IO, lcd::PaletteData, oam::ObjectAttribute, vram::Tile},
};

pub type ScanlineObjectBuffer = [u8; 10];
pub type ScanlineBuffer = [u8; 160];
pub type FrameBuffer = [ScanlineBuffer; 144];

// let bg_tile_map = io.vram.get_tile_map_2d(io.lcd.bg_tile_map_area());
// let bg_palette = io.lcd.get_palette_data();

pub struct BGState {
    bg_window_tile_data_area: u16,
    bg_palette: PaletteData,
    bg_tile_map: [[u8; 32]; 32],
    window_tile_map: [[u8; 32]; 32],
}

impl BGState {
    fn default() -> BGState {
        BGState {
            bg_window_tile_data_area: 0,
            bg_palette: PaletteData::default(),
            bg_tile_map: [[0; 32]; 32],
            window_tile_map: [[0; 32]; 32],
        }
    }
}

pub struct OAMState {
    object_palette_0: PaletteData,
    object_palette_1: PaletteData,
    is_8_height: bool,
}

impl OAMState {
    fn default() -> OAMState {
        OAMState {
            object_palette_0: PaletteData::default(),
            object_palette_1: PaletteData::default(),
            is_8_height: false,
        }
    }
}

pub struct PPU {
    scanline_object_id_buffer: ScanlineObjectBuffer,
    pub frame_buffer: FrameBuffer,
    pub window_internal_line_counter: u8,
    oam_state: OAMState,
    bg_state: BGState,
    current_scanline: u8,
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            scanline_object_id_buffer: [0xFF; 10], // 0xFF being treated as empty object
            frame_buffer: [[0; 160]; 144],
            window_internal_line_counter: 0,
            oam_state: OAMState::default(),
            bg_state: BGState::default(),
            current_scanline: 0,
        }
    }

    pub fn update_scanline_object_id_buffer(&mut self, io: &IO) {
        // Clear Buffer
        for i in 0..10 {
            self.scanline_object_id_buffer[i] = 0xFF;
        }

        // Don't scan objects if they are disabled
        if !io.lcd.is_object_enabled() {
            return;
        }

        // Scan Objects
        self.current_scanline = io.lcd.lcd_y_coordinate;
        let mut buffer_index = 0;
        let object_height = io.lcd.object_height();

        for object_index in 0..40 {
            let object_attribute = io.oam.get_object_attribute(object_index);
            if object_attribute.is_in_scanline(self.current_scanline, object_height) {
                self.scanline_object_id_buffer[buffer_index] = object_index as u8;
                buffer_index += 1;
            }

            // Only scan 10 objects
            if buffer_index >= 10 {
                break;
            }
        }
    }

    pub fn update_current_scanline_in_frame_buffer(&mut self, io: &IO) {
        self.current_scanline = io.lcd.lcd_y_coordinate;

        if self.current_scanline >= 144 {
            println!(
                "[Warning] - Attempted to update scanline during vblank - LY {}",
                self.current_scanline
            );
            return;
        }

        self.update_bg_state(io);

        // LCD Enable
        if !io.lcd.is_lcd_enabled() {
            for pixel_index in 0..160_u8 {
                self.frame_buffer[self.current_scanline as usize][pixel_index as usize] = 0x00;
            }
        }

        // Background
        if io.lcd.is_background_enabled() {
            self.render_background(io);
        } else {
            self.clear_background();
        }

        // Window
        let (window_scroll_y, window_scroll_x) = io.lcd.get_window_scroll_data();

        if io.lcd.is_window_enabled()
            && io.lcd.is_background_enabled()
            && self.current_scanline >= window_scroll_y
            && window_scroll_y < 144
        {
            self.render_window(io, window_scroll_x);
        }

        // Objects
        if io.lcd.is_object_enabled() {
            let object_attributes = self.get_object_attributes(io);

            self.update_oam_state(io);

            // Iterates in reverse for object priority
            for object_attribute in object_attributes.iter().rev() {
                self.render_object(object_attribute, io);
            }
        }
    }

    // General Helper Functions

    fn update_bg_state(&mut self, io: &IO) -> () {
        let bg_tile_map = io.vram.get_tile_map_2d(io.lcd.bg_tile_map_area());
        let bg_window_tile_data_area = io.lcd.bg_window_tile_data_area();
        let bg_palette = io.lcd.get_palette_data();
        let window_tile_map = io.vram.get_tile_map_2d(io.lcd.window_tile_map_area());

        self.bg_state = BGState {
            bg_window_tile_data_area,
            bg_palette,
            bg_tile_map,
            window_tile_map,
        }
    }

    fn get_tile(&self, io: &IO, scrolled_x: u8, scrolled_y: u8, is_bg: bool) -> Tile {
        let tile_map_x_offset = scrolled_x / 8;
        let tile_map_y_offset = scrolled_y / 8;

        let current_tile_index = if is_bg {
            self.bg_state.bg_tile_map[tile_map_y_offset as usize][tile_map_x_offset as usize]
        } else {
            self.bg_state.window_tile_map[tile_map_y_offset as usize][tile_map_x_offset as usize]
        };

        io.vram
            .get_tile(self.bg_state.bg_window_tile_data_area, current_tile_index)
    }

    fn render_tile_pixel(
        &mut self,
        io: &IO,
        tile: Tile,
        scrolled_x: u8,
        scrolled_y: u8,
        pixel_index: u8,
    ) {
        let tile_data_x_offset = scrolled_x % 8;
        let tile_data_y_offset = scrolled_y % 8;

        let raw_pixel = io.vram.get_tile_pixel(
            tile,
            tile_data_x_offset as usize,
            tile_data_y_offset as usize,
        );

        let palette_mapped_pixel = self.bg_state.bg_palette.get_color(raw_pixel);

        self.frame_buffer[self.current_scanline as usize][pixel_index as usize] =
            palette_mapped_pixel;
    }

    // Background Rendering Helper Functions

    fn render_background(&mut self, io: &IO) -> () {
        let (bg_scrolly_y, bg_scroll_x) = io.lcd.get_scroll_data();

        for pixel_index in 0..160_u8 {
            let scrolled_x: u8 = pixel_index.wrapping_add(bg_scroll_x);
            let scrolled_y: u8 = self.current_scanline.wrapping_add(bg_scrolly_y);

            // Get Tile Index
            let tile = self.get_tile(io, scrolled_x, scrolled_y, true);

            // Get Tile Pixel
            self.render_tile_pixel(io, tile, scrolled_x, scrolled_y, pixel_index);
        }
    }

    fn clear_background(&mut self) -> () {
        for pixel_index in 0..160_u8 {
            self.frame_buffer[self.current_scanline as usize][pixel_index as usize] = 0x00;
        }
    }

    // Window Rendering Helper Functions

    fn render_window(&mut self, io: &IO, window_scroll_x: u8) -> () {
        for pixel_index in 0..160 {
            self.render_window_pixel(io, window_scroll_x, pixel_index);
        }

        if window_scroll_x < 166 {
            self.window_internal_line_counter += 1;
        }
    }

    fn render_window_pixel(&mut self, io: &IO, window_scroll_x: u8, pixel_index: u8) {
        // Pixel outside of window
        if (pixel_index < (window_scroll_x - 7)) || window_scroll_x > 166 {
            return;
        }

        let scrolled_x: u8 = pixel_index + 7 - window_scroll_x;
        let scrolled_y: u8 = self.window_internal_line_counter;

        let tile = self.get_tile(io, scrolled_x, scrolled_y, false);

        // Get Tile Pixel
        self.render_tile_pixel(io, tile, scrolled_x, scrolled_y, pixel_index);
    }

    // Object Rendering Helper Functions

    fn update_oam_state(&mut self, io: &IO) -> () {
        let object_palette_0 = io.lcd.get_object_palette_0_data();
        let object_palette_1 = io.lcd.get_object_palette_1_data();

        let is_8_height = io.lcd.is_8_height();

        self.oam_state = OAMState {
            object_palette_0,
            object_palette_1,
            is_8_height,
        }
    }

    fn render_object(&mut self, object_attribute: &ObjectAttribute, io: &IO) -> () {
        // Do not render object if completely (all 8 pixels) outside of screen width
        if object_attribute.x == 0 || object_attribute.x >= 168 {
            return;
        }

        let (object_tile_lower, object_tile_higher) = self.get_object_tiles(io, object_attribute);

        let y_pixel = (self.current_scanline - (object_attribute.y - 16))
            % (if self.oam_state.is_8_height { 8 } else { 16 });

        for pixel_index in 0..8 {
            let in_window = self.render_object_pixel(
                io,
                object_attribute,
                object_tile_lower,
                object_tile_higher,
                pixel_index,
                y_pixel,
            );

            if !in_window {
                break;
            };
        }
    }

    fn render_object_pixel(
        &mut self,
        io: &IO,
        object_attribute: &ObjectAttribute,
        object_tile_lower: Tile,
        object_tile_higher: Tile,
        pixel_index: u8,
        y_pixel: u8,
    ) -> bool {
        let current_pixel_offset = object_attribute.x - 8 + pixel_index;

        // Do not render pixel if outside of screen
        // Differs from render_object check because some objects can be partially in frame
        if current_pixel_offset >= 160 {
            return false;
        }

        // Gets pixel offset for a specific tile
        let x_pixel_offset: usize = self.get_object_x_pixel_offset(object_attribute, pixel_index);

        let y_pixel_offset: usize = self.get_object_y_pixel_offset(object_attribute, y_pixel);

        let pixel = if y_pixel_offset < 8 {
            io.vram
                .get_tile_pixel(object_tile_lower, x_pixel_offset, y_pixel_offset)
        } else {
            io.vram
                .get_tile_pixel(object_tile_higher, x_pixel_offset, y_pixel_offset % 8)
        };

        let palette_mapped_pixel = self.map_pixel_by_palette(object_attribute, pixel);

        self.set_object_pixel(
            palette_mapped_pixel,
            current_pixel_offset as usize,
            object_attribute.is_bg_priority(),
        );

        true
    }

    fn get_object_x_pixel_offset(
        &self,
        object_attribute: &ObjectAttribute,
        pixel_index: u8,
    ) -> usize {
        if object_attribute.is_h_flip() {
            return 7 - pixel_index as usize;
        } else {
            return pixel_index as usize;
        };
    }

    fn get_object_y_pixel_offset(&self, object_attribute: &ObjectAttribute, y_pixel: u8) -> usize {
        if object_attribute.is_v_flip() {
            if self.oam_state.is_8_height {
                return 7 - y_pixel as usize;
            } else {
                return 15 - y_pixel as usize;
            }
        } else {
            return y_pixel as usize;
        };
    }

    fn get_object_attributes(&self, io: &IO) -> Vec<ObjectAttribute> {
        let mut object_attributes: Vec<ObjectAttribute> = Vec::new();

        for oam_index in self.scanline_object_id_buffer {
            if oam_index != 0xFF {
                object_attributes.push(io.oam.get_object_attribute(oam_index));
            }
        }

        object_attributes.sort_by(|a, b| a.x.cmp(&b.x));

        object_attributes
    }

    fn get_object_tiles(&self, io: &IO, object_attribute: &ObjectAttribute) -> (Tile, Tile) {
        let object_tile_lower = io.vram.get_tile(
            0x8000,
            if io.lcd.is_8_height() {
                object_attribute.tile_number
            } else {
                object_attribute.tile_number & 0xFE
            },
        );
        let object_tile_higher = io
            .vram
            .get_tile(0x8000, object_attribute.tile_number | 0x01);

        (object_tile_lower, object_tile_higher)
    }

    // TODO: Move to lcd probably
    fn map_pixel_by_palette(&self, object_attribute: &ObjectAttribute, pixel: u8) -> u8 {
        if object_attribute.is_obj_palette_0() {
            self.oam_state.object_palette_0.get_object_color(pixel)
        } else {
            self.oam_state.object_palette_1.get_object_color(pixel)
        }
    }

    fn set_object_pixel(&mut self, pixel: u8, pixel_offset: usize, is_bg_priority: bool) -> () {
        let current_pixel_value =
            self.frame_buffer[self.current_scanline as usize][pixel_offset as usize];

        let has_priority = !is_bg_priority || current_pixel_value == 0;

        if pixel != 4 && has_priority {
            self.frame_buffer[self.current_scanline as usize][pixel_offset as usize] = pixel;
        }
    }
}
