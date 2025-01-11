use crate::bus::Bus;

#[derive(Debug, Copy, Clone)]
pub struct ObjectAttribute {
    pub y: u8,
    pub x: u8,
    pub tile_number: u8,
    pub flags: u8,
}

impl ObjectAttribute {
    pub fn is_in_scanline(&self, scanline: u8, object_height: u8) -> bool {
        (scanline + 16) >= self.y && (scanline + 16) < (self.y + object_height)
    }

    pub fn is_obj_palette_0(&self) -> bool {
        (self.flags & 0x10) == 0
    }

    pub fn is_h_flip(&self) -> bool {
        (self.flags & 0x20) != 0
    }

    pub fn is_v_flip(&self) -> bool {
        (self.flags & 0x40) != 0
    }

    pub fn is_bg_priority(&self) -> bool {
        (self.flags & 0x80) != 0
    }
}

pub struct ObjectAttributeMemory {
    pub oam: [u8; 0xA0],
    pub dma: u8,
    pub dma_transfer: bool,
}

pub type ObjectAttributeArray = [ObjectAttribute; 40];

impl ObjectAttributeMemory {
    pub fn default() -> ObjectAttributeMemory {
        ObjectAttributeMemory {
            oam: [0; 0xA0],
            dma: 0,
            dma_transfer: false,
        }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize],
            0xFF46 => self.dma,
            _ => panic!("Invalid OAM Read address: 0x{:04X}", address),
        }
    }

    pub fn write_u8(&mut self, address: u16, value: u8) {
        match address {
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize] = value,
            0xFF46 => {
                self.dma = value;
                self.dma_transfer = true;
            }
            _ => panic!("Invalid OAM Write address: 0x{:04X}", address),
        }
    }

    pub fn get_object_attribute(&self, object_index: u8) -> ObjectAttribute {
        let offset = object_index as usize * 4;
        ObjectAttribute {
            y: self.oam[offset],
            x: self.oam[offset + 1],
            tile_number: self.oam[offset + 2],
            flags: self.oam[offset + 3],
        }
    }

    pub fn get_object_attributes(&self) -> ObjectAttributeArray {
        let objects: [ObjectAttribute; 40] =
            core::array::from_fn(|i| self.get_object_attribute(i as u8));
        objects
    }
}
