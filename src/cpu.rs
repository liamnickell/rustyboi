use crate::regs::Regs;
use crate::mmu::MMU;
use crate::gpu::GPU;
use std::num::Wrapping;

//use crate::gpu::GPU;

pub struct CPU <'a>{
    regs: Regs,
    mmu: &'a MMU,
    write_addr: u16,
    halted: bool,
}

enum RegIndex {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    
    AF,
    BC,
    DE,
    HL,

    SP,
    PC,
}

impl<'a> CPU<'a> {
    pub fn init(rom_file: &str, mmu_: &'a mut MMU) -> CPU<'a> {
        let mut c = CPU {
            regs: Regs::init(),
            mmu: mmu_,
            write_addr: 0x00,
            halted: false,
        };

        //c.power_up_seq();
        c
    }

    // refer to section 2.7.1
    pub fn power_up_seq(&mut self){
        // is is auto-initialized by the instructions from 0-0x100, or do we need to initialize them?
    }

    pub fn fetch_ins_byte(&mut self) -> u8 {
        let op = self.mmu.read_byte(self.regs.pc());
        self.regs.set_pc(self.regs.pc() + 1);
        println!("ins: {:#12b}", op);
        op
    }

    pub fn fetch_ins_word(&mut self) -> u16 {
        let word = self.mmu.read_word(self.regs.pc());
        self.regs.set_pc(self.regs.pc() + 2);
        word
    }

    pub fn cpu_cycle(&mut self) -> u32 {
        if self.halted { return 4 } 
        let opcode = self.fetch_ins_byte();
        
        match opcode {
            0x00 => { 4 },
            0x01 => { let data = self.fetch_ins_word(); self.ld_word(data, 0, Some(RegIndex::BC)); 12 },
            0x02 => { self.ld_byte(self.regs.a(), self.regs.bc(), None); 8 },
            0x03 => { self.inc(Some(RegIndex::BC), 0); 8 },
            0x04 => { self.inc(Some(RegIndex::B), 0); 4 },
            0x05 => { self.dec(Some(RegIndex::B), 0); 4 },
            // ...
            0x10 => { self.stop(); 4 },
            // ...
            0x0b => { self.dec(Some(RegIndex::BC), 0); 8 },
            0x0d => { self.dec(Some(RegIndex::C), 0); 4 },
            // ...
            0x15 => { self.dec(Some(RegIndex::D), 0); 4 },
            0x1b => { self.dec(Some(RegIndex::DE), 0); 8 },
            0x1d => { self.dec(Some(RegIndex::E), 0); 4 },
            // ...
            0x25 => { self.dec(Some(RegIndex::H), 0); 4 },
            0x2b => { self.dec(Some(RegIndex::HL), 0); 8 },
            0x2d => { self.dec(Some(RegIndex::L), 0); 4 },
            // ...
            0x35 => { self.dec(None, self.regs.hl()); 12 },
            0x3b => { self.dec(Some(RegIndex::SP), 0); 8 },
            0x3d => { self.dec(Some(RegIndex::A), 0); 4 },
            // ...
            // other instructions
            _ => { 0 },
        }
    }

    
    // general execute instruction functions with parameters

    // move 8 bit value from src to dest
    fn ld_byte(&mut self, src: u8, dest: u16, reg: Option<RegIndex>) {
        if let Some(r) = reg {
            match r {
                RegIndex::A => self.regs.set_a(src),
                RegIndex::B => self.regs.set_b(src),
                RegIndex::C => self.regs.set_c(src),
                RegIndex::D => self.regs.set_d(src),
                RegIndex::E => self.regs.set_e(src),
                RegIndex::H => self.regs.set_h(src),
                RegIndex::L => self.regs.set_l(src),
                _ => (),
            };
        } else {
            self.mmu.write_byte(dest, src);
        }
    }

    // move 16 bit value from src to dest
    fn ld_word(&mut self, src: u16, dest: u16, reg: Option<RegIndex>) {
        if let Some(r) = reg {
            match r {
                RegIndex::BC => self.regs.set_bc(src),
                RegIndex::DE => self.regs.set_de(src),
                RegIndex::HL => self.regs.set_hl(src),
                RegIndex::SP => self.regs.set_sp(src),
                _ => (),
            };
        } else {
            self.mmu.write_word(dest, src);
        }
    }

    // increment reg or given dest address
    // TODO: add flag updates
    fn inc(&mut self, reg: Option<RegIndex>, dest: u16) {
        if let Some(r) = reg {
            match r {
                RegIndex::A => self.regs.set_b((Wrapping(self.regs.b()) + Wrapping(1)).0),
                RegIndex::B => self.regs.set_b((Wrapping(self.regs.b()) + Wrapping(1)).0),
                RegIndex::C => self.regs.set_b((Wrapping(self.regs.b()) + Wrapping(1)).0),
                RegIndex::D => self.regs.set_b((Wrapping(self.regs.b()) + Wrapping(1)).0),
                RegIndex::E => self.regs.set_b((Wrapping(self.regs.b()) + Wrapping(1)).0),
                RegIndex::H => self.regs.set_b((Wrapping(self.regs.b()) + Wrapping(1)).0),
                RegIndex::L => self.regs.set_b((Wrapping(self.regs.b()) + Wrapping(1)).0),
                RegIndex::BC => self.regs.set_bc((Wrapping(self.regs.bc()) + Wrapping(1)).0),
                RegIndex::DE => self.regs.set_de((Wrapping(self.regs.bc()) + Wrapping(1)).0),
                RegIndex::HL => self.regs.set_hl((Wrapping(self.regs.bc()) + Wrapping(1)).0),
                RegIndex::SP => self.regs.set_sp((Wrapping(self.regs.bc()) + Wrapping(1)).0),
                _ => (),
            };
        } else {
            let value = self.mmu.read_word(dest) + 1;
            self.mmu.write_word(dest, value);
        }
    }

    // decrement reg or given dest address
    // TODO: add flag updates
    fn dec(&mut self, reg: Option<RegIndex>, dest: u16) {
        if let Some(r) = reg {
            match r {
                RegIndex::A => self.regs.set_b((Wrapping(self.regs.b()) - Wrapping(1)).0),
                RegIndex::B => self.regs.set_b((Wrapping(self.regs.b()) - Wrapping(1)).0),
                RegIndex::C => self.regs.set_b((Wrapping(self.regs.b()) - Wrapping(1)).0),
                RegIndex::D => self.regs.set_b((Wrapping(self.regs.b()) - Wrapping(1)).0),
                RegIndex::E => self.regs.set_b((Wrapping(self.regs.b()) - Wrapping(1)).0),
                RegIndex::H => self.regs.set_b((Wrapping(self.regs.b()) - Wrapping(1)).0),
                RegIndex::L => self.regs.set_b((Wrapping(self.regs.b()) - Wrapping(1)).0),
                RegIndex::BC => self.regs.set_bc((Wrapping(self.regs.bc()) - Wrapping(1)).0),
                RegIndex::DE => self.regs.set_de((Wrapping(self.regs.bc()) - Wrapping(1)).0),
                RegIndex::HL => self.regs.set_hl((Wrapping(self.regs.bc()) - Wrapping(1)).0),
                RegIndex::SP => self.regs.set_sp((Wrapping(self.regs.bc()) - Wrapping(1)).0),
                _ => (),
            };
        } else {
            let value = self.mmu.read_word(dest) - 1;
            self.mmu.write_word(dest, value);
        }
    }

    fn stop(&mut self) {
        if self.fetch_ins_byte() == 0x00 {
            self.halted = true;
            // other stuff ?
        }
    }

    fn halt(&mut self) {
        self.halted = true;
        // other stuff ?
    }
}