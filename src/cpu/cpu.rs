pub struct CPU {
    register: Registers,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            register: Registers.new(),
        }
    }
    pub fn reset(&mut self) {
        self.register.reset();
    }
}
