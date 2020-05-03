pub struct Registers{
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

pub struct Flags{
    C : bool, // carry
    H : bool, // half carry
    Z : bool, // is zero
    S : bool, // is substract
}