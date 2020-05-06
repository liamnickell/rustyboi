use crate::regs::Regs;
use crate::mmu::MMU;

pub struct CPU {
    regs: Regs,
    mmu: MMU,
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

impl CPU {
    pub fn init(rom_file: &str) -> CPU {
        let mut c = CPU {
            regs: Regs::init(),
            mmu: MMU::init(rom_file),
            write_addr: 0x00,
            halted: false,
        };

        c.power_up_seq();
        c
    }

    // refer to section 2.7.1
    pub fn power_up_seq(&mut self){
        // is is auto-initialized by the instructions from 0-0x100, or do we need to initialize them?
    }

    pub fn fetch_ins_byte(&mut self) -> u8 {
        let op = self.mmu.read_byte(self.regs.pc());
        self.regs.set_pc(self.regs.pc() + 1);
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
                RegIndex::A => self.regs.set_a(self.regs.a() + 1),
                RegIndex::B => self.regs.set_b(self.regs.b() + 1),
                RegIndex::C => self.regs.set_c(self.regs.c() + 1),
                RegIndex::D => self.regs.set_d(self.regs.d() + 1),
                RegIndex::E => self.regs.set_e(self.regs.e() + 1),
                RegIndex::H => self.regs.set_h(self.regs.h() + 1),
                RegIndex::L => self.regs.set_l(self.regs.l() + 1),
                RegIndex::BC => self.regs.set_bc(self.regs.bc() + 1),
                RegIndex::DE => self.regs.set_de(self.regs.de() + 1),
                RegIndex::HL => self.regs.set_hl(self.regs.hl() + 1),
                RegIndex::SP => self.regs.set_sp(self.regs.sp() + 1),
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
                RegIndex::A => self.regs.set_a(self.regs.a() - 1),
                RegIndex::B => self.regs.set_b(self.regs.b() - 1),
                RegIndex::C => self.regs.set_c(self.regs.c() - 1),
                RegIndex::D => self.regs.set_d(self.regs.d() - 1),
                RegIndex::E => self.regs.set_e(self.regs.e() - 1),
                RegIndex::H => self.regs.set_h(self.regs.h() - 1),
                RegIndex::L => self.regs.set_l(self.regs.l() - 1),
                RegIndex::BC => self.regs.set_bc(self.regs.bc() - 1),
                RegIndex::DE => self.regs.set_de(self.regs.de() - 1),
                RegIndex::HL => self.regs.set_hl(self.regs.hl() - 1),
                RegIndex::SP => self.regs.set_sp(self.regs.sp() - 1),
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