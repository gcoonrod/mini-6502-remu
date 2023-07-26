use crate::cpu::cpu::*;
use crate::devices::memory::*;
use crate::devices::memory_map::*;

#[derive(Debug)]
pub struct Emulator {
    cpu: CPU,
    memory_map: MemoryMap
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            cpu: CPU::new(),
            memory_map: MemoryMap::new()
        }
    }

    pub fn init(&mut self) {
        // Create a MemoryMap and add the RAM and ROM to it
        self.memory_map.create(String::from("RAM"), MemoryType::RAM, 0x4000, 0x0000).unwrap();
        self.memory_map.create(String::from("IO"), MemoryType::MMIO, 0x4000, 0x4000).unwrap();
        self.memory_map.create(String::from("ROM"), MemoryType::ROM, 0x8000, 0x8000).unwrap();
    }

    pub fn warm_reset(&mut self) {
        self.cpu.reset();
    }

    pub fn cold_reset(&mut self) {
        // Zero out the RAM
        for i in 0..0x4000 {
            self.memory_map.write(i, 0).unwrap();
        }

        // Reset the CPU
        self.cpu.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emulator() {
        let mut emulator = Emulator::new();
        
        // Initialize the emulator
        emulator.init();

        // Load test data in to RAM
        emulator.memory_map.write(0x0000, 0x12).unwrap();
        emulator.memory_map.write(0x0001, 0x34).unwrap();
        emulator.memory_map.write(0x0002, 0x56).unwrap();
        emulator.memory_map.write(0x0003, 0x78).unwrap();

        // Verify that the data was loaded in to RAM
        assert_eq!(emulator.memory_map.read(0x0000).unwrap(), 0x12);
        assert_eq!(emulator.memory_map.read(0x0001).unwrap(), 0x34);
        assert_eq!(emulator.memory_map.read(0x0002).unwrap(), 0x56);
        assert_eq!(emulator.memory_map.read(0x0003).unwrap(), 0x78);

        // Warm reset the emulator
        emulator.warm_reset();

        // Verify that the RAM was not cleared
        assert_eq!(emulator.memory_map.read(0x0000).unwrap(), 0x12);
        assert_eq!(emulator.memory_map.read(0x0001).unwrap(), 0x34);
        assert_eq!(emulator.memory_map.read(0x0002).unwrap(), 0x56);
        assert_eq!(emulator.memory_map.read(0x0003).unwrap(), 0x78);

        // Cold reset the emulator
        emulator.cold_reset();

        // Verify that the RAM was cleared
        assert_eq!(emulator.memory_map.read(0x0000).unwrap(), 0x00);
        assert_eq!(emulator.memory_map.read(0x0001).unwrap(), 0x00);
        assert_eq!(emulator.memory_map.read(0x0002).unwrap(), 0x00);
        assert_eq!(emulator.memory_map.read(0x0003).unwrap(), 0x00);
        
    }
}