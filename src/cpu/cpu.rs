use super::cpu_debug::{INSTRUCTION_NAMES, INSTRUCTION_SIZES};
use crate::bus::bus::Bus;

use std::fmt::Write;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::convert::TryInto;
#[allow(dead_code)]
// #region
enum Flag {
    Carry = 0b00000001,
    Zero = 0b00000010,
    IrqDisable = 0b00000100,
    Decimal = 0b00001000,
    Break = 0b00010000,
    Push = 0b00100000,
    Overflow = 0b01000000,
    Negative = 0b10000000,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Mode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteXForceTick,
    AbsoluteYForceTick,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    IndirectYForceTick,
    NoMode,
}

#[allow(dead_code)] // TODO Remove after done Debugging
impl Display for Mode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

// TODO Implement Interrupts
#[allow(dead_code)]
pub struct CPU {
    pub bus: Bus,
    pc: u16,
    stack_pointer: u8,
    a: u8,
    x: u8,
    y: u8,
    status: u8,
}

#[allow(dead_code)]
impl CPU {
    pub fn new(bus:Bus) -> Self {
        CPU {
            bus: bus,
            pc: 0,
            stack_pointer: 0,
            a: 0,
            x: 0,
            y: 0,
            status: 0,
        }
    }
    pub fn reset(&mut self) {
        self.stack_pointer = 0xFF;
        /*
         * Sets the status register to 0x34 (0b0011_0100)
         * Negative - 0
         * Overflow - 0
         * B Flag 1 - 1
         * B Flag 0 - 1
         * Decimal - 0
         * Interrupt Disable - 1
         * Zero - 0
         * Carry - 0
         */
        self.status = 0b00110100
    }

    /* Stack starts at 0x100 */
    fn pop_byte(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let address = 0x100 + self.stack_pointer as u16;
        self.bus.read_byte(address)
    }

    fn pop_word(&mut self) -> u16 {
        self.pop_byte() as u16 | (self.pop_byte() as u16) << 8
    }

    /* Stack starts at 0x100 */
    fn push_byte(&mut self, value: u8) {
        let address = 0x100 + self.stack_pointer as u16;
        self.bus.write_byte(address, value);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn push_word(&mut self, value: u16) {
        // 6502 is Big Endian
        self.push_byte((value >> 8) as u8);
        self.push_byte(value as u8);
    }

    fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }

    fn next_byte(&mut self) -> u8 {
        let starting_pc = self.pc;
        self.increment_pc();
        self.bus.read_byte(starting_pc)
    }

    fn next_word(&mut self) -> u16 {
        let starting_pc = self.pc;
        self.increment_pc();
        self.increment_pc();
        self.bus.read_word(starting_pc)
    }

    fn get_flag(&self, flag: Flag) -> bool {
        self.status & (flag as u8) != 0
    }

