use crate::regs::Regs;
use crate::mmu::MMU;

const B_WIDTH: usize = 256;  // Background
const B_HEIGHT: usize = 256;
const W_WIDTH: usize = 160;  // Windows
const W_HEIGHT: usize = 144;

pub struct GPU{
    SX : u8,  // scrollx & scrolly
    SY : u8,
    LCDC : u8,  // LCD Control byte FF40
    STAT : u8,  // LCDC Status FF41
    scan_line : u8, // Current scan line
    BGP : u8,  // background palette, 0xFF47
    mmu : MMU,

    // each tile is 16 bytes
    tile_set : Vec<u128>,   // pick one between 0 or 1
    tile_set_0 : Vec<u128>,  // 8000-8FFF, 0-255
    tile_set_1 : Vec<u128>,  // 8800-97FF, -128-127

    background : Vec<u32>,
    window : Vec<u32>,
}

impl GPU {

    pub fn init(mmu_ : MMU) -> GPU {
        let mut gpu = GPU{
            SX : 0,
            SY : 0,
            LCDC : 0,
            STAT : 0,
            scan_line : 0,
            BGP : 0,

            mmu : mmu_,
            tile_set : vec![0; 256],
            tile_set_0 : vec![0; 256],
            tile_set_1 : vec![0; 256],
            background : vec![0; B_WIDTH * B_HEIGHT],
            window : vec![0; W_WIDTH * W_HEIGHT],
        };
        gpu
    }

    pub fn update_tile(&mut self) {
        // update tile sets
        for i in 0..255{
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
    }

    pub fn output(&mut self) -> Vec<u32>{
        // still need to first render the window on the background
        let mut buffer : Vec<u32> = vec![0; W_WIDTH * W_HEIGHT];

        for i in 1..32 {
            
        }

        buffer
    }
}