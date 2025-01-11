use crate::{
    bus::{self, Bus},
    io::{io::IO, lcd::PaletteData, oam::ObjectAttribute, vram::Tile},
};

pub type ScanlineObjectBuffer = [u8; 10];
pub type ScanlineBuffer = [u8; 160];
pub type FrameBuffer = [ScanlineBuffer; 144];

pub struct PPU {
    scanline_object_id_buffer: ScanlineObjectBuffer,
    pub frame_buffer: FrameBuffer,
    pub window_internal_line_counter: u8,
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            scanline_object_id_buffer: [0xFF; 10], // 0xFF being treated as empty object
            frame_buffer: [[0; 160]; 144],
            window_internal_line_counter: 0,
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
        let current_scanline = io.lcd.lcd_y_coordinate;
        let mut buffer_index = 0;
        let object_height = io.lcd.object_height();

        for object_index in 0..40 {
            let object_attribute = io.oam.get_object_attribute(object_index);
            if object_attribute.is_in_scanline(current_scanline, object_height) {
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
        let current_scanline = io.lcd.lcd_y_coordinate;

        if current_scanline >= 144 {
            println!(
                "[Warning] - Attempted to update scanline during vblank - LY {}",
                current_scanline
            );
            return;
        }

        let bg_tile_map = io.vram.get_tile_map_2d(io.lcd.bg_tile_map_area());
        let bg_window_tile_data_area = io.lcd.bg_window_tile_data_area();
        let bg_palette = io.lcd.get_palette_data();

        // LCD Enable
        if !io.lcd.is_lcd_enabled() {
            for pixel_index in 0..160_u8 {
                self.frame_buffer[current_scanline as usize][pixel_index as usize] = 0x00;
            }
        }

        // Background
        let (bg_scrolly_y, bg_scroll_x) = io.lcd.get_scroll_data();

        if io.lcd.is_background_enabled() {
            for pixel_index in 0..160_u8 {
                let scrolled_x: u8 = pixel_index.wrapping_add(bg_scroll_x);
                let scrolled_y: u8 = current_scanline.wrapping_add(bg_scrolly_y);

                // Get Tile Index
                let tile_map_x_offset = scrolled_x / 8;
                let tile_map_y_offset = scrolled_y / 8;

                let current_tile_index =
                    bg_tile_map[tile_map_y_offset as usize][tile_map_x_offset as usize];

                let tile = io
                    .vram
                    .get_tile(bg_window_tile_data_area, current_tile_index);

                // Get Tile Pixel
                let tile_data_x_offset = scrolled_x % 8;
                let tile_data_y_offset = scrolled_y % 8;

                let raw_pixel = io.vram.get_tile_pixel(
                    tile,
                    tile_data_x_offset as usize,
                    tile_data_y_offset as usize,
                );
                let palette_mapped_pixel = bg_palette.get_color(raw_pixel);

                self.frame_buffer[current_scanline as usize][pixel_index as usize] =
                    palette_mapped_pixel;
            }
        } else {
            for pixel_index in 0..160_u8 {
                self.frame_buffer[current_scanline as usize][pixel_index as usize] = 0x00;
            }
        }
        // Window
        let (window_scroll_y, window_scroll_x) = io.lcd.get_window_scroll_data();
        let window_tile_map = io.vram.get_tile_map_2d(io.lcd.window_tile_map_area());

        if io.lcd.is_window_enabled()
            && io.lcd.is_background_enabled()
            && current_scanline >= window_scroll_y
            && window_scroll_y < 144
        {
            for pixel_index in 0..160 {
                // Pixel outside of window
                if (pixel_index < (window_scroll_x - 7)) || window_scroll_x > 166 {
                    continue;
                }

                let scrolled_x: u8 = (pixel_index + 7 - window_scroll_x);
                let scrolled_y: u8 = (self.window_internal_line_counter);

                // Get Tile Index
                let tile_map_x_offset = scrolled_x / 8;
                let tile_map_y_offset = scrolled_y / 8;

                // println!(
                //     "({}, {}) - {} - {} - {}",
                //     tile_map_x_offset,
                //     tile_map_y_offset,
                //     current_scanline,
                //     window_scroll_y,
                //     self.window_internal_line_counter,
                // );

                let current_tile_index =
                    window_tile_map[tile_map_y_offset as usize][tile_map_x_offset as usize];

                let tile = io
                    .vram
                    .get_tile(bg_window_tile_data_area, current_tile_index);

                // Get Tile Pixel
                let tile_data_x_offset = scrolled_x % 8;
                let tile_data_y_offset = scrolled_y % 8;

                let raw_pixel = io.vram.get_tile_pixel(
                    tile,
                    tile_data_x_offset as usize,
                    tile_data_y_offset as usize,
                );

                let palette_mapped_pixel = bg_palette.get_color(raw_pixel);

                self.frame_buffer[current_scanline as usize][pixel_index as usize] =
                    palette_mapped_pixel;
            }

            if window_scroll_x < 166 {
                self.window_internal_line_counter += 1;
            }
        }

        // Objects

        // Iterates in reverse for object priority
        if io.lcd.is_object_enabled() {
            let mut object_attributes: Vec<ObjectAttribute> = Vec::new();
            for oam_index in self.scanline_object_id_buffer {
                if oam_index != 0xFF {
                    object_attributes.push(io.oam.get_object_attribute(oam_index));
                }
            }

            object_attributes.sort_by(|a, b| a.x.cmp(&b.x));

            let object_palette_0 = io.lcd.get_object_palette_0_data();
            let object_palette_1 = io.lcd.get_object_palette_1_data();

            let is_8_height = io.lcd.is_8_height();

            for object_attribute in object_attributes.iter().rev() {
                if object_attribute.x == 0 || object_attribute.x >= 168 {
                    continue;
                }

                let x_offset = object_attribute.x - 8;
                let y_offset = object_attribute.y - 16;

                let (object_tile_lower, object_tile_higher) =
                    self.get_object_tiles(io, object_attribute);

                let y_pixel = (current_scanline - y_offset) % (if is_8_height { 8 } else { 16 });

                for pixel_index in 0..8 {
                    let current_pixel_offset = x_offset + pixel_index;

                    if current_pixel_offset >= 160 {
                        break;
                    }

                    let pixel_index_x_offset: usize = if object_attribute.is_h_flip() {
                        7 - pixel_index as usize
                    } else {
                        pixel_index as usize
                    };

                    let y_pixel_offset: usize = if object_attribute.is_v_flip() {
                        if is_8_height {
                            7 - y_pixel as usize
                        } else {
                            15 - y_pixel as usize
                        }
                    } else {
                        y_pixel as usize
                    };

                    let pixel = if y_pixel_offset < 8 {
                        io.vram.get_tile_pixel(
                            object_tile_lower,
                            pixel_index_x_offset,
                            y_pixel_offset,
                        )
                    } else {
                        io.vram.get_tile_pixel(
                            object_tile_higher,
                            pixel_index_x_offset,
                            y_pixel_offset % 8,
                        )
                    };

                    let palette_mapped_pixel = self.map_pixel_by_palette(
                        object_attribute,
                        &object_palette_0,
                        &object_palette_1,
                        pixel,
                    );

                    self.set_object_pixel(
                        palette_mapped_pixel,
                        current_scanline as usize,
                        current_pixel_offset as usize,
                        object_attribute.is_bg_priority(),
                    );
                }
            }
        }
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
    fn map_pixel_by_palette(
        &self,
        object_attribute: &ObjectAttribute,
        object_palette_0: &PaletteData,
        object_palette_1: &PaletteData,
        pixel: u8,
    ) -> u8 {
        if object_attribute.is_obj_palette_0() {
            object_palette_0.get_object_color(pixel)
        } else {
            object_palette_1.get_object_color(pixel)
        }
    }

    fn set_object_pixel(
        &mut self,
        pixel: u8,
        scanline: usize,
        pixel_offset: usize,
        is_bg_priority: bool,
    ) -> () {
        let current_pixel_value = self.frame_buffer[scanline as usize][pixel_offset as usize];

        let has_priority = !is_bg_priority || current_pixel_value == 0;

        if pixel != 4 && has_priority {
            self.frame_buffer[scanline as usize][pixel_offset as usize] = pixel;
        }
    }
}
