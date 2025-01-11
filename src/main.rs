use std::{env, time::Instant};

mod bus;
mod cartridge;
mod cpu;
mod display;
mod hram;
mod io;
mod memory;
mod opcode;
mod ppu;
mod wram;

use bus::Bus;

// use cartridge::Cartridge::Cartridge;

use display::{get_scroll_data, ObjectAttributeData};
use io::lcd::PaletteData;
use macroquad::prelude::*;
use ppu::FrameBuffer;

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
    set_default_filter_mode(FilterMode::Nearest);

    // Read CL Args
    let args: Vec<String> = env::args().collect();
    let rom_filename: &String = &args[1];

    println!("Rom Filename: {}", rom_filename);

    let mut bus = Bus::new(rom_filename);

    let mut frame_counter: u32 = 0;
    let mut fps_display = String::new();
    let DISPLAY_FPS = false;

    let mut instant_time = Instant::now();

    while !is_key_down(KeyCode::Escape) {
        get_input(&mut bus);
        frame_counter = frame_counter.wrapping_add(1);
        let should_update_frame = bus.run_cycle();

        if should_update_frame {
            draw_macroquad_frame(bus.ppu.frame_buffer);
            draw_fps(DISPLAY_FPS, frame_counter, &mut fps_display);

            next_frame().await;
            instant_time = limit_60_fps(instant_time);
        }
    }
}

fn get_input(bus: &mut Bus) {
    if is_key_pressed(KeyCode::Right) {
        bus.io.joypad.joypad_buttons.set_right(true);
    }
    if is_key_released(KeyCode::Right) {
        bus.io.joypad.joypad_buttons.set_right(false);
    }
    if is_key_pressed(KeyCode::Left) {
        bus.io.joypad.joypad_buttons.set_left(true);
    }
    if is_key_released(KeyCode::Left) {
        bus.io.joypad.joypad_buttons.set_left(false);
    }
    if is_key_pressed(KeyCode::Up) {
        bus.io.joypad.joypad_buttons.set_up(true);
    }
    if is_key_released(KeyCode::Up) {
        bus.io.joypad.joypad_buttons.set_up(false);
    }
    if is_key_pressed(KeyCode::Down) {
        bus.io.joypad.joypad_buttons.set_down(true);
    }
    if is_key_released(KeyCode::Down) {
        bus.io.joypad.joypad_buttons.set_down(false);
    }
    if is_key_pressed(KeyCode::Z) {
        bus.io.joypad.joypad_buttons.set_a(true);
    }
    if is_key_released(KeyCode::Z) {
        bus.io.joypad.joypad_buttons.set_a(false);
    }
    if is_key_pressed(KeyCode::X) {
        bus.io.joypad.joypad_buttons.set_b(true);
    }
    if is_key_released(KeyCode::X) {
        bus.io.joypad.joypad_buttons.set_b(false);
    }
    if is_key_pressed(KeyCode::Space) {
        bus.io.joypad.joypad_buttons.set_start(true);
    }
    if is_key_released(KeyCode::Space) {
        bus.io.joypad.joypad_buttons.set_start(false);
    }
    if is_key_pressed(KeyCode::LeftShift) {
        bus.io.joypad.joypad_buttons.set_select(true);
    }
    if is_key_released(KeyCode::LeftShift) {
        bus.io.joypad.joypad_buttons.set_select(false);
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

fn draw_macroquad_frame(frame_buffer: FrameBuffer) {
    let color_map: [Vec<u8>; 4] = [
        vec![0x9a, 0x9e, 0x3f, 0xFF],
        vec![0x49, 0x6b, 0x22, 0xFF],
        vec![0x0e, 0x45, 0x0b, 0xFF],
        vec![0x1b, 0x2a, 0x09, 0xFF],
    ];

    let mut color_bytes: Vec<u8> = Vec::with_capacity(160 * 144 * 4);

    for pixel_row in frame_buffer {
        for pixel in pixel_row {
            color_bytes.extend_from_slice(&color_map[pixel as usize]);
        }
    }

    let texture = Texture2D::from_rgba8(160, 144, &color_bytes);

    draw_texture_ex(
        &texture,
        0.,
        0.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(160. * 4., 144. * 4.)),
            ..Default::default()
        },
    );
}

// TODO: GB technically not exactly 60 fps but this works for now
fn limit_60_fps(instant_time: Instant) -> Instant {
    std::thread::sleep(
        std::time::Duration::from_secs_f64(1. / 60.).saturating_sub(instant_time.elapsed()),
    );

    Instant::now()
}
