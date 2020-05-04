use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub stuct MMU {
    // various MMU components: WRAM, VRAM, etc.
    //0000 - 3FFF From cartridge, usually a fixed bank
    //4000 - 7FFF From cartridge, switchable bank
    //8000 - 9FFF Vram, Only bank 0 in Non-CGB mode Switchable bank 0/1 in CGB mode
    //A000 - BFFF 8kb external ram
    //C000 - CFFF 4KB Work RAM (WRAM) bank 0	
    //D000 - DFFF 4KB Work RAM (WRAM) bank 1~N	Only bank 1 in Non-CGB mode Switchable bank 1~7 in CGB mode
    //E000 - FDFF Mirror of C000~DDFF (ECHO RAM)	Typically not used
    //FE00 - FE9F Sprite attribute table (OAM	
    //FEA0 - FEFF Not Usable	
    //FF00 - FF7F I/O Registers	
    //FF80 - FFFE High RAM (HRAM)	
    //FFFF - FFFF Interrupts Enable Register (IE)	

    memory: Vec<u8>,
    cart: Vec<u8>,    //cartridge
}

impl MMU {
    pub fn init(filename: &str, clk: &mut Clock) -> MMU {
        let mut mmu = MMU {
            // initialize elements of MMU to correct sizes and values
            memory: vec![0; 0x10000],
            cart: vec![0; 0x4000],
        }

        openRom(*filename, mmu);
        mmu
    }

    pub fn read_byte(addr: u16) -> u8 {
        memory[addr]
    }

    pub fn write_byte(addr: u16, data: u8) {
        memory[addr] = data;
    }

    pub fn read_word(addr: u16) -> u16 {
        ((memory[addr] as u16) << 8) | (memory[addr + 1] as u16);
    }

    pub fn write_word(addr: u16, data: u16) {
        memory[addr] = (data >> 8) as u8;
        memory[addr + 1] = (data & 0x00ff) as u8;
    }

    pub fn openRom(name: &str, mmu: &mut MMU) {
        //let romName = *name;
        let path = Path::new(name);
        let display = path.display();
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display,why.description()),
            Ok(file) => file,
        };

        // need to figure out how we're putting cart in mem (separate module for cart?)
        file.read_to_end(mmu.memory);
    }

}