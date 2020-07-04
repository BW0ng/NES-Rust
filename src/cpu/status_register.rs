#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StatusRegister {
    negative: bool,
    overflow: bool,
    b_flag_2: bool,
    b_flag_1: bool,
    decimal: bool,
    interrupt_disable: bool,
    zero: bool,
    carry: bool,
}

impl StatusRegister {
    pub fn new() -> StatusRegister {
        StatusRegister {
            negative: false,
            overflow: false,
            b_flag_2: false,
            b_flag_1: false,
            decimal: false,
            interrupt_disable: false,
            zero: false,
            carry: false,
        }
    }

    pub fn reset(&mut self) {
        self.negative = false;
        self.overflow = false;
        self.b_flag_2 = false;
        self.b_flag_1 = false;
        self.decimal = false;
        self.interrupt_disable = false;
        self.zero = false;
        self.carry = false;
    }

    pub fn set(&mut self, x: u8) {
        self.negative = ((x & 0b1000_0000) >> 7) == 1;
        self.overflow = ((x & 0b0100_0000) >> 6) == 1;
        self.b_flag_2 = ((x & 0b0010_0000) >> 5) == 1;
        self.b_flag_1 = ((x & 0b0001_0000) >> 4) == 1;
        self.decimal = ((x & 0b0000_1000) >> 3) == 1;
        self.interrupt_disable = ((x & 0b0000_0100) >> 2) == 1;
        self.zero = ((x & 0b0000_0010) >> 1) == 1;
        self.carry = (x & 0b0000_0001) == 1;
    }
}

// Getters and Setters
// TODO Remove dead code linting
#[allow(dead_code)]
impl StatusRegister {
    pub fn negative(&self) -> bool {
        self.negative
    }
    pub fn overflow(&self) -> bool {
        self.overflow
    }
    pub fn b_flag_2(&self) -> bool {
        self.b_flag_2
    }
    pub fn b_flag_1(&self) -> bool {
        self.b_flag_1
    }
    pub fn decimal(&self) -> bool {
        self.decimal
    }
    pub fn zero(&self) -> bool {
        self.zero
    }
    pub fn carry(&self) -> bool {
        self.carry
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let status_register = StatusRegister::new();
        assert_eq!(false, status_register.negative);
        assert_eq!(false, status_register.overflow);
        assert_eq!(false, status_register.b_flag_2);
        assert_eq!(false, status_register.b_flag_1);
        assert_eq!(false, status_register.decimal);
        assert_eq!(false, status_register.interrupt_disable);
        assert_eq!(false, status_register.zero);
        assert_eq!(false, status_register.carry);
    }

    #[test]
    fn test_reset() {
        let mut status_register = StatusRegister::new();
        status_register.set(255);
        assert_eq!(true, status_register.negative);
        assert_eq!(true, status_register.overflow);
        assert_eq!(true, status_register.b_flag_2);
        assert_eq!(true, status_register.b_flag_1);
        assert_eq!(true, status_register.decimal);
        assert_eq!(true, status_register.interrupt_disable);
        assert_eq!(true, status_register.zero);
        assert_eq!(true, status_register.carry);

        status_register.reset();
        assert_eq!(false, status_register.negative);
        assert_eq!(false, status_register.overflow);
        assert_eq!(false, status_register.b_flag_2);
        assert_eq!(false, status_register.b_flag_1);
        assert_eq!(false, status_register.decimal);
        assert_eq!(false, status_register.interrupt_disable);
        assert_eq!(false, status_register.zero);
        assert_eq!(false, status_register.carry);
    }

    #[test]
    fn test_set_negative() {
        let mut status_register = StatusRegister::new();

        // Set negative
        status_register.set(128);
        assert_eq!(true, status_register.negative);
        assert_eq!(false, status_register.overflow);
        assert_eq!(false, status_register.b_flag_2);
        assert_eq!(false, status_register.b_flag_1);
        assert_eq!(false, status_register.decimal);
        assert_eq!(false, status_register.interrupt_disable);
        assert_eq!(false, status_register.zero);
        assert_eq!(false, status_register.carry);
    }

