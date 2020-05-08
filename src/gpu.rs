use crate::regs::Regs;
use crate::mmu::MMU;
use std::mem::transmute;

const B_WIDTH: usize = 256;  // Background
const B_HEIGHT: usize = 256;
const W_WIDTH: usize = 160;  // Windows
const W_HEIGHT: usize = 144;

const modeOAM : u8 = 2;
const modeVRAM : u8 = 3;
const modeHBLANK : u8 = 0;
const modeVBLANK : u8 = 1;

pub struct GPU{
    SX : u8,  // scrollx & scrolly
    SY : u8,
    LCDC : u8,  // LCD Control byte FF40
    STAT : u8,  // LCDC Status FF41
    scan_line : u8, // Current scan line
    BGP : u8,  // background palette, 0xFF47
    mmu : MMU,
    cycles : u32,
    mode : u8,

    // each tile is 16 bytes
    tile_set : Vec<u128>,   // pick one between 0 or 1
    tile_set_0 : Vec<u128>,  // 8000-8FFF, 0-255
    tile_set_1 : Vec<u128>,  // 8800-97FF, -128-127
    tile_set_colored : Vec<u64>,

    background : Vec<u32>,
    window : Vec<u32>,
}

impl GPU {

    pub fn init(mmu_ : &mut MMU) -> GPU {
        let mut gpu = GPU{
            SX : 0,
            SY : 0,
            LCDC : 0,
            STAT : 0,
            scan_line : 0,
            BGP : 0,
            mmu : *mmu_,
            cycles : 0,
            mode : 0,

            tile_set : vec![0; 256],
            tile_set_0 : vec![0; 256],
            tile_set_1 : vec![0; 256],
            tile_set_colored : vec![0; 256],
            background : vec![0; B_WIDTH * B_HEIGHT],
            window : vec![0; W_WIDTH * W_HEIGHT],
        };
        gpu
    }

    pub fn update_tiles(&mut self) {
        // update tile sets
        for i in 0..255 {
            for j in 0..15 {//  each tile is 16 bytes, !!!!assuming little endian here for now!!!!!
                self.tile_set_0[i] += self.mmu.read_byte(0x8000 + (16 * i + j) as u16) as u128 * 2^(j as u128);
                self.tile_set_1[i] += self.mmu.read_byte(0x8800 + (16 * i + j) as u16) as u128 * 2^(j as u128);
            }
        }
        self.LCDC = self.mmu.read_byte(0xff40);
        self.STAT = self.mmu.read_byte(0xff41);
        self.BGP = self.mmu.read_byte(0xff47);

        let tile_set_index : bool = (self.LCDC & 0b0000_1000) == 0b0000_1000;  // tile set 0 or 1
        if (tile_set_index == true){   // using tile set 1
            self.tile_set = self.tile_set_1.clone();
        } else {
            self.tile_set = self.tile_set_0.clone();
        }

        for i in 0..255 {
            let byte_array : [u8; 16] = unsafe{ transmute(self.tile_set[i].to_be())};
            for j in 0..7{
                let byte1 : u8 = byte_array[j] as u8;
                let byte2 : u8 = byte_array[j+8] as u8;
                //println!("{:b}", byte1);
                //println!("{:b}", byte2);

                let x0 : u8 = ((byte1 & 0b0000_0001)> 0) as u8 * 2 + ((byte2 & 0b0000_0001) > 0) as u8;
                let x1 : u8 = ((byte1 & 0b0000_0010)> 0) as u8 * 2 + ((byte2 & 0b0000_0010) > 0) as u8;
                let x2 : u8 = ((byte1 & 0b0000_0100)> 0) as u8 * 2 + ((byte2 & 0b0000_0100) > 0) as u8;
                let x3 : u8 = ((byte1 & 0b0000_1000)> 0) as u8 * 2 + ((byte2 & 0b0000_1000) > 0) as u8;
                let x4 : u8 = ((byte1 & 0b0001_0000)> 0) as u8 * 2 + ((byte2 & 0b0001_0000) > 0) as u8;
                let x5 : u8 = ((byte1 & 0b0010_0000)> 0) as u8 * 2 + ((byte2 & 0b0010_0000) > 0) as u8;
                let x6 : u8 = ((byte1 & 0b0100_0000)> 0) as u8 * 2 + ((byte2 & 0b0100_0000) > 0) as u8;
                let x7 : u8 = ((byte1 & 0b1000_0000)> 0) as u8 * 2 + ((byte2 & 0b1000_0000) > 0) as u8;

                self.background[(i - i%32) * 64 + (i % 32) * 8 + j * 256 + 0] = x0 as u32;
                self.background[(i - i%32) * 64 + (i % 32) * 8 + j * 256 + 1] = x1 as u32;
                self.background[(i - i%32) * 64 + (i % 32) * 8 + j * 256 + 2] = x2 as u32;
                self.background[(i - i%32) * 64 + (i % 32) * 8 + j * 256 + 3] = x3 as u32;
                self.background[(i - i%32) * 64 + (i % 32) * 8 + j * 256 + 4] = x4 as u32;
                self.background[(i - i%32) * 64 + (i % 32) * 8 + j * 256 + 5] = x5 as u32;
                self.background[(i - i%32) * 64 + (i % 32) * 8 + j * 256 + 6] = x6 as u32;
                self.background[(i - i%32) * 64 + (i % 32) * 8 + j * 256 + 7] = x7 as u32;
            }
        }

    }

    pub fn output(&mut self) -> Vec<u32>{
        // print testing
        /*for tile in 0..255 {
            let byte_array : [u8; 16] = unsafe{ transmute(self.tile_set[tile].to_be())};
            //println!("tile num: {}", tile);
            for line in 0..7 {
                //println!("{:b}", byte_array[line]);
            }
        }*/

        // just a wrapper function
        self.background.clone()
    }

    pub fn step(&mut self, cycle_increase : u32) {
        self.cycles += cycle_increase as u32;
        if (self.mode == modeOAM){
            if (self.cycles >= 80){
                self.cycles = 0;
                self.mode = modeVRAM;
            }
        } else if (self.mode == modeVRAM) {
            if (self.cycles >= 172) {
                self.mode = modeHBLANK;

                // render the line on frame buffer using the previous methods
                
            }

        } else if (self.mode == modeHBLANK) {
            if (self.cycles >= 204) {
                self.cycles = 0;
                self.scan_line += 1;
                if (self.scan_line == 143){
                    self.mode = modeVBLANK;
                } else {
                    self.mode = modeOAM;
                }
            }
        } else if (self.mode == modeVBLANK) {
            //println!("vblank mode");
            self.update_tiles();
            if (self.cycles >= 456) {
                self.cycles = 0;
                self.scan_line += 1;
                if (self.scan_line > 153) {
                    self.mode = modeOAM;
                    self.scan_line = 0;
                }
            }
        }
    }
}