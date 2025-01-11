use crate::{
    bus::Bus,
    io::oam::{ObjectAttribute, ObjectAttributeArray},
};

type ProcessedTile = [[u8; 8]; 8];
type ProcessedTiles = [[ProcessedTile; 32]; 32];

pub type ObjectAttributeData = [(ObjectAttribute, ProcessedTile, ProcessedTile); 40];

// Original tile based renderer, scrapping for scanline renderer

// pub fn get_frame_data(bus: &mut Bus) -> (ProcessedTiles, ObjectAttributeData) {
//     // println!("Window Enabled: {}", bus.io.lcd.window_enabled());
//     // println!("LCDC: {:08b}", bus.io.lcd.lcd_control);

//     // if bus.io.lcd.lcd_status > 3 {
//     //     println!("LCD Status: {:08b}", bus.io.lcd.lcd_status);
//     // }

//     // Background Tile Map
//     let bg_tile_map = bus.io.vram.get_tile_map(bus.io.lcd.bg_tile_map_area());
//     let mut processed_bg_tiles: ProcessedTiles = [[[[0; 8]; 8]; 32]; 32];
//     for (index, bg_tile) in bg_tile_map.iter().enumerate() {
//         let bg_area = bus.io.lcd.bg_window_tile_data_area();

//         let tile = bus.io.vram.get_tile(bg_area, *bg_tile);
//         let procesed_tile: [[u8; 8]; 8] = bus.io.vram.process_tile(tile);

//         let row = index / 32;
//         let col = index % 32;

//         processed_bg_tiles[row][col] = procesed_tile;
//     }

//     // Objects
//     let object_attributes: ObjectAttributeArray = bus.io.oam.get_object_attributes();

//     let object_atribute_data: ObjectAttributeData = core::array::from_fn(|i| {
//         let tile = bus
//             .io
//             .vram
//             .get_tile(0x8000, object_attributes[i].tile_number);
//         // TODO: This is wrong, just setting this for now before 8x16 sprites
//         let tile2 = bus
//             .io
//             .vram
//             .get_tile(0x8000, object_attributes[i].tile_number);
//         let procesed_tile: [[u8; 8]; 8] = bus.io.vram.process_tile(tile);
//         let procesed_tile2: [[u8; 8]; 8] = bus.io.vram.process_tile(tile2);
//         (object_attributes[i], procesed_tile, procesed_tile2)
//     });

//     (processed_bg_tiles, object_atribute_data)
// }

pub fn get_scroll_data(bus: &Bus) -> (u8, u8) {
    bus.io.lcd.get_scroll_data()
}

// ------------------------------------------------------------------------------------------------

// New and improved scanline based renderer
// Still not going to be completely accurate, but is a step up

pub fn get_frame(bus: &mut Bus) -> () {}
