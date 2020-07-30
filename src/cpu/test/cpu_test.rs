use super::Mode::*;
use super::*;

use super::super::cpu::CPU;

// #region
macro_rules! build_cpu {
    ($bytes:expr) => {{
        // let mut rom = vec![
        //     0x4e, 0x45, 0x53, 0x1a, 0x02, // Two pages of PRG-ROM
        //     0x00, // Zero pages CHR-ROM means use CHR-RAM
        //     0x01, // Vertical mirroring
        //     0x00, 0x01, // One page of PRG-RAM
        //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // ];

        // add the PRG-ROM
        // rom.extend_from_slice(&[0u8; 2 * 0x4000]);
        let bus = Bus::new();
        // bus.load_rom_from_memory(&rom);
        let mut cpu = CPU::new(bus);
        cpu.pc = 0;
        let bytes = $bytes;
        for (i, &b) in bytes.iter().enumerate() {
            cpu.bus.ram[i] = b as u8;
        }
        cpu
    }};
}

// macro_rules! build_cpu_and_run {
//     ($instruction:expr, $mode:ident, $bytes:expr) => {{
//         let op = opcode($instruction, $mode);
//         let mut mem = $bytes;
//         mem.insert(0, op.code);
//         let mut cpu = build_cpu!(mem);
//         let start_pc = cpu.pc;
//         let start_cycles = cpu.bus.cycles;
//         let start_status = cpu.status;
//         cpu.execute_next_instruction();
//         assert_eq!(0, cpu.status & start_status & !op.mask);
//         assert_eq!(op.size, cpu.pc - start_pc);
//         assert_eq!(op.cycles, cpu.bus.cycles - start_cycles);
//         cpu
//     }};
// }

// macro_rules! bit_convert {
//     (carry, 1)          => 0b0000_0001;
//     (zero, 1)           => 0b0000_0010;
//     (irq_disable, 1)    => 0b0000_0100;
//     (decimal, 1)        => 0b0000_1000;
//     (break_flag, 1)     => 0b0001_0000;
//     (push, 1)           => 0b0010_0000;
//     (overflow, 1)       => 0b0100_0000;
//     (negative, 1)       => 0b1000_0000;
// }

// macro_rules! status_convert {
//     ({$($status_bit:ident: $value:expr)}) => {
//         $(
//             bit_convert!($status_bit, $value)
//         )*
//     };
// }

macro_rules! test_op {
    ($instruction:expr, $mode:ident, [$($b:expr),*]{$($sk:ident : $sv:expr),*} => [$($rb:expr),*]{$($ek:ident : $ev:expr),*}) => {
        {
            let op = opcode($instruction, $mode);
            let mut mem = Vec::new();
            $(mem.push($b);)*
            mem.insert(0, op.code);
            let mut cpu = build_cpu!(mem);
            let start_pc = cpu.pc;
            let start_cycles = cpu.bus.cycles;
            let start_status = cpu.status;
            $(
                cpu.$sk=$sv;
            )*
            cpu.execute_next_instruction();
            println!("Executing {} of type {}", $instruction, $mode);
            assert!(0 == cpu.status & start_status & !op.mask, "Register mask not respected. Status: 0b{:b}", cpu.status);
            if op.size > 0 {
                assert!(op.size == (cpu.pc - start_pc), "Invalid instruction size. Expected: {} bytes, Got: {}", op.size, cpu.pc - start_pc);
            }
            if op.cycles > 0 {
                assert!(op.cycles == (cpu.bus.cycles - start_cycles), "Invalid instruction duration. Expected: {} cycles, Got: {}", op.cycles, cpu.bus.cycles - start_cycles);
            }
            $(
                assert!(cpu.$ek==$ev, "Incorrect Register. Expected cpu.{} to be {}, got {}", stringify!($ek), stringify!($ev), cpu.$ek);
            )*
            let mut mem = Vec::new();
            $(mem.push($rb);)*
            mem.insert(0, op.code);
            for (i, &b) in mem.iter().enumerate() {
                assert!(cpu.bus.ram[i]==b, "Incorrect Memory. Expected ram[{}] to be {}, got {}", i, b, cpu.bus.ram[i]);
            }

            cpu
        }
    }
}

// test_op!("opcode", <Addressing type> , [memory]{registers} => {registers})
#[test]
fn test_lda() {
    test_op!("lda", Immediate, [0x00]{}                         => []{ a: 0x00, status: 0b00000010 });
    test_op!("lda", Immediate, [0xFF]{}                         => []{ a: 0xFF, status: 0b10000000 });
    test_op!("lda", Immediate, [0x20]{}                         => []{ a: 0x20, status: 0 });
    test_op!("lda", ZeroPage,  [0x02, 0x90]{}                   => []{ a: 0x90 });
    test_op!("lda", ZeroPageX, [0x02, 0, 0x90]{x:1}             => []{ a: 0x90 });
    test_op!("lda", Absolute,  [0x04, 0, 0, 0x90]{}             => []{ a: 0x90 });
    test_op!("lda", AbsoluteX, [0x03, 0, 0, 0x90]{x:1}          => []{ a: 0x90 });
    test_op!("lda", AbsoluteY, [0x03, 0, 0, 0x90]{y:1}          => []{ a: 0x90 });
    test_op!("lda", IndirectX, [0x02, 0, 0x05, 0, 0x90]{x:1}    => []{ a: 0x90 });
    test_op!("lda", IndirectY, [0x02, 0x04, 0, 0, 0x90]{y:1}    => []{ a: 0x90 });
}

