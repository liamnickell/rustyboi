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

    memory: [u8 : 0x10000];
    cart: [u8: 0x4000]; //cartriage 
}

impl MMU {
    pub fn init(filename: &str) -> MMU {
        MMU {
            // initialize elements of MMU to correct sizes and values
            openRom(*filename);
        }
    }

    pub fn read(addr: u16) -> u8 {
        return memory[addr];
    }

    pub fn write(addr: u16, data: u8) {
        memory[addr] = data;   
    }

    pub fn openRom(name: &str){
        //let romName = *name;
        let path = Path::new(name);
        let display = path.display();
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display,why.description()),
            Ok(file) => file,
        };

        let mut romData = Vec::new();
        file.read_to_end(&mut romData);

        //this is probably incorrect idk
        cart = romData;

        //load romData into memory
        for i in 0..romData.len{
            memory[i] = romData[i];
        }

        for i in 0..romData.len(){
            print!("{:X}, ", romData[i]);
        }
    }

}