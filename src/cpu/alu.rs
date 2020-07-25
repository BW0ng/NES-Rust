use super::memory::Memory;
use super::register::Registers;

/**
 * AAABBBCC
 * AAA---CC - opcode
 * BBB - addressing modes
 *
 * Addressing modes
 * 000 - zero page, X
 * 001 - zero page
 * 010 - #immediate
 * 011 - absolute
 * 100 - zero page, Y
 * 101 - zero page, X
 * 110 - absolute, Y
 * 111 - absolute, X
 *
 * http://www.cs.columbia.edu/~sedwards/classes/2013/4840/reports/6502.pdf
 * http://www.6502.org/tutorials/6502opcodes.html
 *
 * Returns (results, carry, overflow)
 */
pub fn operate(
    a: u8,
    b: u8,
    cin: bool,
    opcode: u8,
    address: 
    register: &mut Registers,
    memory: &mut Memory,
) -> (u8, bool, bool) {
    

    return (0, false, false);
}

// fn overflowed(A: i8, B: i8, operation: u8, result: i8) -> bool {
//     match operation {
//         0 =>
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_operate_asl_a() {
        
    }
}