    #[test]
    fn test_set_overflow() {
        let mut status_register = StatusRegister::new();

        // Set overflow
        status_register.set(64);
        assert_eq!(false, status_register.negative);
        assert_eq!(true, status_register.overflow);
        assert_eq!(false, status_register.b_flag_2);
        assert_eq!(false, status_register.b_flag_1);
        assert_eq!(false, status_register.decimal);
        assert_eq!(false, status_register.interrupt_disable);
        assert_eq!(false, status_register.zero);
        assert_eq!(false, status_register.carry);
    }

    #[test]
    fn test_set_b_flag_2() {
        let mut status_register = StatusRegister::new();

        // Set b_flag_2
        status_register.set(32);
        assert_eq!(false, status_register.negative);
        assert_eq!(false, status_register.overflow);
        assert_eq!(true, status_register.b_flag_2);
        assert_eq!(false, status_register.b_flag_1);
        assert_eq!(false, status_register.decimal);
        assert_eq!(false, status_register.interrupt_disable);
        assert_eq!(false, status_register.zero);
        assert_eq!(false, status_register.carry);
    }

    #[test]
    fn test_set_b_flag_1() {
        let mut status_register = StatusRegister::new();

        // Set b_flag_1
        status_register.set(16);
        assert_eq!(false, status_register.negative);
        assert_eq!(false, status_register.overflow);
        assert_eq!(false, status_register.b_flag_2);
        assert_eq!(true, status_register.b_flag_1);
        assert_eq!(false, status_register.decimal);
        assert_eq!(false, status_register.interrupt_disable);
        assert_eq!(false, status_register.zero);
        assert_eq!(false, status_register.carry);
    }

    #[test]
    fn test_set_decimal() {
        let mut status_register = StatusRegister::new();

        // Set decimal
        status_register.set(8);
        assert_eq!(false, status_register.negative);
        assert_eq!(false, status_register.overflow);
        assert_eq!(false, status_register.b_flag_2);
        assert_eq!(false, status_register.b_flag_1);
        assert_eq!(true, status_register.decimal);
        assert_eq!(false, status_register.interrupt_disable);
        assert_eq!(false, status_register.zero);
        assert_eq!(false, status_register.carry);
    }

    #[test]
    fn test_set_interrupt_disable() {
        let mut status_register = StatusRegister::new();

        // Set interrupt_disable
        status_register.set(4);
        assert_eq!(false, status_register.negative);
        assert_eq!(false, status_register.overflow);
        assert_eq!(false, status_register.b_flag_2);
        assert_eq!(false, status_register.b_flag_1);
        assert_eq!(false, status_register.decimal);
        assert_eq!(true, status_register.interrupt_disable);
        assert_eq!(false, status_register.zero);
        assert_eq!(false, status_register.carry);
    }

    #[test]
    fn test_set_zero() {
        let mut status_register = StatusRegister::new();

        // Set zero
        status_register.set(2);
        assert_eq!(false, status_register.negative);
        assert_eq!(false, status_register.overflow);
        assert_eq!(false, status_register.b_flag_2);
        assert_eq!(false, status_register.b_flag_1);
        assert_eq!(false, status_register.decimal);
        assert_eq!(false, status_register.interrupt_disable);
        assert_eq!(true, status_register.zero);
        assert_eq!(false, status_register.carry);
    }

    #[test]
    fn test_set_carry() {
        let mut status_register = StatusRegister::new();

        // Set carry
        status_register.set(1);
        assert_eq!(false, status_register.negative);
        assert_eq!(false, status_register.overflow);
        assert_eq!(false, status_register.b_flag_2);
        assert_eq!(false, status_register.b_flag_1);
        assert_eq!(false, status_register.decimal);
        assert_eq!(false, status_register.interrupt_disable);
        assert_eq!(false, status_register.zero);
        assert_eq!(true, status_register.carry);
    }
}