#[test]
fn test_ldx() {
    test_op!("ldx", Immediate, [0x00]{}                 => []{ x: 0x00, status: 0b00000010 });
    test_op!("ldx", Immediate, [0xFF]{}                 => []{ x: 0xFF, status: 0b10000000 });
    test_op!("ldx", Immediate, [0x20]{}                 => []{ x: 0x20, status: 0 });
    test_op!("ldx", ZeroPage,  [0x02, 0x90]{}           => []{ x: 0x90 });
    test_op!("ldx", ZeroPageY, [0x02, 0, 0x90]{y:1}     => []{ x: 0x90 });
    test_op!("ldx", Absolute,  [0x04, 0, 0, 0x90]{}     => []{ x: 0x90 });
    test_op!("ldx", AbsoluteY, [0x03, 0, 0, 0x90]{y:1}  => []{ x: 0x90 });
}

#[test]
fn test_ldy() {
    test_op!("ldy", Immediate, [0x00]{}                 => []{ y: 0x00, status: 0b00000010 });
    test_op!("ldy", Immediate, [0xFF]{}                 => []{ y: 0xFF, status: 0b10000000 });
    test_op!("ldy", Immediate, [0x20]{}                 => []{ y: 0x20, status: 0 });
    test_op!("ldy", ZeroPage,  [0x02, 0x90]{}           => []{ y: 0x90 });
    test_op!("ldy", ZeroPageX, [0x02, 0, 0x90]{x:1}     => []{ y: 0x90 });
    test_op!("ldy", Absolute,  [0x04, 0, 0, 0x90]{x:1}  => []{ y: 0x90 });
    test_op!("ldy", AbsoluteX, [0x03, 0, 0, 0x90]{x:1}  => []{ y: 0x90 });
}

#[test]
fn test_sta() {
    test_op!("sta", ZeroPage,  [0x02]{a: 0x66} => [0x02, 0x66]{});
    test_op!("sta", ZeroPageX, [0x02]{a: 0x66, x:1} => [0x02, 0, 0x66]{});
    test_op!("sta", Absolute,  [0x04, 0]{a:0x66} => [0x04, 0, 0, 0x66]{});
    test_op!("sta", AbsoluteX, [0x03, 0]{a:0x66, x:1} => [0x03, 0, 0, 0x66]{});
    test_op!("sta", AbsoluteY, [0x03, 0]{a:0x66, y:1} => [0x03, 0, 0, 0x66]{});
    test_op!("sta", IndirectX, [0x02, 0, 0x05, 0, 0]{a: 0x66, x:1} => [0x02, 0, 0x05, 0, 0x66]{});
    test_op!("sta", IndirectY, [0x02, 0x04, 0, 0, 0]{a: 0x66, y:1} => [0x02, 0x04, 0, 0, 0x66]{});
}

#[test]
fn test_stx() {
    test_op!("stx", ZeroPage,  [0x02]{x: 0x66} => [0x02, 0x66]{});
    test_op!("stx", ZeroPageY, [0x02]{x: 0x66, y:1} => [0x02, 0, 0x66]{});
    test_op!("stx", Absolute,  [0x04, 0]{x: 0x66} => [0x04, 0, 0, 0x66]{});
}

#[test]
fn test_sty() {
    test_op!("sty", ZeroPage,  [0x02]{y: 0x66} => [0x02, 0x66]{});
    test_op!("sty", ZeroPageX, [0x02]{y: 0x66, x:1} => [0x02, 0, 0x66]{});
    test_op!("sty", Absolute,  [0x04, 0]{y: 0x66} => [0x04, 0, 0, 0x66]{});
}