    fn set_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.status |= flag as u8;
        } else {
            self.status &= !(flag as u8);
        }
    }

    fn set_flags_zero_negative(&mut self, value: u8) {
        self.set_flag(Flag::Zero, value == 0);
        self.set_flag(Flag::Negative, (value & 0b1000_0000) != 0);
    }

    fn set_flags_carry_overflow(&mut self, m: u8, n: u8, result: u16) {
        self.set_flag(Flag::Carry, result > 0xFF);
        let r = result as u8;
        // formula found here: http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
        self.set_flag(Flag::Overflow, (m ^ r) & (n ^ r) & 0b1000_0000 != 0)
    }

    // http://www.obelisk.me.uk/6502/addressing.html
    fn operand_address(&mut self, mode: Mode) -> u16 {
        match mode {
            Mode::Immediate => {
                let starting_pc = self.pc;
                self.increment_pc();
                starting_pc
            }
            Mode::ZeroPage => self.next_byte() as u16,
            Mode::ZeroPageX => {
                self.bus.tick();
                low_byte(offset(self.next_byte(), self.x))
            }
            Mode::ZeroPageY => {
                self.bus.tick();
                low_byte(offset(self.next_byte(), self.y))
            }
            Mode::Absolute => self.next_word(),
            Mode::AbsoluteX => {
                let base = self.next_word();
                if cross(base, self.x) {
                    // Ticking if x isn't zero.
                    self.bus.tick();
                };
                offset(base, self.x)
            }
            Mode::AbsoluteXForceTick => {
                self.bus.tick();
                offset(self.next_word(), self.x)
            }
            Mode::AbsoluteY => {
                let base = self.next_word();
                if cross(base, self.y) {
                    // Ticking if x isn't zero.
                    self.bus.tick();
                };
                offset(base, self.y)
            }
            Mode::AbsoluteYForceTick => {
                self.bus.tick();
                offset(self.next_word(), self.y)
            }
            Mode::Indirect => {
                let i = self.next_word();
                self.bus
                    .read_noncontinuous_word(i, high_byte(i) | (low_byte(i + 1)))
            }
            Mode::IndirectX => {
                self.bus.tick();
                let i = offset(self.next_byte(), self.x);
                self.bus
                    .read_noncontinuous_word(low_byte(i), low_byte(i + 1))
            }
            Mode::IndirectY => {
                let i = self.next_byte();
                let base = self
                    .bus
                    .read_noncontinuous_word(i, low_byte(i + 1).try_into().unwrap());
                if cross(base, self.y) {
                    self.bus.tick();
                }
                offset(base, self.y)
            }
            Mode::IndirectYForceTick => {
                let i = self.next_byte();
                let base = self
                    .bus
                    .read_noncontinuous_word(i, low_byte(i + 1).try_into().unwrap());
                self.bus.tick();
                offset(base, self.y)
            }
            Mode::NoMode => panic!("Mode:NoMode should never be used."),
        }
    }

    fn read_operand(&mut self, mode: Mode) -> u8 {
        let address = self.operand_address(mode);
        self.bus.read_byte(address)
    }

    #[allow(dead_code)]
    pub fn log_next_instructions(&mut self) {
        let pc = self.pc;
        let rom_offset = 15 + (self.pc % 0x4000); // TODO Need to figure out what this is. 
        let opcode = self.bus.unclocked_read_byte(pc) as usize;
        let mut output = String::new();
        for i in 1..INSTRUCTION_SIZES[opcode] {
            write!(&mut output, "{:02X} ", self.bus.unclocked_read_byte(pc + i)).expect("it to work");
        }
        println!(
            "OFFSET:{:06X}\tPC:{:04X}\tA:{:02X}\tX:{:02X}\tY:{:02X}\tStatus:{:08b}\tTEST:{:02X}[{:02X}] {}\t{}",
            rom_offset,
            pc,
            self.a,
            self.x,
            self.y,
            self.status,
            self.bus.unclocked_read_byte(0x6000),
            opcode, 
            INSTRUCTION_NAMES[opcode as usize],
            output,
        )
    }

    pub fn execute_next_instruction(&mut self) {
        // if self.bus.nmi.ready() {
        //     self.bus.nmi.acknowledge();
        //     self.interrupt(Interrupt::Nmi)
        // } else if self.bus.irq() && !self.get_flag(Flag::IrqDisable) {
        //     self.interrupt(Interrupt::Irq)
        // }

        #[cfg(feature = "log")]
        self.log_next_instruction();

        let instruction = self.next_byte();
        self.execute_instruction(instruction);
    }
    // #endregion
    // https://www.masswerk.at/6502/6502_instruction_set.html
    fn execute_instruction(&mut self, opcode: u8) {
        match opcode {
            // #region
            // Loads
            0xA1 => self.lda(Mode::IndirectX),
            0xA5 => self.lda(Mode::ZeroPage),
            0xA9 => self.lda(Mode::Immediate),
            0xAD => self.lda(Mode::Absolute),
            0xB1 => self.lda(Mode::IndirectY),
            0xB5 => self.lda(Mode::ZeroPageX),
            0xB9 => self.lda(Mode::AbsoluteY),
            0xBD => self.lda(Mode::AbsoluteX),

            0xA2 => self.ldx(Mode::Immediate),
            0xA6 => self.ldx(Mode::ZeroPage),
            0xAE => self.ldx(Mode::Absolute),
            0xB6 => self.ldx(Mode::ZeroPageY),
            0xBE => self.ldx(Mode::AbsoluteY),

            0xA0 => self.ldy(Mode::Immediate),
            0xA4 => self.ldy(Mode::ZeroPage),
            0xAC => self.ldy(Mode::Absolute),
            0xB4 => self.ldy(Mode::ZeroPageX),
            0xBC => self.ldy(Mode::AbsoluteX),
            // #endregion
            
            // Stores
            0x81 => self.sta(Mode::IndirectX),
            0x85 => self.sta(Mode::ZeroPage),
            0x8D => self.sta(Mode::Absolute),
            0x91 => self.sta(Mode::IndirectY),
            0x95 => self.sta(Mode::ZeroPageX),
            0x99 => self.sta(Mode::AbsoluteY),
            0x9D => self.sta(Mode::AbsoluteX),







            _ => println!("Opcode: 0x{:X} not implemented yet.", opcode)
        }
    }

    // Loads
    fn lda(&mut self, mode: Mode) {
        let operand = self.read_operand(mode);
        self.set_flags_zero_negative(operand);
        self.a = operand
    }

    fn ldx(&mut self, mode: Mode) {
        let operand = self.read_operand(mode);
        self.set_flags_zero_negative(operand);
        self.x = operand
    }
    fn ldy(&mut self, mode: Mode) {
        let operand = self.read_operand(mode);
        self.set_flags_zero_negative(operand);
        self.y = operand
    }

    // Stores
    fn sta(&mut self, mode: Mode) {

        let address = self.operand_address(mode);
        let value = self.a;
        self.bus.write_byte(address, value);
    }
}

// #region
fn cross(base: u16, offset: u8) -> bool {
    high_byte(base + offset as u16) != high_byte(base)
}

fn offset<T: Into<u16>>(base: T, offset: u8) -> u16 {
    base.into() + offset as u16
}

fn high_byte<T: Into<u16>>(value: T) -> u16 {
    value.into() & 0xFF00
}

fn low_byte<T: Into<u16>>(value: T) -> u16 {
    value.into() & 0xFF
}

#[allow(dead_code)]
fn bytes_to_word(low: u8, high: u8) -> u16 {
    low as u16 | ((high as u16) << 8)
}

#[allow(dead_code)]
fn word_to_bytes(value: u16) -> (u8, u8) {
    ((value & 0xFF) as u8, ((value & 0xFF00) >> 8) as u8)
}
// #endregion

// Copied from starrhorne
// https://github.com/starrhorne/nes-rust/blob/master/src/cpu_test.rs
#[cfg(test)]
#[path = "./test/cpu_test.rs"]
mod cpu_test;