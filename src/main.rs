use std::env;

mod bus;
mod cartridge;
mod cpu;
mod display;
mod hram;
mod io;
mod memory;
mod opcode;
mod wram;

use bus::Bus;

// use cartridge::Cartridge::Cartridge;

use display::{get_frame_data, get_scroll_data, ObjectAttributeData};
use io::lcd::PaletteData;
use macroquad::prelude::*;

fn macroquad_window_conf() -> Conf {
    Conf {
        window_title: String::from("Rustboy 2"),
        window_width: 160 * 4,
        window_height: 144 * 4,

        ..Default::default()
    }
}

type ProcessedTile = [[u8; 8]; 8];
type ProcessedTiles = [[ProcessedTile; 32]; 32];

#[macroquad::main(macroquad_window_conf)]
async fn main() {
    // Read CL Args
    let args: Vec<String> = env::args().collect();
    let rom_filename: &String = &args[1];

    println!("Rom Filename: {}", rom_filename);

    let mut bus = Bus::new(rom_filename);

    let mut tile_textures: [[Texture2D; 32]; 32] = core::array::from_fn(|_| {
        core::array::from_fn(|_| Texture2D::from_rgba8(8, 8, &[0; 8 * 8 * 4]))
    });

    let mut frame_counter: u32 = 0;
    let mut fps_display = String::new();
    let DISPLAY_FPS = true;

    while !is_key_down(KeyCode::Escape) {
        frame_counter = frame_counter.wrapping_add(1);
        let should_update_frame = bus.run_cycle();

        if should_update_frame {
            let (frame_data, object_attribute_data): (ProcessedTiles, ObjectAttributeData) =
                get_frame_data(&mut bus);
            let scroll_data = get_scroll_data(&bus);
            let bg_pallete: PaletteData = bus.io.lcd.get_palette_data();

            draw_fps(DISPLAY_FPS, frame_counter, &mut fps_display);

            draw_macroquad_frame(
                frame_data,
                scroll_data,
                &mut tile_textures,
                object_attribute_data,
                bg_pallete,
            );

            next_frame().await;
        }
    }
}

fn draw_fps(display_fps: bool, frame_counter: u32, fps_display: &mut String) {
    if display_fps {
        if frame_counter % 6 == 0 {
            *fps_display = format!("{}", get_fps());
        }
        draw_text(&fps_display, 0., 20., 40., RED);
    }
}

fn draw_macroquad_frame(
    frame_data: ProcessedTiles,
    scroll_data: (u8, u8),
    tile_textures: &mut [[Texture2D; 32]; 32],
    object_attribute_data: ObjectAttributeData,
    bg_pallete: PaletteData,
) {
    let color_map: [Vec<u8>; 4] = [
        vec![0x9a, 0x9e, 0x3f, 0xFF],
        vec![0x49, 0x6b, 0x22, 0xFF],
        vec![0x0e, 0x45, 0x0b, 0xFF],
        vec![0x1b, 0x2a, 0x09, 0xFF],
    ];

    let object_color_map: [Vec<u8>; 4] = [
        vec![0x00, 0x00, 0x00, 0x00],
        vec![0x49, 0x6b, 0x22, 0xFF],
        vec![0x0e, 0x45, 0x0b, 0xFF],
        vec![0x1b, 0x2a, 0x09, 0xFF],
    ];

    let (scroll_y, scroll_x) = scroll_data;

    // println!("Palette: {:?}", bg_pallete);

    // Draw Background
    for row in 0..32 {
        for col in 0..32 {
            let tile: ProcessedTile = frame_data[row][col];

            let mut color_bytes: Vec<u8> = Vec::with_capacity(8 * 8 * 4);

            for pixel_row in tile {
                for pixel in pixel_row {
                    color_bytes.extend_from_slice(&color_map[bg_pallete.get_color(pixel) as usize]);
                }
            }

            tile_textures[row][col].update_from_bytes(8, 8, &color_bytes);

            let row_pos = (row as u8 * 8).wrapping_sub(scroll_y) / 8;
            let col_pos = (col as u8 * 8).wrapping_sub(scroll_x) / 8;

            if row_pos >= 160 || col_pos >= 144 {
                continue;
            }
            draw_texture_ex(
                &tile_textures[row][col],
                col_pos as f32 * 32.,
                row_pos as f32 * 32.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(32., 32.)),
                    ..Default::default()
                },
            );
        }
    }

    // Draw Objects

    for (object_attribute, tile, tile2) in object_attribute_data.iter() {
        let y = object_attribute.y;
        let x = object_attribute.x;

        if y == 0 || x == 0 || y >= 160 || x >= 168 {
            continue;
        }

        let mut color_bytes: Vec<u8> = Vec::with_capacity(8 * 8 * 4);

        for pixel_row in tile {
            for pixel in pixel_row {
                color_bytes.extend_from_slice(&object_color_map[*pixel as usize]);
            }
        }

        let tile_texture = Texture2D::from_rgba8(8, 8, &color_bytes);

        let y_pos: f32 = (y as f32 * 4.0) - (16. * 4.);
        let x_pos: f32 = (x as f32 * 4.0) - (8. * 4.);

        draw_texture_ex(
            &tile_texture,
            x_pos,
            y_pos,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(32., 32.)),
                ..Default::default()
            },
        );
    }
}
