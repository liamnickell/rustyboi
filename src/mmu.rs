use std::error::Error;
use std::result::Result;
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
            cart: [0; 0x4000],        
        };
        
        mmu.open_rom(rom_file);
        mmu.cart_init();
        mmu
    }

    // internal information in each cart from 0100-014F
    pub fn cart_init(&mut self){
        //Scrolling Nintendo graphic
        self.cart[0x104] = 0xCE;
        self.cart[0x105] = 0xED;
        self.cart[0x106] = 0x66;
        self.cart[0x107] = 0x66;
        self.cart[0x108] = 0xCC;
        self.cart[0x109] = 0x0D;
        self.cart[0x10A] = 0x00;
        self.cart[0x10B] = 0x0B;
        self.cart[0x10C] = 0x03;
        self.cart[0x10D] = 0x73;
        self.cart[0x10E] = 0x00;
        self.cart[0x10F] = 0x83;
        self.cart[0x110] = 0x00;
        self.cart[0x111] = 0x0C;
        self.cart[0x112] = 0x00;
        self.cart[0x113] = 0x0D;
        self.cart[0x114] = 0x00;
        self.cart[0x115] = 0x08;
        self.cart[0x116] = 0x11;
        self.cart[0x117] = 0x1F;
        self.cart[0x118] = 0x88;
        self.cart[0x119] = 0x89;
        self.cart[0x11A] = 0x00;
        self.cart[0x11B] = 0x0E;
        self.cart[0x11C] = 0xDC;
        self.cart[0x11D] = 0xCC;
        self.cart[0x11E] = 0x6E;
        self.cart[0x11F] = 0xE6;
        self.cart[0x120] = 0xDD;
        self.cart[0x121] = 0xDD;
        self.cart[0x122] = 0xD9;
        self.cart[0x123] = 0x99;
        self.cart[0x124] = 0xBB;
        self.cart[0x125] = 0xBB;
        self.cart[0x126] = 0x67;
        self.cart[0x127] = 0x63;
        self.cart[0x128] = 0x6E;
        self.cart[0x129] = 0x0E;
        self.cart[0x12A] = 0xEC;
        self.cart[0x12B] = 0xCC;
        self.cart[0x12C] = 0xDD;
        self.cart[0x12D] = 0xDC;
        self.cart[0x12E] = 0x99;
        self.cart[0x12F] = 0x9F;
        self.cart[0x130] = 0xBB;
        self.cart[0x131] = 0xB9;
        self.cart[0x132] = 0x33;
        self.cart[0x133] = 0x3E;
        
        self.cart[0x143] = 0x80; //color GB
        self.cart[0x146] = 0; // gb, not super gb
        self.cart[0x147] = 0; // using ROM-only cartridge for now
        self.cart[0x148] = 0; // using 32kb/two banks for now
        self.cart[0x149] = 0; // not using RAM in cartridge
        self.cart[0x14A] = 1; // we are not japanese lol
        self.cart[0x14D] = 1; // document does not make this clear, not sure

    }


    pub fn read_byte(&mut self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write_byte(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;

        // echoing E000-FE00 with C000-DE00--see section 2.5.2
        if (0xE000 <= addr) & (addr <= 0xFE00){
            self.memory[addr as usize] = data;
            let mirror = addr - 0x2000;
            self.memory[mirror as usize] = data;
        }
        if (0xC000 <= addr) & (addr <= 0xE000){
            self.memory[addr as usize] = data;
            let mirror = addr + 0x2000;
            self.memory[mirror as usize] = data;
        }
    }

    pub fn read_word(&mut self, addr: u16) -> u16 {
        ((self.memory[(addr + 1) as usize] as u16) << 8) | (self.memory[addr as usize] as u16)      // little endian
    }

    pub fn write_word(&mut self, addr: u16, data: u16) {
        self.memory[addr as usize] = (data & 0x00ff) as u8;        // little endian
        self.memory[(addr + 1) as usize] = (data >> 8) as u8;

        // echoing E000-FE00 with C000-DE00--see section 2.5.2
        if (0xE000 <= addr) & (addr <= 0xFE00){
            self.memory[addr as usize] = (data & 0x00ff) as u8;
            self.memory[(addr + 1) as usize] = (data >> 8) as u8;
            let mirror = addr - 0x2000;
            self.memory[mirror as usize] = (data & 0x00ff) as u8;
            self.memory[(mirror + 1) as usize] = (data >> 8) as u8;
        }
        if (0xC000 <= addr) & (addr <= 0xE000){
            self.memory[addr as usize] = (data & 0x00ff) as u8;
            self.memory[(addr + 1) as usize] = (data >> 8) as u8;
            let mirror = addr + 0x2000;
            self.memory[mirror as usize] = (data & 0x00ff) as u8;
            self.memory[(mirror + 1) as usize] = (data >> 8) as u8;
        }
    }

    pub fn open_rom(&mut self, name: &str){
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
        //self.cart = romData;

        //load romData into memory
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