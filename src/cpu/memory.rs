pub struct Memory {
    pub memory: [u8; 65536],
}

impl Memory {
    pub fn new() -> Memory {
        Memory { memory: [0; 65536] }
    }
    pub fn print(self: &Self) {
        let mut debug_string = String::new();
        for x in 0..self.memory.len() {
            debug_string.push_str(&format!("{:04X}: {value}, ", x, value = self.memory[x]));
            if x != 0 && (x + 1) % 16 == 0 {
                debug_string.push_str("\n");
            }
        }
        println!("Memory: {}", debug_string)
    }
    pub fn print_value(self: &Self, memory_address: usize) {
        println!(
            "{:04X}: {value}",
            memory_address,
            value = self.memory[memory_address]
        )
    }
}
