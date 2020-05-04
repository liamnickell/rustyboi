pub struct CPU {
    // regs, fetch data, fetch addr, cycles, etc.
}

impl CPU {
    pub fn init() -> CPU {
        CPU {
            // initialize stuff
        }
    }

    pub fn next() {
        fetch();
        decode();
        add_cycles();
    }

    pub fn add_cycles() {
        // add recorded cycles
    }

    pub fn fetch() {
        // opcode = read(pc)
        // pc++
    }

    pub fn decode() {
        // Instruction ins  = match opcode { opcodes => Instruction(string, src, dest, cycles) }
    }

    // general execute instruction functions with parameters

}