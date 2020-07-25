mod cpu;
use cpu::alu;
use cpu::memory::Memory;
use cpu::register::Registers;
/*
 * Master - 1:1 21.477272 Mhz
 * CPU - 1:12
 * PPU - 1:4
 * APU - 1:24
 *
 */

fn main() {
    const MAX_MEMORY_ADDRESS: u16 = 65535;

    println!("Hello, world!");
    let mut memory = Memory::new();
    let mut register = Registers::new();
    alu::operate(0, 0, false, 0b000_000_01, &mut register, &mut memory);
    memory.print(Some(0xFFFE), None);
    memory.print_value(0xFFFF);
}
