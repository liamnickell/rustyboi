use crate::regs::Regs;
use crate::mmu::MMU;
use crate::gpu::GPU;
use std::num::Wrapping;

//use crate::gpu::GPU;

pub struct CPU <'a>{
    regs: Regs,
    mmu: &'a MMU,
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

enum Condition {
    Z,
    C,
    NZ,
    NC,
}

impl<'a> CPU<'a> {
    pub fn init(rom_file: &str, mmu_: &'a mut MMU) -> CPU<'a> {
        let mut c = CPU {
            regs: Regs::init(),
            mmu: mmu_,
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
            0x01 => { let data = self.fetch_ins_word(); self.ld_word(data, Some(RegIndex::BC), 0); 12 },
            0x02 => { self.ld_byte(self.regs.a(), None, self.regs.bc()); 8 },
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
            _ => { self.undefined_ins(opcode); 4 },
        }
    }

    
    // general execute instruction functions with parameters

    // move 8 bit value from src to dest
    fn ld_byte(&mut self, src: u8, reg: Option<RegIndex>, dest: u16) {
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

    // move 16 bit value from src to dest or reg
    fn ld_word(&mut self, src: u16, reg: Option<RegIndex>, dest: u16) {
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

    // add given byte to accumulator (reg A)
    fn add_byte(&mut self, src: u8, carry: bool) {
        let c = if carry { self.regs.cflag() as u8 } else { 0 };
        let old_a = self.regs.a();
        self.regs.set_a(self.regs.a().wrapping_add(src.wrapping_add(c)));

        // set flags
        self.regs.set_sflag(false);
        self.regs.set_zflag(self.regs.a() == 0);
        self.regs.set_hflag((src & 0x0f) + (old_a & 0x0f) + c > 0x0f);
        self.regs.set_cflag((src as u16) + (old_a as u16) + (c as u16) > 0xff);
    }

    // add given reg or 8-bit immediate value to given reg
    fn add_word(&mut self, src: u16, reg: RegIndex) {
        let old_val: Option<u16>;
        match reg {
            RegIndex::HL => { old_val = Some(self.regs.hl()); self.regs.set_hl(old_val.unwrap().wrapping_add(src)); },
            RegIndex::SP => { old_val = Some(self.regs.sp()); self.regs.set_sp(old_val.unwrap().wrapping_add(src)); self.regs.set_zflag(false); },
            _ => old_val = None,
        };
     
        // set flags
        self.regs.set_sflag(false);
        if let Some(v) = old_val {
            self.regs.set_hflag((src & 0x0fff) + (v & 0x0fff) > 0x0fff);
            self.regs.set_cflag((src as u32) + (v as u32) > 0xffff);
        }
    }

    // sub given byte from accumulator (reg A)
    fn sub_byte(&mut self, src: u8, carry: bool) {
        let c = if carry { self.regs.cflag() as u8 } else { 0 };
        let old_a = self.regs.a();
        self.regs.set_a(self.regs.a().wrapping_add(src.wrapping_sub(c)));

        // set flags
        self.regs.set_sflag(true);
        self.regs.set_zflag(self.regs.a() == 0);
        self.regs.set_hflag((src & 0x0f) + (old_a & 0x0f) + c > 0x0f);
        self.regs.set_cflag((old_a as u16) < (src as u16) + (c as u16));
    }

    // increment reg or given dest address or reg
    fn inc(&mut self, reg: Option<RegIndex>, dest: u16) {
        let mut old_val: Option<u8> = None;
        if let Some(r) = reg {
            match r {
                RegIndex::A => { old_val = Some(self.regs.a()); self.regs.set_a(old_val.unwrap().wrapping_add(1)); },
                RegIndex::B => { old_val = Some(self.regs.b()); self.regs.set_b(old_val.unwrap().wrapping_add(1)); },
                RegIndex::C => { old_val = Some(self.regs.c()); self.regs.set_c(old_val.unwrap().wrapping_add(1)); },
                RegIndex::D => { old_val = Some(self.regs.d()); self.regs.set_d(old_val.unwrap().wrapping_add(1)); },
                RegIndex::E => { old_val = Some(self.regs.e()); self.regs.set_e(old_val.unwrap().wrapping_add(1)); },
                RegIndex::H => { old_val = Some(self.regs.h()); self.regs.set_h(old_val.unwrap().wrapping_add(1)); },
                RegIndex::L => { old_val = Some(self.regs.l()); self.regs.set_l(old_val.unwrap().wrapping_add(1)); },
                RegIndex::BC => self.regs.set_bc(self.regs.bc().wrapping_add(1)),
                RegIndex::DE => self.regs.set_de(self.regs.de().wrapping_add(1)),
                RegIndex::HL => self.regs.set_hl(self.regs.hl().wrapping_add(1)),
                RegIndex::SP => self.regs.set_sp(self.regs.sp().wrapping_add(1)),
                _ => (),
            };
        } else {
            old_val = Some(self.mmu.read_byte(dest));
            self.mmu.write_byte(dest, old_val.unwrap().wrapping_add(1));
        }

        if let Some(v) = old_val {
            self.regs.set_sflag(false);
            self.regs.set_zflag(v.wrapping_add(1) == 0);
            self.regs.set_hflag((v & 0x0f) == 0x0f);
        }
    }

    // decrement reg or given dest address
    fn dec(&mut self, reg: Option<RegIndex>, dest: u16) {
        let mut old_val: Option<u8> = None;
        if let Some(r) = reg {
            match r {
                RegIndex::A => { old_val = Some(self.regs.a()); self.regs.set_a(old_val.unwrap().wrapping_sub(1)); },
                RegIndex::B => { old_val = Some(self.regs.b()); self.regs.set_b(old_val.unwrap().wrapping_sub(1)); },
                RegIndex::C => { old_val = Some(self.regs.c()); self.regs.set_c(old_val.unwrap().wrapping_sub(1)); },
                RegIndex::D => { old_val = Some(self.regs.d()); self.regs.set_d(old_val.unwrap().wrapping_sub(1)); },
                RegIndex::E => { old_val = Some(self.regs.e()); self.regs.set_e(old_val.unwrap().wrapping_sub(1)); },
                RegIndex::H => { old_val = Some(self.regs.h()); self.regs.set_h(old_val.unwrap().wrapping_sub(1)); },
                RegIndex::L => { old_val = Some(self.regs.l()); self.regs.set_l(old_val.unwrap().wrapping_sub(1)); },
                RegIndex::BC => self.regs.set_bc(self.regs.bc().wrapping_sub(1)),
                RegIndex::DE => self.regs.set_de(self.regs.bc().wrapping_sub(1)),
                RegIndex::HL => self.regs.set_hl(self.regs.bc().wrapping_sub(1)),
                RegIndex::SP => self.regs.set_sp(self.regs.bc().wrapping_sub(1)),
                _ => (),
            };
        } else {
            old_val = Some(self.mmu.read_byte(dest));
            self.mmu.write_byte(dest, old_val.unwrap().wrapping_sub(1));
        }

        if let Some(v) = old_val {
            self.regs.set_sflag(true);
            self.regs.set_zflag(v.wrapping_sub(1) == 0); 
            self.regs.set_hflag((v & 0x0f) == 0);
        }
    }

    fn and(&mut self, val: u8) {
        self.regs.set_a(self.regs.a() & val);

        // set flags
        self.regs.set_zflag(self.regs.a() == 0);
        self.regs.set_hflag(true);
        self.regs.set_sflag(false);
        self.regs.set_cflag(false);
    }

    fn or(&mut self, val: u8) {
        self.regs.set_a(self.regs.a() | val);

        // set flags
        self.regs.set_zflag(self.regs.a() == 0);
        self.regs.set_hflag(false);
        self.regs.set_sflag(false);
        self.regs.set_cflag(false);
    }

    fn xor(&mut self, val: u8) {
        self.regs.set_a(self.regs.a() ^ val);

        // set flags
        self.regs.set_zflag(self.regs.a() == 0);
        self.regs.set_hflag(false);
        self.regs.set_sflag(false);
        self.regs.set_cflag(false);
    }

    fn cp(&mut self, val: u8) {
        let a = self.regs.a();
        self.sub_byte(val, false);
        self.regs.set_a(a);
    }

    fn jump(&mut self, val: u16, cond: Option<Condition>) -> bool {
        match cond {
            Some(Condition::Z) => { if self.regs.zflag() { self.regs.set_pc(val); return true } false },
            Some(Condition::NZ) => { if !self.regs.zflag() { self.regs.set_pc(val); return true } false },
            Some(Condition::C) => { if self.regs.cflag() { self.regs.set_pc(val); return true } false },
            Some(Condition::NC) => { if !self.regs.cflag() { self.regs.set_pc(val); return true } false },
            None => { self.regs.set_pc(val); true },
        }
    }

    fn push() {

    }

    fn pop() {

    }

    fn call() {

    }

    fn ret() {

    }

    fn undefined_ins(&self, opcode: u8) {
        println!("Instruction {:#04x} is undefined!", opcode);
    }

    fn stop(&mut self) {
        if self.fetch_ins_byte() == 0x00 {
            // stop logic
            // i don't think this is really necessary though
        }
    }

    fn halt(&mut self) {
        self.halted = true;
        // other stuff ?
    }
}