#[test]
fn test_adc() {
    test_op!("adc", Immediate, [3]{a:2, status:1}                   => []{ a: 6 });
    test_op!("adc", Immediate, [255]{a:1, status:0}                 => []{ a: 0, status: 0b00000011 });
    test_op!("adc", Immediate, [127]{a:1, status:0}                 => []{ a: 128, status: 0b11000000 });
    test_op!("adc", Immediate, [200]{a:100}                         => []{ a: 44 });
    test_op!("adc", ZeroPage,  [0x02, 0x90]{a: 1}                   => []{ a: 0x91 });
    test_op!("adc", ZeroPageX, [0x02, 0, 0x90]{x:1, a: 1}           => []{ a: 0x91 });
    test_op!("adc", Absolute,  [0x04, 0, 0, 0x90]{a:1}              => []{ a: 0x91 });
    test_op!("adc", AbsoluteX, [0x03, 0, 0, 0x90]{x:1, a: 1}        => []{ a: 0x91 });
    test_op!("adc", AbsoluteY, [0x03, 0, 0, 0x90]{y:1, a: 1}        => []{ a: 0x91 });
    test_op!("adc", IndirectX, [0x02, 0, 0x05, 0, 0x90]{x:1, a: 1}  => []{ a: 0x91 });
    test_op!("adc", IndirectY, [0x02, 0x04, 0, 0, 0x90]{y:1, a: 1}  => []{ a: 0x91 });
}
// #endregion

/**
 *
7  bit  0
---- ----
NVss DIZC
|||| ||||
|||| |||+- Carry
|||| ||+-- Zero
|||| |+--- Interrupt Disable
|||| +---- Decimal
||++------ No CPU effect, see: the B flag
|+-------- Overflow
+--------- Negative
 */
#[test]
fn test_sbc() {
    test_op!("sbc", Immediate, [2]{a:10, status:1} => []{ a: 8 });
    test_op!("sbc", Immediate, [2]{a:10, status:0} => []{ a: 7 });
    test_op!("sbc", Immediate, [176]{a:80, status:1} => []{ a: 160, status: 0b11000000 });
    test_op!("sbc", ZeroPage,  [0x02, 0x90]{a: 0xFF, status: 1} => []{ a: 0x6f });
    test_op!("sbc", ZeroPageX, [0x02, 0, 0x90]{x:1, a: 0xFF, status: 1} => []{ a: 0x6f });
    test_op!("sbc", Absolute,  [0x04, 0, 0, 0x90]{a:0xFF, status: 1} => []{ a: 0x6f });
    test_op!("sbc", AbsoluteX, [0x03, 0, 0, 0x90]{x:1, a: 0xFF, status: 1} => []{ a: 0x6f });
    test_op!("sbc", AbsoluteY, [0x03, 0, 0, 0x90]{y:1, a: 0xFF, status: 1} => []{ a: 0x6f });
    test_op!("sbc", IndirectX, [0x02, 0, 0x05, 0, 0x90]{x:1, a: 0xFF, status: 1} => []{ a: 0x6f });
    test_op!("sbc", IndirectY, [0x02, 0x04, 0, 0, 0x90]{y:1, a: 0xFF, status: 1} => []{ a: 0x6f });
}

#[test]
fn test_and() {
    test_op!("and", Immediate, [0b00001111]{a:0b01010101} => []{ a: 0b00000101, status: 0 });
    test_op!("and", Immediate, [0b10001111]{a:0b11010101} => []{ a: 0b10000101, status: 0b10000000 });
    test_op!("and", Immediate, [0]{a:0b11010101} => []{ a: 0, status: 0b00000010 });
    test_op!("and", ZeroPage,  [0x02, 0xFF]{a: 0xF0} => []{a: 0xF0});
    test_op!("and", ZeroPageX, [0x02, 0, 0xFF]{x:1, a: 0xF0} => []{a: 0xF0});
    test_op!("and", Absolute,  [0x04, 0, 0, 0xFF]{a:0xF0} => []{a: 0xF0});
    test_op!("and", AbsoluteX, [0x03, 0, 0, 0xFF]{x:1, a: 0xF0} => []{a: 0xF0});
    test_op!("and", AbsoluteY, [0x03, 0, 0, 0xFF]{y:1, a: 0xF0} => []{a: 0xF0});
    test_op!("and", IndirectX, [0x02, 0, 0x05, 0, 0xFF]{x:1, a: 0xF0} => []{a: 0xF0});
    test_op!("and", IndirectY, [0x02, 0x04, 0, 0, 0xFF]{y:1, a: 0xF0} => []{a: 0xF0});
}

#[test]
fn test_asl() {
    test_op!("asl", ZeroPage,  [0x02, 0xFF]{status:1} => [0x02, 0xFE]{status: 0b10000001});
    test_op!("asl", ZeroPage,  [0x02, 0xFF]{status:0} => [0x02, 0xFE]{status: 0b10000001});
    test_op!("asl", ZeroPage,  [0x02, 0b10000000]{} => [0x02, 0]{status: 0b00000011});
    test_op!("asl", ZeroPageX, [0x02, 0, 1]{x: 1} => [0x02, 0, 2]{});
    test_op!("asl", Absolute,  [0x03, 0, 1]{} => [0x03, 0, 2]{});
    test_op!("asl", AbsoluteX, [0x03, 0, 0, 1]{x: 1} => [0x03, 0, 0, 2]{});
    test_op!("asl", NoMode, []{a: 1} => []{a: 2});
}

// #region
#[derive(Debug)]
struct Op {
    code: u8,
    size: u16,
    cycles: u64,
    check: bool,
    mask: u8,
}

