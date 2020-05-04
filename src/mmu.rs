pub stuct MMU {
    // various MMU components: WRAM, VRAM, etc.
}

impl MMU {
    pub fn init() -> MMU {
        MMU {
            // initialize elements of MMU to correct sizes and values
        }
    }

    pub fn read(addr: u16) -> u8 {
        // read from correct place
    }

    pub fn write(addr: u16, data: u8) {
        // write to correct place
    }
}