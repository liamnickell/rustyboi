use crate::regs::Regs;
use crate::mmu::MMU;

pub struct CPU {
    regs: Regs,
    mmu: MMU,
    write_addr: u16,
    //clock: Clock,
    halted: bool,
}

impl CPU {
    pub fn init(romName: &str) -> CPU {
        CPU {
            regs: Regs::init(),
            mmu: MMU::init(romName),
            write_addr: 0x00,
            //clock: Clock::init(),
            halted: false,
        }
    }

    
    pub fn fetch_byte(&mut self) -> u8 {
        let opcode = self.mmu.read(self.regs.pc());        // fetch op
        self.regs.set_pc(self.regs.pc() + 1);            // pc++
        opcode
    }
    
    /*
    pub fn fetch_word(&mut self) -> u16 {
        let data = self.mmu.read_word(self.regs.pc);          // fetch data
        self.regs.set_pc(self.regs.pc() + 2);            // pc += 2
        data
    }
    */

    pub fn write_byte(&mut self, data: u8) {
        self.mmu.write(self.write_addr, data);
    }

    /*
    pub fn write_word(&mut self, data: u16) {
        self.mmu.write_word(self.write_addr, data);
    }
    */

    pub fn run(&mut self) {
        //self.clock.tick(self.cpu_cycle());
    }

    pub fn cpu_cycle(&mut self) -> u8 {
        
        //if self.halted { 4 } 

        let opcode = self.fetch_byte();
        
        //match opcode {
            //0x00 => { self.nop(); 4 },
            //0x01 => { self.ld_word(self.fetch_word, self.regs.set_bc); 12 },
            //0x02 => { self.write_addr = self.regs.bc(); self.ld_byte(self.regs.a(), self.write_byte); 8 },
            // ...
            //0x10 => { self.stop(); 4 },
            // ...
            // other instructions
        //}
        0
    }

    
    // general execute instruction functions with parameters

    // move 8 bit value from src to dest
    fn ld_byte(&mut self, src: fn() -> u8, dest: fn(u8)) {
        dest(src());
    }

    // move 16 bit value from src to dest
    fn ld_word(&mut self, src: fn() -> u16, dest: fn(u16)) {
        dest(src());
    }

    fn stop(&mut self) {
        if self.fetch_byte() == 0x00 {
            self.halted = true;
            // other stuff ?
        }
    }

    fn halt(&mut self) {
        self.halted = true;
        // other stuff ?
    }
}