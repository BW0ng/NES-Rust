use super::status_register::StatusRegister;

#[derive(Debug)]
pub struct Registers {
    accumulator: u8,
    x: u8,
    y: u8,
    program_counter: u16,
    stack_pointer: u8,
    status: StatusRegister,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            accumulator: 0,
            x: 0,
            y: 0,
            program_counter: 0,
            stack_pointer: 0,
            status: StatusRegister::new(),
        }
    }

    pub fn reset(&mut self) {
        self.stack_pointer = 0xFF;
        self.status().set(0x34);
    }

    pub fn increment_pc(&mut self, increment_amount: u16) -> u16 {
        self.program_counter += increment_amount;
        self.program_counter
    }
    pub fn increment_sp(&mut self, increment_amount: u8) -> u8 {
        self.stack_pointer += increment_amount;
        self.stack_pointer
    }
}

// Getters and Setters
// TODO Remove dead code linting
#[allow(dead_code)]
impl Registers {
    pub fn accumulator(&self) -> u8 {
        self.accumulator
    }
    pub fn x(&self) -> u8 {
        self.x
    }
    pub fn y(&self) -> u8 {
        self.y
    }
    pub fn program_counter(&self) -> u16 {
        self.program_counter
    }
    pub fn stack_pointer(&self) -> u8 {
        self.stack_pointer
    }
    pub fn status(&self) -> StatusRegister {
        self.status
    }

    pub fn set_accumulator(&mut self, accumulator: u8) {
        self.accumulator = accumulator
    }
    pub fn set_x(&mut self, x: u8) {
        self.x = x
    }
    pub fn set_y(&mut self, y: u8) {
        self.y = y
    }
    pub fn set_program_counter(&mut self, program_counter: u16) {
        self.program_counter = program_counter
    }
    pub fn set_stack_pointer(&mut self, stack_pointer: u8) {
        self.stack_pointer = stack_pointer
    }
    pub fn set_status(&mut self, status: StatusRegister) {
        self.status = status
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let registers = Registers::new();
        assert_eq!(0, registers.accumulator);
        assert_eq!(0, registers.x);
        assert_eq!(0, registers.y);
        assert_eq!(0, registers.program_counter);
        assert_eq!(0, registers.stack_pointer);
        assert_eq!(StatusRegister::new(), registers.status);
    }

    #[test]
    fn test_increment_pc() {
        let mut registers = Registers::new();

        assert_eq!(4, registers.increment_pc(4));
        assert_eq!(6, registers.increment_pc(2));
        assert_eq!(12, registers.increment_pc(6));
        assert_eq!(24, registers.increment_pc(12));
    }

    #[test]
    fn test_increment_sp() {
        let mut registers = Registers::new();

        assert_eq!(4, registers.increment_sp(4));
        assert_eq!(6, registers.increment_sp(2));
        assert_eq!(12, registers.increment_sp(6));
        assert_eq!(24, registers.increment_sp(12));
    }
}
