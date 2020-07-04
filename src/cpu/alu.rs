pub fn operate(A: u8, B: u8, Cin: bool, status: u8, opcode: u8) -> (u8, bool, bool) {
    // TODO figure out how to split out opcodes.
    let enable_sum: bool = helper::bit_set(opcode, 0);
    let enable_and: bool = helper::bit_set(opcode, 0);
    let enable_eor: bool = helper::bit_set(opcode, 0);
    let enable_or: bool = helper::bit_set(opcode, 0);
    let enable_shift_right: bool = helper::bit_set(opcode, 0);
    let enable_barrel_shift_right: bool = helper::bit_set(opcode, 0);
}

// fn overflowed(A: i8, B: i8, operation: u8, result: i8) -> bool {
//     match operation {
//         0 =>
//     }
// }
