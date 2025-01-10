pub struct VRam {
    data: [[u8; 0x2000]; 2],
    vram_bank: u8, // FF4F
}

impl VRam {
    pub fn default() -> VRam {
        VRam {
            data: [[0; 0x2000]; 2],
            vram_bank: 0,
        }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            0xFF4F => self.vram_bank,
            0x8000..=0x9FFF => self.data[self.vram_bank as usize][(address - 0x8000) as usize],
            _ => panic!("Invalid VRam Read address: 0x{:04X}", address),
        }
    }

    pub fn write_u8(&mut self, address: u16, value: u8) {
        match address {
            0xFF4F => self.vram_bank = value,
            0x8000..=0x9FFF => {
                self.data[self.vram_bank as usize][(address - 0x8000) as usize] = value
            }
            _ => panic!("Invalid VRam Write address: 0x{:04X}", address),
        }
    }

    pub fn get_tile_map(&self, address: u16) -> [u8; 0x400] {
        let mapped_address = (address - 0x8000) as usize;

        let mut tile_map = [0; 0x400];
        tile_map.clone_from_slice(
            &self.data[self.vram_bank as usize][mapped_address..(mapped_address + 0x400)],
        );

        tile_map
    }

    pub fn get_tile(&self, offset: u16, tile_index: u8) -> [u8; 16] {
        let tile_address: usize = match tile_index {
            0..=127 => ((offset - 0x8000) + ((tile_index as u16) * 16)) as usize,
            128..=255 => (tile_index as usize) * 16,
        };

        let mut tile = [0; 16];
        tile.clone_from_slice(
            &self.data[self.vram_bank as usize]
                [tile_address as usize..(tile_address as usize + 16)],
        );

        tile
    }

    pub fn process_tile(&self, tile: [u8; 16]) -> [[u8; 8]; 8] {
        let mut processed_tile = [[0; 8]; 8];

        for tile_row in 0..8 {
            let low_byte = tile[tile_row * 2];
            let high_byte = tile[(tile_row * 2) + 1];

            for tile_col in 0..8 {
                let low_bit = (low_byte >> (7 - tile_col)) & 0x1;
                let high_bit = (high_byte >> (7 - tile_col)) & 0x1;

                let processed_value: u8 = (high_bit << 1) | low_bit;

                processed_tile[tile_row][tile_col] = processed_value
            }
        }

        processed_tile
    }
}
