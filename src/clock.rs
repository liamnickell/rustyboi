pub struct Clock {
    cycles: u128,
    // other stuff
}

impl Clock {
    pub fn init() -> Clock {
        Clock {
            cycles: 0,
            // other stuff
        }
    }

    pub fn tick(&mut self, num: u32) {
        self.cycles += num as u128;
        // other stuff
    }
}