// # Cycles can be found here: httstatus://nesdev.com/6502_cpu.txt
fn opcode(name: &str, mode: Mode) -> Op {
    match (name, mode) {
        ("adc", Immediate) => Op {
            code: 0x69,
            size: 2,
            cycles: 2,
            check: false,
            mask: 0b11000011,
        },
        ("adc", ZeroPage) => Op {
            code: 0x65,
            size: 2,
            cycles: 3,
            check: false,
            mask: 0b11000011,
        },
        ("adc", ZeroPageX) => Op {
            code: 0x75,
            size: 2,
            cycles: 4,
            check: false,
            mask: 0b11000011,
        },
        ("adc", Absolute) => Op {
            code: 0x6D,
            size: 3,
            cycles: 4,
            check: false,
            mask: 0b11000011,
        },
        ("adc", AbsoluteX) => Op {
            code: 0x7D,
            size: 3,
            cycles: 4,
            check: true,
            mask: 0b11000011,
        },
        ("adc", AbsoluteY) => Op {
            code: 0x79,
            size: 3,
            cycles: 4,
            check: true,
            mask: 0b11000011,
        },
        ("adc", IndirectX) => Op {
            code: 0x61,
            size: 2,
            cycles: 6,
            check: false,
            mask: 0b11000011,
        },
        ("adc", IndirectY) => Op {
            code: 0x71,
            size: 2,
            cycles: 5,
            check: true,
            mask: 0b11000011,
        },
        ("and", Immediate) => Op {
            code: 0x29,
            size: 2,
            cycles: 2,
            check: false,
            mask: 0b10000010,
        },
        ("and", ZeroPage) => Op {
            code: 0x25,
            size: 2,
            cycles: 3,
            check: false,
            mask: 0b10000010,
        },
        ("and", ZeroPageX) => Op {
            code: 0x35,
            size: 2,
            cycles: 4,
            check: false,
            mask: 0b10000010,
        },
        ("and", Absolute) => Op {
            code: 0x2D,
            size: 3,
            cycles: 4,
            check: false,
            mask: 0b10000010,
        },
        ("and", AbsoluteX) => Op {
            code: 0x3D,
            size: 3,
            cycles: 4,
            check: true,
            mask: 0b10000010,
        },
        ("and", AbsoluteY) => Op {
            code: 0x39,
            size: 3,
            cycles: 4,
            check: true,
            mask: 0b10000010,
        },
        ("and", IndirectX) => Op {
            code: 0x21,
            size: 2,
            cycles: 6,
            check: false,
            mask: 0b10000010,
        },
        ("and", IndirectY) => Op {
            code: 0x31,
            size: 2,
            cycles: 5,
            check: true,
            mask: 0b10000010,
        },
        ("asl", NoMode) => Op {
            code: 0x0A,
            size: 1,
            cycles: 2,
            check: false,
            mask: 0b10000011,
        },
        ("asl", ZeroPage) => Op {
            code: 0x06,
            size: 2,
            cycles: 5,
            check: false,
            mask: 0b10000011,
        },
        ("asl", ZeroPageX) => Op {
            code: 0x16,
            size: 2,
            cycles: 6,
            check: false,
            mask: 0b10000011,
        },
        ("asl", Absolute) => Op {
            code: 0x0E,
            size: 3,
            cycles: 6,
            check: false,
            mask: 0b10000011,
        },
        ("asl", AbsoluteX) => Op {
            code: 0x1E,
            size: 3,
            cycles: 7,
            check: false,
            mask: 0b10000011,
        },
        ("bcc", NoMode) => Op {
            code: 0x90,
            size: 0,
            cycles: 0,
            check: true,
            mask: 0b00000000,
        },
        ("bcs", NoMode) => Op {
            code: 0xB0,
            size: 0,
            cycles: 0,
            check: true,
            mask: 0b00000000,
        },
        ("beq", NoMode) => Op {
            code: 0xF0,
            size: 0,
            cycles: 0,
            check: true,
            mask: 0b00000000,
        },
        ("bit", ZeroPage) => Op {
            code: 0x24,
            size: 2,
            cycles: 3,
            check: false,
            mask: 0b11000010,
        },
        ("bit", Absolute) => Op {
            code: 0x2C,
            size: 3,
            cycles: 4,
            check: false,
            mask: 0b11000010,
        },
        ("bmi", NoMode) => Op {
            code: 0x30,
            size: 0,
            cycles: 0,
            check: true,
            mask: 0b00000000,
        },
        ("bne", NoMode) => Op {
            code: 0xD0,
            size: 0,
            cycles: 0,
            check: true,
            mask: 0b00000000,
        },
        ("bpl", NoMode) => Op {
            code: 0x10,
            size: 0,
            cycles: 0,
            check: true,
            mask: 0b00000000,
        },
        ("brk", NoMode) => Op {
            code: 0x00,
            size: 0,
            cycles: 7,
            check: false,
            mask: 0b00010000,
        },
        ("bvc", NoMode) => Op {
            code: 0x50,
            size: 0,
            cycles: 0,
            check: true,
            mask: 0b00000000,
        },
        ("bvs", NoMode) => Op {
            code: 0x70,
            size: 0,
            cycles: 0,
            check: true,
            mask: 0b00000000,
        },
        ("clc", NoMode) => Op {
            code: 0x18,
            size: 1,
            cycles: 2,
            check: false,
            mask: 0b00000001,
        },
        ("cld", NoMode) => Op {
            code: 0xD8,
            size: 1,
            cycles: 2,
            check: false,
            mask: 0b00001000,
        },
        ("cli", NoMode) => Op {
            code: 0x58,
            size: 1,
            cycles: 2,
            check: false,
            mask: 0b00000100,
        },
        ("clv", NoMode) => Op {
            code: 0xB8,
            size: 1,
            cycles: 2,
            check: false,
            mask: 0b01000000,
        },
        ("cmp", Immediate) => Op {
            code: 0xC9,
            size: 2,
            cycles: 2,
            check: false,
            mask: 0b10000011,
        },
        ("cmp", ZeroPage) => Op {
            code: 0xC5,
            size: 2,
            cycles: 3,
            check: false,
            mask: 0b10000011,
        },
        ("cmp", ZeroPageX) => Op {
            code: 0xD5,
            size: 2,
            cycles: 4,
            check: false,
            mask: 0b10000011,
        },
        ("cmp", Absolute) => Op {
            code: 0xCD,
            size: 3,
            cycles: 4,
            check: false,
            mask: 0b10000011,
        },
        ("cmp", AbsoluteX) => Op {
            code: 0xDD,
            size: 3,
            cycles: 4,
            check: true,
            mask: 0b10000011,
        },
        ("cmp", AbsoluteY) => Op {
            code: 0xD9,
            size: 3,
            cycles: 4,
            check: true,
            mask: 0b10000011,
        },
        ("cmp", IndirectX) => Op {
            code: 0xC1,
            size: 2,
            cycles: 6,
            check: false,
            mask: 0b10000011,
        },
        ("cmp", IndirectY) => Op {
            code: 0xD1,
            size: 2,
            cycles: 5,
            check: true,
            mask: 0b10000011,
        },
        ("cpx", Immediate) => Op {
            code: 0xE0,
            size: 2,
            cycles: 2,
            check: false,
            mask: 0b10000011,
        },
        ("cpx", ZeroPage) => Op {
            code: 0xE4,
            size: 2,
            cycles: 3,
            check: false,
            mask: 0b10000011,
        },
        ("cpx", Absolute) => Op {
            code: 0xEC,
            size: 3,
            cycles: 4,
            check: false,
            mask: 0b10000011,
        },
        ("cpy", Immediate) => Op {
            code: 0xC0,
            size: 2,
            cycles: 2,
            check: false,
            mask: 0b10000011,
        },
        ("cpy", ZeroPage) => Op {
            code: 0xC4,
            size: 2,
            cycles: 3,
            check: false,
            mask: 0b10000011,
        },
        ("cpy", Absolute) => Op {
            code: 0xCC,
            size: 3,
            cycles: 4,
            check: false,
            mask: 0b10000011,
        },
        ("dec", ZeroPage) => Op {
            code: 0xC6,
            size: 2,
            cycles: 5,
            check: false,
            mask: 0b10000010,
        },
        ("dec", ZeroPageX) => Op {
            code: 0xD6,
            size: 2,
            cycles: 6,
            check: false,
            mask: 0b10000010,
        },
        ("dec", Absolute) => Op {
            code: 0xCE,
            size: 3,
            cycles: 6,
            check: false,
            mask: 0b10000010,
        },
        ("dec", AbsoluteX) => Op {
            code: 0xDE,
            size: 3,
            cycles: 7,
            check: false,
            mask: 0b10000010,
        },
        ("dex", NoMode) => Op {
            code: 0xCA,
            size: 1,
            cycles: 2,
            check: false,
            mask: 0b10000010,
        },
        ("dey", NoMode) => Op {
            code: 0x88,
            size: 1,
            cycles: 2,
            check: false,
            mask: 0b10000010,
        },
        ("eor", Immediate) => Op {
            code: 0x49,
            size: 2,
            cycles: 2,
            check: false,
            mask: 0b10000010,
        },
        ("eor", ZeroPage) => Op {
            code: 0x45,
            size: 2,
            cycles: 3,
            check: false,
            mask: 0b10000010,
        },
        ("eor", ZeroPageX) => Op {
            code: 0x55,
            size: 2,
            cycles: 4,
            check: false,
            mask: 0b10000010,
        },
        ("eor", Absolute) => Op {
            code: 0x4D,
            size: 3,
            cycles: 4,
            check: false,
            mask: 0b10000010,
        },
        ("eor", AbsoluteX) => Op {
            code: 0x5D,
            size: 3,
            cycles: 4,
            check: true,
            mask: 0b10000010,
        },
        ("eor", AbsoluteY) => Op {
            code: 0x59,
            size: 3,
            cycles: 4,
            check: true,
            mask: 0b10000010,
        },
        ("eor", IndirectX) => Op {
            code: 0x41,
            size: 2,
            cycles: 6,
            check: false,
            mask: 0b10000010,
        },
        ("eor", IndirectY) => Op {
            code: 0x51,
            size: 2,
            cycles: 5,
            check: true,
            mask: 0b10000010,
        },
        ("inc", ZeroPage) => Op {
            code: 0xE6,
            size: 2,
            cycles: 5,
            check: false,
            mask: 0b10000010,
        },
        ("inc", ZeroPageX) => Op {
            code: 0xF6,
            size: 2,
            cycles: 6,
            check: false,
            mask: 0b10000010,
        },
        ("inc", Absolute) => Op {
            code: 0xEE,
            size: 3,
            cycles: 6,
            check: false,
            mask: 0b10000010,
        },
        ("inc", AbsoluteX) => Op {
            code: 0xFE,
            size: 3,
            cycles: 7,
            check: false,
            mask: 0b10000010,
        },
        ("inx", NoMode) => Op {
            code: 0xE8,
            size: 1,
            cycles: 2,
            check: false,
            mask: 0b10000010,
        },
        ("iny", NoMode) => Op {
            code: 0xC8,
            size: 1,
            cycles: 2,
            check: false,
            mask: 0b10000010,
        },
        ("jmp", Absolute) => Op {
            code: 0x4C,
            size: 0,
            cycles: 3,
            check: false,
            mask: 0b00000000,
        },
        ("jmp", Indirect) => Op {
            code: 0x6C,
            size: 0,
            cycles: 5,
            check: false,
            mask: 0b00000000,
        },
        ("jsr", Absolute) => Op {
            code: 0x20,
            size: 0,
            cycles: 6,
            check: false,
            mask: 0b00000000,
        },
        ("lda", Immediate) => Op {
            code: 0xA9,
            size: 2,
            cycles: 2,
            check: false,
            mask: 0b10000010,
        },
        ("lda", ZeroPage) => Op {
            code: 0xA5,
            size: 2,
            cycles: 3,
            check: false,
            mask: 0b10000010,
        },
        ("lda", ZeroPageX) => Op {
            code: 0xB5,
            size: 2,
            cycles: 4,
            check: false,
            mask: 0b10000010,
        },
        ("lda", Absolute) => Op {
            code: 0xAD,
            size: 3,
            cycles: 4,
            check: false,
            mask: 0b10000010,
        },
        ("lda", AbsoluteX) => Op {
            code: 0xBD,
            size: 3,
            cycles: 4,
            check: true,
            mask: 0b10000010,
        },
        ("lda", AbsoluteY) => Op {
            code: 0xB9,
            size: 3,
            cycles: 4,
            check: true,
            mask: 0b10000010,
        },
        ("lda", IndirectX) => Op {
            code: 0xA1,
            size: 2,
            cycles: 6,
            check: false,
            mask: 0b10000010,
        },
        ("lda", IndirectY) => Op {
            code: 0xB1,
            size: 2,
            cycles: 5,
            check: true,
            mask: 0b10000010,
        },
        ("ldx", Immediate) => Op {
            code: 0xA2,
            size: 2,
            cycles: 2,
            check: false,
            mask: 0b10000010,
        },
        ("ldx", ZeroPage) => Op {
            code: 0xA6,
            size: 2,
            cycles: 3,
            check: false,
            mask: 0b10000010,
        },
        ("ldx", ZeroPageY) => Op {
            code: 0xB6,
            size: 2,
            cycles: 4,
            check: false,
            mask: 0b10000010,
        },
        ("ldx", Absolute) => Op {
            code: 0xAE,
            size: 3,
            cycles: 4,
            check: false,
            mask: 0b10000010,
        },
        ("ldx", AbsoluteY) => Op {
            code: 0xBE,
            size: 3,
            cycles: 4,
            check: true,
            mask: 0b10000010,
        },
        ("ldy", Immediate) => Op {
            code: 0xA0,
            size: 2,
            cycles: 2,
            check: false,
            mask: 0b10000010,
        },
        ("ldy", ZeroPage) => Op {
            code: 0xA4,
            size: 2,
            cycles: 3,
            check: false,
            mask: 0b10000010,
        },
        ("ldy", ZeroPageX) => Op {
            code: 0xB4,
            size: 2,
            cycles: 4,
            check: false,
            mask: 0b10000010,
        },
        ("ldy", Absolute) => Op {
            code: 0xAC,
            size: 3,
            cycles: 4,
            check: false,
            mask: 0b10000010,
        },
        ("ldy", AbsoluteX) => Op {
            code: 0xBC,
            size: 3,
            cycles: 4,
            check: true,
            mask: 0b10000010,
        },
        ("lsr", NoMode) => Op {
            code: 0x4A,
            size: 1,
            cycles: 2,
            check: false,
            mask: 0b10000011,
        },
        ("lsr", ZeroPage) => Op {
            code: 0x46,
            size: 2,
            cycles: 5,
            check: false,
            mask: 0b10000011,
        },
        ("lsr", ZeroPageX) => Op {
            code: 0x56,
            size: 2,
            cycles: 6,
            check: false,
            mask: 0b10000011,
        },
        ("lsr", Absolute) => Op {
            code: 0x4E,
            size: 3,
            cycles: 6,
            check: false,
            mask: 0b10000011,
        },
        ("lsr", AbsoluteX) => Op {
            code: 0x5E,
            size: 3,
            cycles: 7,
            check: false,
            mask: 0b10000011,
        },
        ("nop", NoMode) => Op {
            code: 0xEA,
            size: 1,
            cycles: 2,
            check: false,
            mask: 0b00000000,
        },
        ("ora", Immediate) => Op {
            code: 0x09,
            size: 2,
            cycles: 2,
            check: false,
            mask: 0b10000010,
        },
        ("ora", ZeroPage) => Op {
            code: 0x05,
            size: 2,
            cycles: 3,
            check: false,
            mask: 0b10000010,
        },
        ("ora", ZeroPageX) => Op {
            code: 0x15,
            size: 2,
            cycles: 4,
            check: false,
            mask: 0b10000010,
        },
        ("ora", Absolute) => Op {
            code: 0x0D,
            size: 3,
            cycles: 4,
            check: false,
            mask: 0b10000010,
        },
        ("ora", AbsoluteX) => Op {
            code: 0x1D,
            size: 3,
            cycles: 4,
            check: true,
            mask: 0b10000010,
        },
        ("ora", AbsoluteY) => Op {
            code: 0x19,
            size: 3,
            cycles: 4,
            check: true,
            mask: 0b10000010,
        },
        ("ora", IndirectX) => Op {
            code: 0x01,
            size: 2,
            cycles: 6,
            check: false,
            mask: 0b10000010,
        },
        ("ora", IndirectY) => Op {
            code: 0x11,
            size: 2,
            cycles: 5,
            check: true,
            mask: 0b10000010,
        },
        ("pha", NoMode) => Op {
            code: 0x48,
            size: 1,
            cycles: 3,
            check: false,
            mask: 0b00000000,
        },
        ("php", NoMode) => Op {
            code: 0x08,
            size: 1,
            cycles: 3,
            check: false,
            mask: 0b00000000,
        },
        ("pla", NoMode) => Op {
            code: 0x68,
            size: 1,
            cycles: 4,
            check: false,
            mask: 0b10000010,
        },
        ("plp", NoMode) => Op {
            code: 0x28,
            size: 1,
            cycles: 4,
            check: false,
            mask: 0b11011111,
        },
        ("rol", NoMode) => Op {
            code: 0x2A,
            size: 1,
            cycles: 2,
            check: false,
            mask: 0b10000011,
        },
        ("rol", ZeroPage) => Op {
            code: 0x26,
            size: 2,
            cycles: 5,
            check: false,
            mask: 0b10000011,
        },
        ("rol", ZeroPageX) => Op {
            code: 0x36,
            size: 2,
            cycles: 6,
            check: false,
            mask: 0b10000011,
        },
        ("rol", Absolute) => Op {
            code: 0x2E,
            size: 3,
            cycles: 6,
            check: false,
            mask: 0b10000011,
        },
        ("rol", AbsoluteX) => Op {
            code: 0x3E,
            size: 3,
            cycles: 7,
            check: false,
            mask: 0b10000011,
        },
        ("ror", NoMode) => Op {
            code: 0x6A,
            size: 1,
            cycles: 2,
            check: false,
            mask: 0b10000011,
        },
        ("ror", ZeroPage) => Op {
            code: 0x66,
            size: 2,
            cycles: 5,
            check: false,
            mask: 0b10000011,
        },
        ("ror", ZeroPageX) => Op {
            code: 0x76,
            size: 2,
            cycles: 6,
            check: false,
            mask: 0b10000011,
        },
        ("ror", Absolute) => Op {
            code: 0x6E,
            size: 3,
            cycles: 6,
            check: false,
            mask: 0b10000011,
        },
        ("ror", AbsoluteX) => Op {
            code: 0x7E,
            size: 3,
            cycles: 7,
            check: false,
            mask: 0b10000011,
        },
        ("rti", NoMode) => Op {
            code: 0x40,
            size: 1,
            cycles: 6,
            check: false,
            mask: 0b11011111,
        },
        ("rts", NoMode) => Op {
            code: 0x60,
            size: 0,
            cycles: 6,
            check: false,
            mask: 0b00000000,
        },
        ("sbc", Immediate) => Op {
            code: 0xE9,
            size: 2,
            cycles: 2,
            check: false,
            mask: 0b11000011,
        },
        ("sbc", ZeroPage) => Op {
            code: 0xE5,
            size: 2,
            cycles: 3,
            check: false,
            mask: 0b11000011,
        },
        ("sbc", ZeroPageX) => Op {
            code: 0xF5,
            size: 2,
            cycles: 4,
            check: false,
            mask: 0b11000011,
        },
        ("sbc", Absolute) => Op {
            code: 0xED,
            size: 3,
            cycles: 4,
            check: false,
            mask: 0b11000011,
        },
        ("sbc", AbsoluteX) => Op {
            code: 0xFD,
            size: 3,
            cycles: 4,
            check: true,
            mask: 0b11000011,
        },
        ("sbc", AbsoluteY) => Op {
            code: 0xF9,
            size: 3,
            cycles: 4,
            check: true,
            mask: 0b11000011,
        },
        ("sbc", IndirectX) => Op {
            code: 0xE1,
            size: 2,
            cycles: 6,
            check: false,
            mask: 0b11000011,
        },
        ("sbc", IndirectY) => Op {
            code: 0xF1,
            size: 2,
            cycles: 5,
            check: true,
            mask: 0b11000011,
        },
        ("sec", NoMode) => Op {
            code: 0x38,
            size: 1,
            cycles: 2,
            check: false,
            mask: 0b00000001,
        },
        ("sed", NoMode) => Op {
            code: 0xF8,
            size: 1,
            cycles: 2,
            check: false,
            mask: 0b00001000,
        },
        ("sei", NoMode) => Op {
            code: 0x78,
            size: 1,
            cycles: 2,
            check: false,
            mask: 0b00000100,
        },
        ("sta", ZeroPage) => Op {
            code: 0x85,
            size: 2,
            cycles: 3,
            check: false,
            mask: 0b00000000,
        },
        ("sta", ZeroPageX) => Op {
            code: 0x95,
            size: 2,
            cycles: 4,
            check: false,
            mask: 0b00000000,
        },
        ("sta", Absolute) => Op {
            code: 0x8D,
            size: 3,
            cycles: 4,
            check: false,
            mask: 0b00000000,
        },
        ("sta", AbsoluteX) => Op {
            code: 0x9D,
            size: 3,
            cycles: 5,
            check: false,
            mask: 0b00000000,
        },
        ("sta", AbsoluteY) => Op {
            code: 0x99,
            size: 3,
            cycles: 5,
            check: false,
            mask: 0b00000000,
        },
        ("sta", IndirectX) => Op {
            code: 0x81,
            size: 2,
            cycles: 6,
            check: false,
            mask: 0b00000000,
        },
        ("sta", IndirectY) => Op {
            code: 0x91,
            size: 2,
            cycles: 6,
            check: false,
            mask: 0b00000000,
        },
        ("stx", ZeroPage) => Op {
            code: 0x86,
            size: 2,
            cycles: 3,
            check: false,
            mask: 0b00000000,
        },
        ("stx", ZeroPageY) => Op {
            code: 0x96,
            size: 2,
            cycles: 4,
            check: false,
            mask: 0b00000000,
        },
        ("stx", Absolute) => Op {
            code: 0x8E,
            size: 3,
            cycles: 4,
            check: false,
            mask: 0b00000000,
        },
        ("sty", ZeroPage) => Op {
            code: 0x84,
            size: 2,
            cycles: 3,
            check: false,
            mask: 0b00000000,
        },
        ("sty", ZeroPageX) => Op {
            code: 0x94,
            size: 2,
            cycles: 4,
            check: false,
            mask: 0b00000000,
        },
        ("sty", Absolute) => Op {
            code: 0x8C,
            size: 3,
            cycles: 4,
            check: false,
            mask: 0b00000000,
        },
        ("tax", NoMode) => Op {
            code: 0xAA,
            size: 1,
            cycles: 2,
            check: false,
            mask: 0b10000010,
        },
        ("tay", NoMode) => Op {
            code: 0xA8,
            size: 1,
            cycles: 2,
            check: false,
            mask: 0b10000010,
        },
        ("tsx", NoMode) => Op {
            code: 0xBA,
            size: 1,
            cycles: 2,
            check: false,
            mask: 0b10000010,
        },
        ("txa", NoMode) => Op {
            code: 0x8A,
            size: 1,
            cycles: 2,
            check: false,
            mask: 0b10000010,
        },
        ("txs", NoMode) => Op {
            code: 0x9A,
            size: 1,
            cycles: 2,
            check: false,
            mask: 0b00000000,
        },
        ("tya", NoMode) => Op {
            code: 0x98,
            size: 1,
            cycles: 2,
            check: false,
            mask: 0b10000010,
        },
        (_, _) => panic!("invalid instruction"),
    }
}
// #endregion
