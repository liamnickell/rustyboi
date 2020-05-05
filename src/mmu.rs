use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


pub struct MMU {
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

    memory: [u8; 0x10000],
    cart: [u8; 0x4000], //cartriage 
}

impl MMU {
    pub fn init(rom_file: &str) -> MMU {
        let mut mmu = MMU {
            memory: [0; 0x10000],
            cart: [0; 0x4000]
        };
        
        mmu.open_rom(rom_file);
        mmu
    }

    pub fn read_byte(&mut self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write_byte(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn read_word(&mut self, addr: u16) -> u16 {
        ((self.memory[(addr + 1) as usize] as u16) << 8) | (self.memory[addr as usize] as u16)      // little endian
    }

    pub fn write_word(&mut self, addr: u16, data: u16) {
        self.memory[addr as usize] = (data & 0x00ff) as u8;        // little endian
        self.memory[(addr + 1) as usize] = (data >> 8) as u8;
    }

    pub fn open_rom(&mut self, name: &str) {
        //let romName = *name;
        let path = Path::new(name);
        let display = path.display();
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why.to_string()),
            Ok(file) => file,
        };

        let mut rom_data = Vec::new();
        file.read_to_end(&mut rom_data);

        //this is probably incorrect idk
        //self.cart = rom_data;

        //load rom_data into memory
        for i in 0..rom_data.len(){
            self.memory[i] = rom_data[i];
        }

        for i in 0..rom_data.len(){
            print!("{:X}, ", rom_data[i]);
        }
    }

    fn test(&mut self, x: u32) -> u32{
        x + 2
    }

}