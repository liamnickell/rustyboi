use crate::regs::Regs;
use crate::mmu::MMU;
use crate::gpu::GPU;
use std::num::Wrapping;

//use crate::gpu::GPU;

pub struct CPU {
    regs: Regs,
    mmu: MMU,
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

impl CPU {
    pub fn init(rom_file: &str) -> CPU {
        let mut c = CPU {
            regs: Regs::init(),
            mmu: MMU::init(rom_file, "Roms/DMG_ROM.bin"),
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
        println!("ins: {:#12b}", op);
        op
    }

    pub fn fetch_ins_word(&mut self) -> u16 {
        let word = self.mmu.read_word(self.regs.pc());
        self.regs.set_pc(self.regs.pc() + 2);
        word
    }

    pub fn cpu_cycle(&mut self) -> u8 {
        if self.halted { return 1 } 
        let op = self.fetch_ins_byte();
        
        match op {
            0x00 => { 1 },
            0x01 => { let data = self.fetch_ins_word(); self.ld_word(data, Some(RegIndex::BC), 0); 3 },
            0x02 => { self.ld_byte(self.regs.a(), None, self.regs.bc()); 2 },
            0x03 => { self.inc(Some(RegIndex::BC), 0); 2 },
            0x04 => { self.inc(Some(RegIndex::B), 0); 1 },
            0x05 => { self.dec(Some(RegIndex::B), 0); 1 },
            0x06 => { let data = self.fetch_ins_byte(); self.ld_byte(data, Some(RegIndex::B), 0); 2 },
            0x07 => { self.rlca(); 1 },
            0x08 => { let addr = self.fetch_ins_word(); self.ld_word(self.regs.sp(), None, addr); 5 },
            0x09 => { self.add_word(self.regs.bc(), RegIndex::HL); 2 },
            0x0a => { let data = self.mmu.read_byte(self.regs.bc()); self.ld_byte(data, Some(RegIndex::A), 0); 2 },
            0x0b => { self.dec(Some(RegIndex::BC), 0); 2 },
            0x0c => { self.inc(Some(RegIndex::C), 0); 1 },
            0x0d => { self.dec(Some(RegIndex::C), 0); 1 },
            0x0e => { let data = self.fetch_ins_byte(); self.ld_byte(data, Some(RegIndex::C), 0); 2 },
            0x0f => { self.rrca(); 1 },
            0x10 => { self.stop(); 1 },
            0x11 => { let data = self.fetch_ins_word(); self.ld_word(data, Some(RegIndex::DE), 0); 3 },
            0x12 => { self.ld_byte(self.regs.a(), None, self.regs.de()); 2 },
            0x13 => { self.inc(Some(RegIndex::DE), 0); 2 },
            0x14 => { self.inc(Some(RegIndex::D), 0); 1 },
            0x15 => { self.dec(Some(RegIndex::D), 0); 1 },
            0x16 => { let data = self.fetch_ins_byte(); self.ld_byte(data, Some(RegIndex::D), 0); 2 },
            0x17 => { self.rla(); 1 },
            0x18 => { self.jr(None); 3 },
            0x19 => { self.add_word(self.regs.de(), RegIndex::HL); 2 },
            0x1a => { let data = self.mmu.read_byte(self.regs.de()); self.ld_byte(data, Some(RegIndex::A), 0); 2 },
            0x1b => { self.dec(Some(RegIndex::DE), 0); 2 },
            0x1c => { self.inc(Some(RegIndex::E), 0); 1 },
            0x1d => { self.dec(Some(RegIndex::E), 0); 1 },
            0x1e => { let data = self.fetch_ins_byte(); self.ld_byte(data, Some(RegIndex::E), 0); 2 },
            0x1f => { self.rra(); 1 },
            0x20 => { self.jr(Some(Condition::NZ)) as u8 + 2 },
            0x21 => { let data = self.fetch_ins_word(); self.ld_word(data, Some(RegIndex::HL), 0); 3 },
            0x22 => { self.ld_byte(self.regs.a(), None, self.regs.hl()); self.regs.set_hl(self.regs.hl().wrapping_add(1)); 2 },
            0x23 => { self.inc(Some(RegIndex::HL), 0); 2 },
            0x24 => { self.inc(Some(RegIndex::H), 0); 1 },
            0x25 => { self.dec(Some(RegIndex::H), 0); 1 },
            0x26 => { let data = self.fetch_ins_byte(); self.ld_byte(data, Some(RegIndex::H), 0); 2 },
            0x27 => { self.daa(); 1 },
            0x28 => { self.jr(Some(Condition::Z)) as u8 + 2 },
            0x29 => { self.add_word(self.regs.hl(), RegIndex::HL); 2 },
            0x2a => { let data = self.mmu.read_byte(self.regs.hl()); self.ld_byte(data, Some(RegIndex::A), 0); self.regs.set_hl(self.regs.hl().wrapping_add(1)); 2 },
            0x2b => { self.dec(Some(RegIndex::HL), 0); 2 },
            0x2c => { self.inc(Some(RegIndex::L), 0); 1 },
            0x2d => { self.dec(Some(RegIndex::L), 0); 1 },
            0x2e => { let data = self.fetch_ins_byte(); self.ld_byte(data, Some(RegIndex::L), 0); 2 },
            0x2f => { self.cpl(); 1 },
            0x30 => { self.jr(Some(Condition::NC)) as u8 + 2 },
            0x31 => { let data = self.fetch_ins_word(); self.ld_word(data, Some(RegIndex::SP), 0); 3 },
            0x32 => { self.ld_byte(self.regs.a(), None, self.regs.hl()); self.regs.set_hl(self.regs.hl().wrapping_sub(1)); 2 },
            0x33 => { self.inc(Some(RegIndex::SP), 0); 2 },
            0x34 => { self.inc(None, self.regs.hl()); 3 },
            0x35 => { self.dec(None, self.regs.hl()); 3 },
            0x36 => { let data = self.fetch_ins_byte(); self.ld_byte(data, None, self.regs.hl()); 3 },
            0x37 => { self.scf(); 1 },
            0x38 => { self.jr(Some(Condition::C)) as u8 + 2 },
            0x39 => { self.add_word(self.regs.sp(), RegIndex::HL); 2 },
            0x3a => { let data = self.mmu.read_byte(self.regs.hl()); self.ld_byte(data, Some(RegIndex::A), 0); self.regs.set_hl(self.regs.hl().wrapping_sub(1)); 2 },
            0x3b => { self.dec(Some(RegIndex::SP), 0); 2 },
            0x3c => { self.inc(Some(RegIndex::A), 0); 1 },
            0x3d => { self.dec(Some(RegIndex::A), 0); 1 },
            0x3e => { let data = self.fetch_ins_byte(); self.ld_byte(data, Some(RegIndex::A), 0); 2 },
            0x3f => { self.ccf(); 1 },
            0x40 => { self.ld_byte(self.regs.b(), Some(RegIndex::B), 0); 1 },
            0x41 => { self.ld_byte(self.regs.c(), Some(RegIndex::B), 0); 1 },
            0x42 => { self.ld_byte(self.regs.d(), Some(RegIndex::B), 0); 1 },
            0x43 => { self.ld_byte(self.regs.e(), Some(RegIndex::B), 0); 1 },
            0x44 => { self.ld_byte(self.regs.h(), Some(RegIndex::B), 0); 1 },
            0x45 => { self.ld_byte(self.regs.l(), Some(RegIndex::B), 0); 1 },
            0x46 => { let data = self.mmu.read_byte(self.regs.hl()); self.ld_byte(data, Some(RegIndex::B), 0); 2 },
            0x47 => { self.ld_byte(self.regs.a(), Some(RegIndex::B), 0); 1 },
            0x48 => { self.ld_byte(self.regs.b(), Some(RegIndex::C), 0); 1 },
            0x49 => { self.ld_byte(self.regs.c(), Some(RegIndex::C), 0); 1 },
            0x4a => { self.ld_byte(self.regs.d(), Some(RegIndex::C), 0); 1 },
            0x4b => { self.ld_byte(self.regs.e(), Some(RegIndex::C), 0); 1 },
            0x4c => { self.ld_byte(self.regs.h(), Some(RegIndex::C), 0); 1 },
            0x4d => { self.ld_byte(self.regs.l(), Some(RegIndex::C), 0); 1 },
            0x4e => { let data = self.mmu.read_byte(self.regs.hl()); self.ld_byte(data, Some(RegIndex::C), 0); 2 },
            0x4f => { self.ld_byte(self.regs.a(), Some(RegIndex::C), 0); 1 },
            0x50 => { self.ld_byte(self.regs.b(), Some(RegIndex::D), 0); 1 },
            0x51 => { self.ld_byte(self.regs.c(), Some(RegIndex::D), 0); 1 },
            0x52 => { self.ld_byte(self.regs.d(), Some(RegIndex::D), 0); 1 },
            0x53 => { self.ld_byte(self.regs.e(), Some(RegIndex::D), 0); 1 },
            0x54 => { self.ld_byte(self.regs.h(), Some(RegIndex::D), 0); 1 },
            0x55 => { self.ld_byte(self.regs.l(), Some(RegIndex::D), 0); 1 },
            0x56 => { let data = self.mmu.read_byte(self.regs.hl()); self.ld_byte(data, Some(RegIndex::D), 0); 2 },
            0x57 => { self.ld_byte(self.regs.a(), Some(RegIndex::D), 0); 1 },
            0x58 => { self.ld_byte(self.regs.b(), Some(RegIndex::E), 0); 1 },
            0x59 => { self.ld_byte(self.regs.c(), Some(RegIndex::E), 0); 1 },
            0x5a => { self.ld_byte(self.regs.d(), Some(RegIndex::E), 0); 1 },
            0x5b => { self.ld_byte(self.regs.e(), Some(RegIndex::E), 0); 1 },
            0x5c => { self.ld_byte(self.regs.h(), Some(RegIndex::E), 0); 1 },
            0x5d => { self.ld_byte(self.regs.l(), Some(RegIndex::E), 0); 1 },
            0x5e => { let data = self.mmu.read_byte(self.regs.hl()); self.ld_byte(data, Some(RegIndex::E), 0); 2 },
            0x5f => { self.ld_byte(self.regs.a(), Some(RegIndex::E), 0); 1 },
            0x60 => { self.ld_byte(self.regs.b(), Some(RegIndex::H), 0); 1 },
            0x61 => { self.ld_byte(self.regs.c(), Some(RegIndex::H), 0); 1 },
            0x62 => { self.ld_byte(self.regs.d(), Some(RegIndex::H), 0); 1 },
            0x63 => { self.ld_byte(self.regs.e(), Some(RegIndex::H), 0); 1 },
            0x64 => { self.ld_byte(self.regs.h(), Some(RegIndex::H), 0); 1 },
            0x65 => { self.ld_byte(self.regs.l(), Some(RegIndex::H), 0); 1 },
            0x66 => { let data = self.mmu.read_byte(self.regs.hl()); self.ld_byte(data, Some(RegIndex::H), 0); 2 },
            0x67 => { self.ld_byte(self.regs.a(), Some(RegIndex::H), 0); 1 },
            0x68 => { self.ld_byte(self.regs.b(), Some(RegIndex::L), 0); 1 },
            0x69 => { self.ld_byte(self.regs.c(), Some(RegIndex::L), 0); 1 },
            0x6a => { self.ld_byte(self.regs.d(), Some(RegIndex::L), 0); 1 },
            0x6b => { self.ld_byte(self.regs.e(), Some(RegIndex::L), 0); 1 },
            0x6c => { self.ld_byte(self.regs.h(), Some(RegIndex::L), 0); 1 },
            0x6d => { self.ld_byte(self.regs.l(), Some(RegIndex::L), 0); 1 },
            0x6e => { let data = self.mmu.read_byte(self.regs.hl()); self.ld_byte(data, Some(RegIndex::L), 0); 2 },
            0x6f => { self.ld_byte(self.regs.a(), Some(RegIndex::L), 0); 1 },
            0x70 => { self.ld_byte(self.regs.b(), None, self.regs.hl()); 2 },
            0x71 => { self.ld_byte(self.regs.c(), None, self.regs.hl()); 2 },
            0x72 => { self.ld_byte(self.regs.d(), None, self.regs.hl()); 2 },
            0x73 => { self.ld_byte(self.regs.e(), None, self.regs.hl()); 2 },
            0x74 => { self.ld_byte(self.regs.h(), None, self.regs.hl()); 2 },
            0x75 => { self.ld_byte(self.regs.l(), None, self.regs.hl()); 2 },
            0x76 => { self.halt(); 1 },
            0x77 => { self.ld_byte(self.regs.a(), None, self.regs.hl()); 2 },
            0x78 => { self.ld_byte(self.regs.b(), Some(RegIndex::A), 0); 1 },
            0x79 => { self.ld_byte(self.regs.c(), Some(RegIndex::A), 0); 1 },
            0x7a => { self.ld_byte(self.regs.d(), Some(RegIndex::A), 0); 1 },
            0x7b => { self.ld_byte(self.regs.e(), Some(RegIndex::A), 0); 1 },
            0x7c => { self.ld_byte(self.regs.h(), Some(RegIndex::A), 0); 1 },
            0x7d => { self.ld_byte(self.regs.l(), Some(RegIndex::A), 0); 1 },
            0x7e => { let data = self.mmu.read_byte(self.regs.hl()); self.ld_byte(data, Some(RegIndex::A), 0); 2 },
            0x7f => { self.ld_byte(self.regs.a(), Some(RegIndex::A), 0); 1 },
            0x80 => { self.add_byte(self.regs.b(), false); 1 },
            0x81 => { self.add_byte(self.regs.c(), false); 1 },
            0x82 => { self.add_byte(self.regs.d(), false); 1 },
            0x83 => { self.add_byte(self.regs.e(), false); 1 },
            0x84 => { self.add_byte(self.regs.h(), false); 1 },
            0x85 => { self.add_byte(self.regs.l(), false); 1 },
            0x86 => { let data = self.mmu.read_byte(self.regs.hl()); self.add_byte(data, false); 2 },
            0x87 => { self.add_byte(self.regs.a(), false); 1 },
            0x88 => { self.add_byte(self.regs.b(), true); 1 },
            0x89 => { self.add_byte(self.regs.c(), true); 1 },
            0x8a => { self.add_byte(self.regs.d(), true); 1 },
            0x8b => { self.add_byte(self.regs.e(), true); 1 },
            0x8c => { self.add_byte(self.regs.h(), true); 1 },
            0x8d => { self.add_byte(self.regs.l(), true); 1 },
            0x8e => { let data = self.mmu.read_byte(self.regs.hl()); self.add_byte(data, true); 2 },
            0x8f => { self.add_byte(self.regs.a(), true); 1 },
            0x90 => { self.sub_byte(self.regs.b(), false); 1 },
            0x91 => { self.sub_byte(self.regs.c(), false); 1 },
            0x92 => { self.sub_byte(self.regs.d(), false); 1 },
            0x93 => { self.sub_byte(self.regs.e(), false); 1 },
            0x94 => { self.sub_byte(self.regs.h(), false); 1 },
            0x95 => { self.sub_byte(self.regs.l(), false); 1 },
            0x96 => { let data = self.mmu.read_byte(self.regs.hl()); self.sub_byte(data, false); 2 },
            0x97 => { self.sub_byte(self.regs.a(), false); 1 },
            0x98 => { self.sub_byte(self.regs.b(), true); 1 },
            0x99 => { self.sub_byte(self.regs.c(), true); 1 },
            0x9a => { self.sub_byte(self.regs.d(), true); 1 },
            0x9b => { self.sub_byte(self.regs.e(), true); 1 },
            0x9c => { self.sub_byte(self.regs.h(), true); 1 },
            0x9d => { self.sub_byte(self.regs.l(), true); 1 },
            0x9e => { let data = self.mmu.read_byte(self.regs.hl()); self.sub_byte(data, true); 2 },
            0x9f => { self.sub_byte(self.regs.a(), true); 1 },
            0xa0 => { self.and(self.regs.b()); 1 },
            0xa1 => { self.and(self.regs.c()); 1 },
            0xa2 => { self.and(self.regs.d()); 1 },
            0xa3 => { self.and(self.regs.e()); 1 },
            0xa4 => { self.and(self.regs.h()); 1 },
            0xa5 => { self.and(self.regs.l()); 1 },
            0xa6 => { let data = self.mmu.read_byte(self.regs.hl()); self.and(data); 2 },
            0xa7 => { self.and(self.regs.a()); 1 },
            0xa8 => { self.xor(self.regs.b()); 1 },
            0xa9 => { self.xor(self.regs.c()); 1 },
            0xaa => { self.xor(self.regs.d()); 1 },
            0xab => { self.xor(self.regs.e()); 1 },
            0xac => { self.xor(self.regs.h()); 1 },
            0xad => { self.xor(self.regs.l()); 1 },
            0xae => { let data = self.mmu.read_byte(self.regs.hl()); self.xor(data); 2 },
            0xaf => { self.xor(self.regs.a()); 1 },
            0xb0 => { self.or(self.regs.b()); 1 },
            0xb1 => { self.or(self.regs.c()); 1 },
            0xb2 => { self.or(self.regs.d()); 1 },
            0xb3 => { self.or(self.regs.e()); 1 },
            0xb4 => { self.or(self.regs.h()); 1 },
            0xb5 => { self.or(self.regs.l()); 1 },
            0xb6 => { let data = self.mmu.read_byte(self.regs.hl()); self.or(data); 2 },
            0xb7 => { self.or(self.regs.a()); 1 },
            0xb8 => { self.cp(self.regs.b()); 1 },
            0xb9 => { self.cp(self.regs.c()); 1 },
            0xba => { self.cp(self.regs.d()); 1 },
            0xbb => { self.cp(self.regs.e()); 1 },
            0xbc => { self.cp(self.regs.h()); 1 },
            0xbd => { self.cp(self.regs.l()); 1 },
            0xbe => { let data = self.mmu.read_byte(self.regs.hl()); self.cp(data); 2 },
            0xbf => { self.cp(self.regs.a()); 1 },
            0xc0 => { self.ret(Some(Condition::NZ)) as u8 * 3 + 2 },
            0xc1 => { self.pop(RegIndex::BC); 3 },
            0xc2 => { let addr = self.fetch_ins_word(); self.jump(addr, Some(Condition::NZ)) as u8 + 3 },
            0xc3 => { let addr = self.fetch_ins_word(); self.jump(addr, None); 4 },
            0xc4 => { self.call(Some(Condition::NZ)) as u8 * 3 + 3 },
            0xc5 => { self.push(self.regs.bc()); 4 },
            0xc6 => { let data = self.fetch_ins_byte(); self.add_byte(data, false); 2 },
            0xc7 => { self.rst(0x00 as u16); 4 },
            0xc8 => { self.ret(Some(Condition::Z)) as u8 * 3 + 2 },
            0xc9 => { self.ret(None); 4 },
            0xca => { let addr = self.fetch_ins_word(); self.jump(addr, Some(Condition::Z)) as u8 + 3 },
            0xcb => { self.decode_cb() + 1 },
            0xcc => { self.call(Some(Condition::Z)) as u8 * 3 + 3 },
            0xcd => { self.call(None); 6 },
            0xce => { let data = self.fetch_ins_byte(); self.add_byte(data, true); 2 },
            0xcf => { self.rst(0x08 as u16); 4 },
            0xd0 => { self.ret(Some(Condition::NC)) as u8 * 3 + 2 },
            0xd1 => { self.pop(RegIndex::DE); 3 },
            0xd2 => { let addr = self.fetch_ins_word(); self.jump(addr, Some(Condition::NC)) as u8 + 3 },
            0xd3 => { self.undefined_op(op); 1 },
            0xd4 => { self.call(Some(Condition::NC)) as u8 * 3 + 3 },
            0xd5 => { self.push(self.regs.de()); 4 },
            0xd6 => { let data = self.fetch_ins_byte(); self.sub_byte(data, false); 2 },
            0xd7 => { self.rst(0x10 as u16); 4 },
            0xd8 => { self.ret(Some(Condition::C)) as u8 * 5 + 2 },
            0xd9 => { self.ret(None); 4 },
            0xda => { let addr = self.fetch_ins_word(); self.jump(addr, Some(Condition::C)) as u8 + 3 },
            0xdb => { self.undefined_op(op); 1 },
            0xdc => { self.call(Some(Condition::C)) as u8 * 3 + 3 },
            0xdd => { self.undefined_op(op); 1 },
            0xde => { let data = self.fetch_ins_byte(); self.sub_byte(data, true); 2 },
            0xdf => { self.rst(0x18 as u16); 4 },
            0xe0 => { let addr = self.fetch_ins_byte() as u16 + 0xff00; self.ld_byte(self.regs.a(), None, addr); 3 },
            0xe1 => { self.pop(RegIndex::HL); 3 },
            0xe2 => { self.ld_byte(self.regs.a(), None, self.regs.c() as u16 + 0xff00); 2 },
            0xe3 => { self.undefined_op(op); 1 },
            0xe4 => { self.undefined_op(op); 1 },
            0xe5 => { self.push(self.regs.hl()); 4 },
            0xe6 => { let data = self.fetch_ins_byte(); self.and(data); 2 },
            0xe7 => { self.rst(0x20 as u16); 4 },
            0xe8 => { let data = self.fetch_ins_byte() as i8 as i16 as u16; self.add_word(data, RegIndex::SP); 4 },
            0xe9 => { self.jump(self.regs.hl(), None); 1 },
            0xea => { let addr = self.fetch_ins_word(); self.ld_byte(self.regs.a(), None, addr); 4 },
            0xeb => { self.undefined_op(op); 1 },
            0xec => { self.undefined_op(op); 1 },
            0xed => { self.undefined_op(op); 1 },
            0xee => { let data = self.fetch_ins_byte(); self.xor(data); 2 },
            0xef => { self.rst(0x28 as u16); 4 },
            0xf0 => { let addr = self.fetch_ins_byte() as u16 + 0xff00; let data = self.mmu.read_byte(addr); self.ld_byte(data, Some(RegIndex::A), 0); 3 },
            0xf1 => { self.pop(RegIndex::AF); 3 },
            0xf2 => { let addr = self.regs.c() as u16 + 0xff00; let data = self.mmu.read_byte(addr); self.ld_byte(data, Some(RegIndex::A), 0); 2 },
            0xf3 => { self.di(); 1 },
            0xf4 => { self.undefined_op(op); 1 },
            0xf5 => { self.push(self.regs.af()); 4 },
            0xf6 => { let data = self.fetch_ins_byte(); self.or(data); 2 },
            0xf7 => { self.rst(0x30 as u16); 4 },
            0xf8 => { let data = self.fetch_ins_byte() as i8 as i16 as u16; let old_sp = self.regs.sp(); self.add_word(data, RegIndex::SP); self.ld_word(self.regs.sp(), Some(RegIndex::HL), 0); self.regs.set_sp(old_sp); self.regs.set_zflag(false); 3 },
            0xf9 => { self.ld_word(self.regs.hl(), Some(RegIndex::SP), 0); 2 },
            0xfa => { let addr = self.fetch_ins_word(); let data = self.mmu.read_byte(addr); self.ld_byte(data, Some(RegIndex::A), 0); 4 },
            0xfb => { self.ei(); 1 },
            0xfc => { self.undefined_op(op); 1 },
            0xfd => { self.undefined_op(op); 1 },
            0xfe => { let data = self.fetch_ins_byte(); self.cp(data); 2 },
            0xff => { self.rst(0x38 as u16); 4 },
            _ => { self.undefined_op(op); 1 },
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

    fn jr(&mut self, cond: Option<Condition>) -> bool {
        let signed_val = self.fetch_ins_byte() as i8 as i16 as u16;
        let jump_loc = self.regs.pc().wrapping_add(signed_val);
        
        self.jump(jump_loc, cond)
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

    fn push(&mut self, data: u16) {
        self.regs.set_sp(self.regs.sp() - 2);
        self.mmu.write_word(self.regs.sp(), data);
    }

    fn pop(&mut self, reg: RegIndex) {
        let data = self.mmu.read_word(self.regs.sp());
        self.regs.set_sp(self.regs.sp() + 2);
        
        match reg {
            RegIndex::AF => self.regs.set_af(data),
            RegIndex::BC => self.regs.set_bc(data),
            RegIndex::DE => self.regs.set_de(data),
            RegIndex::HL => self.regs.set_hl(data),
            RegIndex::PC => self.regs.set_pc(data),
            _ => (),
        }
    }

    fn call(&mut self, cond: Option<Condition>) -> bool {
        let addr = self.fetch_ins_word();
        let didCall;
        match cond {
            Some(Condition::Z) => { didCall = self.regs.zflag(); },
            Some(Condition::NZ) => { didCall = !self.regs.zflag(); },
            Some(Condition::C) => { didCall = self.regs.cflag(); },
            Some(Condition::NC) => { didCall = !self.regs.cflag(); },
            None => didCall = true,
        };

        if didCall {
            self.push(self.regs.pc());
            self.regs.set_pc(addr);
        }

        didCall
    }

    fn ret(&mut self, cond: Option<Condition>) -> bool {
        let didRet;
        match cond {
            Some(Condition::Z) => { didRet = self.regs.zflag(); },
            Some(Condition::NZ) => { didRet = !self.regs.zflag(); },
            Some(Condition::C) => { didRet = self.regs.cflag(); },
            Some(Condition::NC) => { didRet = !self.regs.cflag(); },
            None => didRet = true,
        };

        if didRet {
            self.pop(RegIndex::PC);
        }

        // add interrupt stuff later to implement reti instruction

        didRet
    }

    fn rst(&mut self, addr: u16) {
        self.push(self.regs.pc());
        self.regs.set_pc(addr);
    }

    // rotate reg A left, putting the previous high order bit into carry flag
    fn rlca(&mut self) {
        let bit7 = self.regs.a() & (1 << 7) > 0;
        let new_a = (self.regs.a() << 1) | bit7 as u8;

        self.regs.set_a(new_a);
        self.regs.set_cflag(bit7);
    }

    // rotate reg A and carry flag together left
    fn rla(&mut self) {
        let bit7 = self.regs.a() & (1 << 7) > 0;
        let new_a = (self.regs.a() << 1) | self.regs.cflag() as u8;

        self.regs.set_a(new_a);
        self.regs.set_cflag(bit7);
    }

    // rotate reg A right, putting the previous high order bit into carry flag
    fn rrca(&mut self) {
        let bit0 = self.regs.a() & 1 > 0;
        let new_a = (self.regs.a() >> 1) | ((bit0 as u8) << 7);

        self.regs.set_a(new_a);
        self.regs.set_cflag(bit0);
    }

    // rotate reg A and carry flag together right
    fn rra(&mut self) {
        let bit0 = self.regs.a() & 1 > 0;
        let new_a = (self.regs.a() >> 1) | ((self.regs.cflag() as u8) << 7);

        self.regs.set_a(new_a);
        self.regs.set_cflag(bit0);
    }

    // converts reg A into a binary coded decimal number
    fn daa(&mut self) {
        
    }

    // set reg A to its complement
    fn cpl(&mut self) {
        self.regs.set_a(self.regs.a() ^ 0xff);

        self.regs.set_hflag(true);
        self.regs.set_sflag(true);
    }

    // set carry flag
    fn scf(&mut self) {
        self.regs.set_cflag(true);
        self.regs.set_hflag(false);
        self.regs.set_sflag(false);
    }

    // set carry flag to its complement (flip bit)
    fn ccf(&mut self) {
        self.regs.set_cflag(!self.regs.cflag());
        self.regs.set_hflag(false);
        self.regs.set_sflag(false);
    }

    fn decode_cb(&mut self) -> u8 {
        let op = self.fetch_ins_byte();
        match op {

            _ => { self.undefined_op(op); 1 },
        }
    }

    fn undefined_op(&self, op: u8) {
        println!("Instruction {:#04x} is undefined!", op);
    }

    // enable interrupts
    fn ei(&mut self) {

    }

    // disable interrupts
    fn di(&mut self) {

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