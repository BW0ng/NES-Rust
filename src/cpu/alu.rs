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
#[allow(overflowing_literals)]
pub fn operate(
    a: u8,
    b: u8,
    cin: bool,
    opcode: u8,
    register: &mut Registers,
    memory: &mut Memory,
) -> (u8, bool, bool) {
    // TODO figure out how to split out opcodes.
    //     let enable_sum: bool = helper::bit_set(opcode, 0);
    //     let enable_and: bool = helper::bit_set(opcode, 0);
    //     let enable_eor: bool = helper::bit_set(opcode, 0);
    //     let enable_or: bool = helper::bit_set(opcode, 0);
    //     let enable_shift_right: bool = helper::bit_set(opcode, 0);
    //     let enable_barrel_shift_right: bool = helper::bit_set(opcode, 0);
    match opcode {
        // A - 0 C - 0
        0b000_000_00 => {
            println!("BRK impl");
            // TODO interrupt
        }
        0b000_010_00 => println!("PHP impl"),
        0b000_100_00 => println!("BPL rel"),
        0b000_110_00 => println!("CLC impl"),
        // A - 0, C - 1
        0b000_000_01 => {
            println!("ORA X, ind");
            memory.memory[0xFFFF] = 1;
        }
        0b000_001_01 => println!("ORA zpg"),
        0b000_010_01 => println!("ORA #"),
        0b000_011_01 => println!("ORA abs"),
        0b000_100_01 => println!("ORA ind, Y"),
        0b000_101_01 => println!("ORA zpg, X"),
        0b000_110_01 => println!("ORA abs, Y"),
        0b000_111_01 => println!("ORA abs, X"),
        // A - 0, C - 2
        0b000_001_10 => println!("ASL zpg"),
        0b000_010_10 => {
            println!("ASL A");
            let shifted: u8 = a << 1;
            let carry: bool = ((a & 0b1000_0000) >> 7) == 1;
            register.set_carry(carry);
            let overflow: bool = shifted < a; // TODO Need to figure out how to do overflow
            register.set_overflow(overflow);
            return (shifted, carry, overflow);
        }
        0b000_011_10 => println!("ASL abs"),
        0b000_101_10 => println!("ASL zpg, X"),
        0b000_111_10 => println!("ASL abs, X"),
        // A - 1 C - 0
        0b001_000_00 => println!("JSR abs"),
        0b001_001_00 => println!("BIT zpg"),
        0b001_010_00 => println!("PLP impl"),
        0b001_011_00 => println!("BIT abs"),
        0b001_100_00 => println!("BMI rel"),
        0b001_110_00 => println!("SEC impl"),
        // A - 1, C - 1
        0b001_000_01 => println!("AND X, ind"),
        0b001_001_01 => println!("AND zpy"),
        0b001_010_01 => println!("AND #"),
        0b001_011_01 => println!("AND abs"),
        0b001_100_01 => println!("AND ind, Y"),
        0b001_101_01 => println!("AND zpg, X"),
        0b001_110_01 => println!("AND abs, Y"),
        0b001_111_01 => println!("AND abs, X"),
        // A - 1, C - 2
        0b001_001_10 => println!("ROL zpg"),
        0b001_010_10 => println!("ROL A"),
        0b001_011_10 => println!("ROL abs"),
        0b001_101_10 => println!("ROL zpg, X"),
        0b001_111_10 => println!("ROL abs, X"),
        // A - 2 C - 0
        0b010_000_00 => println!("RTI impl"),
        0b010_010_00 => println!("PHA impl"),
        0b010_011_00 => println!("JMP abs"),
        0b010_100_00 => println!("BVC rel"),
        0b010_110_00 => println!("CLI impl"),
        // A - 2, C - 1
        0b010_000_01 => println!("EOR X, ind"),
        0b010_001_01 => println!("EOR zpg"),
        0b010_010_01 => println!("EOR #"),
        0b010_011_01 => println!("EOR abs"),
        0b010_100_01 => println!("EOR ind, Y"),
        0b010_101_01 => println!("EOR zpg, X"),
        0b010_110_01 => println!("EOR abs, Y"),
        0b010_111_01 => println!("EOr abs, X"),
        // A - 2, C - 2
        0b010_001_10 => println!("ROR zpg"),
        0b010_010_10 => println!("ROR A"),
        0b010_011_10 => println!("ROR abs"),
        0b010_101_10 => println!("ROR zpg, X"),
        0b010_111_10 => println!("ROR abs, X"),
        // A - 3 C - 0
        0b011_000_00 => println!("RTS impl"),
        0b011_010_00 => println!("PLA impl"),
        0b011_011_00 => println!("JMP ind"),
        0b011_100_00 => println!("BVS rel"),
        0b011_110_00 => println!("SEI impl"),
        // A - 3, C - 1
        0b011_000_01 => println!("ADC X, ind"),
        0b011_001_01 => println!("ADC zpg"),
        0b011_010_01 => println!("ADC #"),
        0b011_011_01 => println!("ADC abs"),
        0b011_100_01 => println!("ADC ind, Y"),
        0b011_101_01 => println!("ADC zpg, X"),
        0b011_110_01 => println!("ADC abs, Y"),
        0b011_111_01 => println!("ADC abs, X"),
        // A - 3, C - 2
        0b011_001_10 => println!("ROR zpg"),
        0b011_010_10 => println!("ROR A"),
        0b011_011_10 => println!("ROR abs"),
        0b011_101_10 => println!("ROR zpg, X"),
        0b011_111_10 => println!("ROR abs, X"),
        // A - 4 C - 0
        0b100_001_00 => println!("STY zpg"),
        0b100_010_00 => println!("DEY impl"),
        0b100_011_00 => println!("STY abs"),
        0b100_100_00 => println!("BCC rel"),
        0b100_101_00 => println!("STY zpg, X"),
        0b100_110_00 => println!("TYA impl"),
        // A - 4, C - 1
        0b100_000_01 => println!("STA X, ind"),
        0b100_001_01 => println!("STA zpg"),
        0b100_011_01 => println!("STA abs"),
        0b100_100_01 => println!("STA ind, Y"),
        0b100_101_01 => println!("STA ind, X"),
        0b100_110_01 => println!("STA abs, Y"),
        0b100_111_01 => println!("STA abs, X"),
        // A - 4, C - 2
        0b100_001_10 => println!("STX zpg"),
        0b100_010_10 => println!("TXA impl"),
        0b100_011_10 => println!("STX abs"),
        0b100_101_10 => println!("STX zpg, Y"),
        0b100_110_10 => println!("TXS impl"),
        // A - 5 C - 0
        0b101_000_00 => println!("LDY #"),
        0b101_001_00 => println!("LDY zpg"),
        0b101_010_00 => println!("TAY impl"),
        0b101_011_00 => println!("LDY abs"),
        0b101_100_00 => println!("BCS rel"),
        0b101_101_00 => println!("LDY zpg, X"),
        0b101_110_00 => println!("CLV impl"),
        0b101_111_00 => println!("LDY abs, X"),
        // A - 5, C - 1
        0b101_000_01 => println!("LDA X, ind"),
        0b101_001_01 => println!("LDA zpg"),
        0b101_010_01 => println!("LDA #"),
        0b101_011_01 => println!("LDA abs"),
        0b101_100_01 => println!("LDA ind, Y"),
        0b101_101_01 => println!("LDA zpg, X"),
        0b101_110_01 => println!("LDA abs, Y"),
        0b101_111_01 => println!("LDA abs, X"),
        // A - 5, C - 2
        0b101_000_10 => println!("LDX #"),
        0b101_001_10 => println!("LDX zpg"),
        0b101_010_10 => println!("TAX impl"),
        0b101_011_10 => println!("LDX abs"),
        0b101_101_10 => println!("LDX zpg, Y"),
        0b101_110_10 => println!("TSX impl"),
        0b101_111_10 => println!("LDX abs, Y"),
        // A - 6 C - 0
        0b110_000_00 => println!("CPY #"),
        0b110_001_00 => println!("CPY zgp"),
        0b110_010_00 => println!("INY impl"),
        0b110_011_00 => println!("CPY abs"),
        0b110_100_00 => println!("BNE rel"),
        0b110_110_00 => println!("CLD impl"),
        // A - 6, C - 1
        0b110_000_01 => println!("CMP X, ind"),
        0b110_001_01 => println!("CMP zpg"),
        0b110_010_01 => println!("CMP #"),
        0b110_011_01 => println!("CMP abs"),
        0b110_100_01 => println!("CMP ind, Y"),
        0b110_101_01 => println!("CMP zpg, X"),
        0b110_110_01 => println!("CMP abs, Y"),
        0b110_111_01 => println!("CMP abs, X"),
        // A - 6, C - 2
        0b110_001_10 => println!("DEC zpg"),
        0b110_010_10 => println!("DEX impl"),
        0b110_011_10 => println!("DEC abs"),
        0b110_101_10 => println!("DEC zpg, X"),
        0b110_111_10 => println!("DEC abs, X"),
        // A - 7 C - 0
        0b111_000_00 => println!("CPX #"),
        0b111_001_00 => println!("CPX zpg"),
        0b111_010_00 => println!("INX impl"),
        0b111_011_00 => println!("CPX abs"),
        0b111_100_00 => println!("BEQ rel"),
        0b111_110_00 => println!("SED impl"),
        // A - 7, C - 1
        0b111_000_01 => println!("SBC X, ind"),
        0b111_001_01 => println!("SBC zpg"),
        0b111_010_01 => println!("SBC #"),
        0b111_011_01 => println!("SBC abs"),
        0b111_100_01 => println!("SBC ind, Y"),
        0b111_101_01 => println!("SBC zpg, X"),
        0b111_110_01 => println!("SBC abs, Y"),
        0b111_111_01 => println!("SBC abs, X"),
        // A - 7, C - 2
        0b111_001_10 => println!("INC zpg"),
        0b111_010_10 => println!("NOP impl"),
        0b111_011_10 => println!("INC abs"),
        0b111_101_10 => println!("INC zpg, X"),
        0b111_111_10 => println!(" INC abs, Xs"),
        // Designed to catch any weird cases.
        _ => println!("Unknown opcode: {:#08b}", opcode),
    }

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
        let mut register = Registers::new();
        let mut memory = Memory::new();

        let (output, carry, overflow) = crate::cpu::alu::operate(
            0b0000_0001,
            0b0000_0100,
            false,
            0b000_010_10,
            &mut register,
            &mut memory,
        );
        assert_eq!(0b000_0010, output);
        assert_eq!(false, carry);
        assert_eq!(false, overflow);
        assert_eq!(0b0000_0000, register.status().get_status());

        let (output, carry, overflow) = crate::cpu::alu::operate(
            0b1000_0000,
            0b0000_0100,
            false,
            0b000_010_10,
            &mut register,
            &mut memory,
        );
        assert_eq!(0b0000_0000, output);
        assert_eq!(true, carry);
        assert_eq!(true, overflow);
        assert_eq!(0b0100_0001, register.status().get_status());
    }
}
