pub struct Regs {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,   // flags
    h: u8,
    l: u8,

    sp: u16,
    pc: u16,
}

// TODO: consider using binary values and treating flags as masks
enum Flags {
    Zero = 7,
    Sub = 6,
    HalfCarry = 5,
    Carry = 4,
}

impl Regs {
    pub fn init() -> Regs {
        Regs {
            a: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            f: 0x00,
            h: 0x00,
            l: 0x00,
            sp: 0xfffe,
            pc: 0x0100,
        }        
    }

    // get individual 8 bit reg values
    pub fn a(&self) -> u8 { self.a }
    pub fn b(&self) -> u8 { self.b }
    pub fn c(&self) -> u8 { self.c }
    pub fn d(&self) -> u8 { self.d }
    pub fn e(&self) -> u8 { self.e }
    pub fn h(&self) -> u8 { self.a }
    pub fn l(&self) -> u8 { self.l }

    // combine registers to get 16 bit reg values
    pub fn af(&self) -> u16 { ((self.a as u16) << 8) | ((self.f & 0xf0) as u16) }   // ensure lower 4 bits of flags reg are always 0
    pub fn bc(&self) -> u16 { ((self.b as u16) << 8) | (self.c as u16) }
    pub fn de(&self) -> u16 { ((self.d as u16) << 8) | (self.e as u16) }
    pub fn hl(&self) -> u16 { ((self.h as u16) << 8) | (self.l as u16) }

    // get state of cpu flags
    pub fn zflag(&self) -> bool { self.f & (1 << (Flags::Zero as u8)) }
    pub fn sflag(&self) -> bool { self.f & (1 << (Flags::Sub as u8)) }
    pub fn hflag(&self) -> bool { self.f & (1 << (Flags::HalfCarry as u8)) }
    pub fn cflag(&self) -> bool { self.f & (1 << (Flags::Carry as u8)) }

    // get pc and sp values
    pub fn sp(&self) -> u16 { self.sp }
    pub fn pc(&self) -> u16 { self.pc }

    // setting individual 8 bit reg values
    pub fn set_a(&mut self, val: u8) { self.a = val; }
    pub fn set_b(&mut self, val: u8) { self.b = val; }
    pub fn set_c(&mut self, val: u8) { self.c = val; }
    pub fn set_d(&mut self, val: u8) { self.d = val; }
    pub fn set_e(&mut self, val: u8) { self.e = val; }
    pub fn set_h(&mut self, val: u8) { self.a = val; }
    pub fn set_l(&mut self, val: u8) { self.l = val; }

    // setting 16 bit reg conjunction values
    pub fn set_af(&mut self, val: u16) { 
        self.a = (val >> 8) as u8;
        self.f = (val & 0x00f0) as u8;   // ensure lower 4 bits of flags reg are always 0
    }

    pub fn set_bc(&mut self, val: u16) { 
        self.b = (val >> 8) as u8;
        self.c = (val & 0x00ff) as u8;
    }

    pub fn set_de(&mut self, val: u16) { 
        self.d = (val >> 8) as u8;
        self.e = (val & 0x00ff) as u8;
    }

    pub fn set_hl(&mut self, val: u16) { 
        self.h = (val >> 8) as u8;
        self.l = (val & 0x00ff) as u8;
    }

    // setting cpu flags
    // TODO: test this
    pub fn set_zflag(&mut self, val: bool) { 
        if val {
            self.f |= (1 << Flags::Zero as u8);
        } else {
            self.f &= !(1 << (Flags::Zero as u8));
        }
    }

    pub fn set_sflag(&mut self, val: bool) { 
        if val {
            self.f |= (1 << Flags::Sub as u8);
        } else {
            self.f &= !(1 << (Flags::Sub as u8));
        }
    }

    pub fn set_hflag(&mut self, val: bool) { 
        if val {
            self.f |= (1 << Flags::HalfCarry as u8);
        } else {
            self.f &= !(1 << (Flags::HalfCarry as u8));
        }
    }

    pub fn set_cflag(&mut self, val: bool) { 
        if val {
            self.f |= (1 << Flags::Carry as u8);
        } else {
            self.f &= !(1 << (Flags::Carry as u8));
        }
    }

    // setting pc and sp
    pub fn set_sp(&mut self, val: u16) { self.sp = val; }
    pub fn set_pc(&mut self, val: u16) { self.pc = val; }
}
