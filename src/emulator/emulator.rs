use crate::cpu::cpu::*;
use crate::devices::memory::*;

#[derive(Debug)]
pub struct Emulator {
    cpu: CPU,
    ram: RAM,
    rom: ROM
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            cpu: CPU::new(),
            // 16KB of RAM starting at 0x0000
            ram: RAM::new(vec![0; 0x4000], 0x4000, 0),

            // 32KB of ROM starting at 0x8000
            rom: ROM::new(vec![0; 0x8000], 0x8000, 0x8000)
        }
    }

    pub fn warm_reset(&mut self) {
        self.cpu.reset();
    }

    pub fn cold_reset(&mut self) {
        // Zero out the RAM
        for i in 0..0x4000 {
            self.ram.write(i, 0);
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
        
        // Load RAM and ROM with test data
        for i in 0..0x4000 {
            emulator.ram.write(i, i as u8);
        }

        for i in 0..0x8000 {
            emulator.rom.write(i, i as u8);
        }
    }
}