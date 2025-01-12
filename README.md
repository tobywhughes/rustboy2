A Gameboy emulator written in rust

## About this project

Years ago (2017-2018), as a college student, I wrote a (kind of functional) Gameboy emulator as my first serious personal project as a way to learn rust. It was a very enlightening experience, to say the least.

Now, as (I would hope) a more experienced developer, I am going for round 2 to dust off my rust skills once again. I don't have any end goals for this project, I just want it to work well.

## Current Status

As of now, I am passing the blargg's `cpu_instrs` test as well as having the correct `dmg-acid2` rendering.

Joypad and PPU io is supported so it will play roms *technically*. PPU is currently functioning just based on a more simplistic "scanline" basis, meaning that it should work correctly for the vast majority of roms, but edge cases can exist.

MBC1 and MBC3 support is partially implemented. There is no battery support (meaning you can't save your games) or MBC3 RTC support.

No sound support yet. The developer requests you play the sounds in your head for a satisfactory experience.

## Current Focus

Two largest goals right now are sound support (APU) and supporting more cartridge types and functionality. With these two finished, the emulator will be more or less completely functional.

Long term focus will be on increased cycle accuracy, passing more test roms, and maybe eventually CGB support.

## Running the emulator

This project is still in development so no releases are provided.

If you have rust installed, you can build it yourself if you want.

Roms can be run by passing the filepath as a command line argument (e.g. `cargo run -- '.\some_rom.gb'`)