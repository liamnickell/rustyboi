pub struct CPU {
    regs: Regs,
    mmu: MMU,
    opcode: u8,
    cycles: u8,
    clock: Clock,
    halted: bool,
}

impl CPU {
    pub fn init() -> CPU {
        CPU {
            regs: Regs::init(),
            clock: Clock::init(),
            mmu: MMU::init(clock),
            halted: false,
        }
    }

    pub fn fetch_byte(&mut self) -> u8 {
        let opcode = self.mmu.read_byte(pc);        // fetch op
        self.regs.set_pc(regs.pc() + 1);            // pc++
        opcode
    }

    pub fn fetch_word(&mut self) -> u16 {
        let data = self.mmu.read_word(pc);          // fetch data
        self.regs.set_pc(regs.pc() + 2);            // pc += 2
        data
    }

    pub fn run(&mut self) {
        let opcode = self.fetch_byte();
        match opcode {
            0x00 => self.nop(),
            0x01 => self.ld(),
            0x02 => ,
            0x03 => ,
            0x04 => ,
            0x05 => ,
            0x06 => ,
            0x07 => ,
            0x08 => ,
            0x09 => ,
            0x0a => ,
            0x0b => ,
            0x0c => ,
            0x0d => ,
            0x0e => ,
            0x0f => ,
            0x10 => self.stop(),
            // other instructions
        }
    }

    
    // general execute instruction functions with parameters

    fn nop() {
        4
    }

    // move 16 bit value from src to dest
    fn ld(src: fn() -> u16, dest: fn(u16)) {

    }

    // move 8 bit value from src to dest
    fn ld(src: fn() -> u8, dest: fn(u8)) {

    }

    fn stop() {
        if self.fetch_byte() == 0x00 {
            self.halt = true;
            // other stuff ?
        }

        4
    }

    fn halt() {
        self.halt = true;
        // other stuff ?
        4
    }
}