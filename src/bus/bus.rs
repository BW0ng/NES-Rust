// TODO Remove all dead code.
#[allow(dead_code)]
pub struct Interrupt {
    schedule: Option<u8>,
}

#[allow(dead_code)]
impl Interrupt {
    fn new() -> Self {
        Interrupt { schedule: None }
    }

    fn tick(&mut self) {
        match self.schedule.as_mut() {
            Some(x) => {
                if *x > 0 {
                    *x -= 1
                }
            }
            None => (),
        };
    }

    pub fn schedule(&mut self, value: u8) {
        self.schedule = Some(value);
    }

    pub fn acknowledge(&mut self) {
        self.schedule = None;
    }

    pub fn ready(&self) -> bool {
        match self.schedule {
            Some(x) => x == 0,
            None => false,
        }
    }
}

#[allow(dead_code)]
pub struct Bus {
    pub ram: [u8; 2048],
    // TODO add APU, PPU, Controller, Cartridge,
    pub cycles: u64,
    pub nmi: Interrupt,
    cpu_stall_cycles: usize,
}

#[allow(dead_code)]
impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: [0; 2048],
            cycles: 0,
            nmi: Interrupt::new(),
            cpu_stall_cycles: 0,
        }
    }

    // TODO implement stall cycles

    pub fn tick(&mut self) {
        self.cycles += 1;

        // TODO Handle ticking of apu/ppu
    }

    // Public for debugging in cpu.
    pub fn unclocked_read_byte(&mut self, address: u16) -> u8 {
        match address {
            0..=0x1FFF => return self.ram[address as usize % 0x0800],
            // TODO implement apu/ppu memory.
            address => (address >> 8) as u8, // TODO Figure out what this is for.
        }
    }

    fn unclocked_write_byte(&mut self, address: u16, value: u8) {
        match address {
            0..=0x1FFF => self.ram[address as usize % 0x0800] = value,
            // TODO implement apu/ppu addressing
            // TODO implement dma.
            _ => (),
        }
    }

    pub fn read_byte<T: Into<u16>>(&mut self, address: T) -> u8 {
        self.tick();
        self.unclocked_read_byte(address.into())
    }
    pub fn write_byte<T: Into<u16>>(&mut self, address: T, value: u8) {
        self.tick();
        self.unclocked_write_byte(address.into(), value)
    }
    pub fn read_noncontinuous_word<T: Into<u16>>(&mut self, address_1: T, address_2: T) -> u16 {
        self.read_byte(address_1) as u16 | (self.read_byte(address_2) as u16) << 8
    }
    pub fn read_word<T: Into<u16>>(&mut self, address: T) -> u16 {
        let address = address.into();
        self.read_noncontinuous_word(address, address + 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_tick() {
        let mut bus = Bus::new();
        assert_eq!(0, bus.cycles);
        bus.tick();
        bus.tick();
        bus.tick();
        assert_eq!(3, bus.cycles)
    }

    #[test]
    fn test_unclocked_write_byte_ram() {
        let mut bus = Bus::new();
        assert_eq!(0, bus.ram[1234]);
        bus.unclocked_write_byte(1234, 96);
        assert_eq!(96, bus.ram[1234]);

        assert_eq!(0, bus.ram[45]);
        bus.unclocked_write_byte(45, 12);
        assert_eq!(12, bus.ram[45]);
    }

    #[test]
    fn test_unclocked_read_byte_ram() {
        let mut bus = Bus::new();
        assert_eq!(0, bus.ram[1234]);
        bus.ram[1234] = 45;
        assert_eq!(45, bus.unclocked_read_byte(1234));

        assert_eq!(0, bus.ram[123]);
        bus.ram[123] = 32;
        assert_eq!(32, bus.unclocked_read_byte(123));
    }

    #[test]
    fn test_read_byte_ram() {
        let mut bus = Bus::new();
        for i in 0..2048 {
            bus.ram[i] = (i % 256) as u8;
        }

        for i in 0..2048 {
            assert_eq!((i % 256) as u8, bus.read_byte(i as u16));
            assert_eq!((i as u64 + 1), bus.cycles);
        }
    }

    #[test]
    fn test_write_byte_ram() {
        let mut bus = Bus::new();
        for i in 0..2048 {
            bus.write_byte(i as u16, (i % 256) as u8);
            assert_eq!((i % 256) as u8, bus.ram[i]);
            assert_eq!((i as u64 + 1), bus.cycles);
        }
    }

    #[test]
    fn test_read_noncontinuous_word_ram() {
        let mut bus = Bus::new();
        for i in 0..2048 {
            bus.ram[i] = (i % 256) as u8;
        }

        for i in 0..2046 {
            assert_eq!(
                (i % 256) as u16 | ((i + 2 % 256) as u16) << 8,
                bus.read_noncontinuous_word(i as u16, i + 2 as u16)
            );
            assert_eq!((i as u64 + 1) * 2, bus.cycles);
        }
    }

    #[test]
    fn test_read_word_ram() {
        let mut bus = Bus::new();
        for i in 0..2048 {
            bus.ram[i] = (i % 256) as u8;
        }

        for i in 0..2048 {
            assert_eq!(
                (i % 256) as u16 | ((i + 1 % 256) as u16) << 8,
                bus.read_word(i as u16)
            );
            assert_eq!((i as u64 + 1) * 2, bus.cycles);
        }
    }

    /**
     * Testing behavior if accessed address is out of bounds.
     */
    #[test]
    fn test_unclocked_write_byte_oob() {
        let mut bus = Bus::new();
        assert_eq!((), bus.unclocked_write_byte(0xFFFF, 12))
    }

    #[test]
    fn test_unclocked_read_byte_oob() {
        let mut bus = Bus::new();
        assert_eq!(0x0FF, bus.unclocked_read_byte(0xFFFF))
    }
}
