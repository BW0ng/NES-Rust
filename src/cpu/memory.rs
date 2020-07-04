pub struct Memory {
    pub memory: [u8; 65536],
}

impl Memory {
    pub fn new() -> Memory {
        Memory { memory: [0; 65536] }
    }
    pub fn print(self: &Self) {
        let mut debugString = String::new();
        for x in 0..self.memory.len() {
            debugString.push_str(&format!("{:04X}: {value}, ", x, value = self.memory[x]));
            if x != 0 && (x + 1) % 16 == 0 {
                debugString.push_str("\n");
            }
        }
        println!("Memory: {}", debugString)
    }
    pub fn print_value(self: &Self, memoryAddress: usize) {
        println!(
            "{:04X}: {value}",
            memoryAddress,
            value = self.memory[memoryAddress]
        )
    }
}
