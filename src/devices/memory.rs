/**
 * Device: Memory
 * 
 * This device is meant to emulate the memory of the computer. It provides two types of memory:
 * - RAM: Random Access Memory
 * - ROM: Read Only Memory
 * 
 * Both types are backed by a vector of bytes. The RAM is mutable, while the ROM is not. They will both provide the same
 * API, but the ROM will ignore any write operations.
 */

use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub enum MemoryType {
    RAM,
    ROM,
    MMIO
}

#[derive(Debug)]
pub enum MemoryError {
    OutOfBounds,
    Overlap,
    ReadOnly,
    WriteOnly,
    Unmapped
}

pub type MemoryReadResult = Result<u8, MemoryError>;
pub type MemoryWriteResult = Result<(), MemoryError>;

pub trait Memory: std::fmt::Debug {
    fn read(&self, address: u16) -> MemoryReadResult;
    fn write(&mut self, address: u16, value: u8) -> MemoryWriteResult;
    fn load(&mut self, data: Vec<u8>) -> MemoryWriteResult;
    fn type_of(&self) -> MemoryType;
}

#[derive(Debug)]
pub struct ROM {
    data: Vec<u8>,
    size: u32,
    offset: u32
}

impl ROM {
    pub fn new(data: Vec<u8>, size: u32, offset: u32) -> ROM {
        ROM {
            data,
            size,
            offset
        }
    }
}

impl Memory for ROM {
    fn read(&self, address: u16) -> MemoryReadResult {
        let address = address as u32;
        if address >= self.offset && address < self.offset + self.size {
            Ok(self.data[(address - self.offset) as usize])
        } else {
            //panic!("ROM: Address out of bounds: {:#06x}", address);
            Err(MemoryError::OutOfBounds)
        }
    }

    fn write(&mut self, _address: u16, _value: u8) -> MemoryWriteResult {
        // Ignore writes
        Ok(())
    }

    fn type_of(&self) -> MemoryType {
        MemoryType::ROM
    }

    fn load(&mut self, data: Vec<u8>) -> MemoryWriteResult {
        if data.len() as u32 > self.size {
            //panic!("ROM: Data size does not match ROM size: {:#06x} != {:#06x}", data.len(), self.size);
            return Err(MemoryError::OutOfBounds);
        }
        self.data.clear();
        self.data.resize(self.size as usize, 0);
        for i in 0..data.len() {
            self.data[i] = data[i];
        }

        Ok(())
    }
}

impl Index<u16> for ROM {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        let offset = self.offset as u16;
        &self.data[(index - offset) as usize]
    }
}

#[derive(Debug)]
pub struct RAM {
    data: Vec<u8>,
    size: u32,
    offset: u32
}

impl RAM {
    pub fn new(data: Vec<u8>, size: u32, offset: u32) -> RAM {
        RAM {
            data,
            size,
            offset
        }
    }
}

impl Memory for RAM {
    fn read(&self, address: u16) -> MemoryReadResult {
        let address = address as u32;
        if address >= self.offset && address < self.offset + self.size {
            Ok(self.data[(address - self.offset) as usize])
        } else {
            Err(MemoryError::OutOfBounds)
        }
    }

    fn write(&mut self, address: u16, value: u8) -> MemoryWriteResult {
        let address = address as u32;
        if address >= self.offset && address < self.offset + self.size {
            self.data[(address - self.offset) as usize] = value;
            Ok(())
        } else {
            Err(MemoryError::OutOfBounds)
        }
    }

    fn type_of(&self) -> MemoryType {
        MemoryType::RAM
    }

    fn load(&mut self, data: Vec<u8>) -> MemoryWriteResult {
        if data.len() as u32 > self.size {
            //panic!("ROM: Data size does not match ROM size: {:#06x} != {:#06x}", data.len(), self.size);
            return Err(MemoryError::OutOfBounds);
        }
        self.data.clear();
        self.data.resize(self.size as usize, 0);
        for i in 0..data.len() {
            self.data[i] = data[i];
        }

        Ok(())
    }
}

impl Index<u16> for RAM {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        let offset = self.offset as u16;
        &self.data[(index - offset) as usize]
    }
}

impl IndexMut<u16> for RAM {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        let offset = self.offset as u16;
        &mut self.data[(index - offset) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rom() -> Result<(), MemoryError> {
        let rom = ROM::new(vec![0x12, 0x34, 0x56, 0x78], 4, 0x1000);
        assert_eq!(rom.read(0x1000)?, 0x12);
        assert_eq!(rom.read(0x1001)?, 0x34);
        assert_eq!(rom.read(0x1002)?, 0x56);
        assert_eq!(rom.read(0x1003)?, 0x78);
        Ok(())
    }

    #[test]
    fn rom_out_of_bounds() -> Result<(), String> {
        let rom = ROM::new(vec![0x12, 0x34, 0x56, 0x78], 4, 0x1000);
        assert!(rom.read(0x1004).is_err());
        match rom.read(0x1004) {
            Ok(_) => Err(String::from("ROM: Address should be out of bounds")),
            Err(memory_error) => {
                match memory_error {
                    MemoryError::OutOfBounds => Ok(()),
                    _ => Err(String::from("ROM: Address should be out of bounds"))
                }
            }
        }
    }

    #[test]
    fn ram() -> Result<(), MemoryError> {
        let ram = RAM::new(vec![0x12, 0x34, 0x56, 0x78], 4, 0x1000);
        assert_eq!(ram.read(0x1000)?, 0x12);
        assert_eq!(ram.read(0x1001)?, 0x34);
        assert_eq!(ram.read(0x1002)?, 0x56);
        assert_eq!(ram.read(0x1003)?, 0x78);
        Ok(())
    }

    #[test]
   fn ram_out_of_bounds() -> Result<(), String> {
        let ram = RAM::new(vec![0x12, 0x34, 0x56, 0x78], 4, 0x1000);
        match ram.read(0x1004) {
            Ok(_) => Err(String::from("RAM: Address should be out of bounds")),
            Err(memory_error) => {
                match memory_error {
                    MemoryError::OutOfBounds => Ok(()),
                    _ => Err(String::from("RAM: Address should be out of bounds"))
                }
            }
        }
    }

    #[test]
    fn ram_write() -> Result<(), MemoryError> {
        let mut ram = RAM::new(vec![0x12, 0x34, 0x56, 0x78], 4, 0x1000);
        assert_eq!(ram.read(0x1000)?, 0x12);
        let _ = ram.write(0x1000, 0x11);
        assert_eq!(ram.read(0x1000)?, 0x11);
        Ok(())
    }

    #[test]
    fn ram_write_out_of_bounds() -> Result<(), String> {
        let mut ram = RAM::new(vec![0x12, 0x34, 0x56, 0x78], 4, 0x1000);
        match ram.write(0x1004, 0x11) {
            Ok(_) => Err(String::from("RAM: Address should be out of bounds")),
            Err(memory_error) => {
                match memory_error {
                    MemoryError::OutOfBounds => Ok(()),
                    _ => Err(String::from("RAM: Address should be out of bounds"))
                }
            }
        }
    }

    #[test]
    fn rom_index() {
        let rom = ROM::new(vec![0x12, 0x34, 0x56, 0x78], 4, 0x1000);
        assert_eq!(rom[0x1000], 0x12);
        assert_eq!(rom[0x1001], 0x34);
        assert_eq!(rom[0x1002], 0x56);
        assert_eq!(rom[0x1003], 0x78);
    }

    #[test]
    #[should_panic]
    fn rom_index_out_of_bounds() {
        let rom = ROM::new(vec![0x12, 0x34, 0x56, 0x78], 4, 0x1000);
        rom[0x1004];
    }

    #[test]
    fn ram_index() {
        let ram = RAM::new(vec![0x12, 0x34, 0x56, 0x78], 4, 0x1000);
        assert_eq!(ram[0x1000], 0x12);
        assert_eq!(ram[0x1001], 0x34);
        assert_eq!(ram[0x1002], 0x56);
        assert_eq!(ram[0x1003], 0x78);
    }

    #[test]
    #[should_panic]
    fn ram_index_out_of_bounds() {
        let ram = RAM::new(vec![0x12, 0x34, 0x56, 0x78], 4, 0x1000);
        ram[0x1004];
    }

    #[test]
    fn ram_index_write() {
        let mut ram = RAM::new(vec![0x12, 0x34, 0x56, 0x78], 4, 0x1000);
        ram[0x1000] = 0x11;
        assert_eq!(ram[0x1000], 0x11);
    }

    #[test]
    #[should_panic]
    fn ram_index_write_out_of_bounds() {
        let mut ram = RAM::new(vec![0x12, 0x34, 0x56, 0x78], 4, 0x1000);
        ram[0x1004] = 0x11;
    }

    #[test]
    fn ram_index_mut() {
        let mut ram = RAM::new(vec![0x12, 0x34, 0x56, 0x78], 4, 0x1000);

        let value = &mut ram[0x1000];
        *value = 0x11;
        assert_eq!(ram[0x1000], 0x11);
    }

    #[test]
    #[should_panic]
    fn ram_index_mut_out_of_bounds() {
        let mut ram = RAM::new(vec![0x12, 0x34, 0x56, 0x78], 4, 0x1000);

        let value = &mut ram[0x1004];
        *value = 0x11;
    }

    #[test]
    fn rom_load() -> Result<(), MemoryError> {
        let mut rom = ROM::new(vec![0x12, 0x34, 0x56, 0x78], 4, 0x1000);

        let data = vec![0x11, 0x22, 0x33, 0x44];
        rom.load(data)?;

        assert_eq!(rom.read(0x1000)?, 0x11);
        assert_eq!(rom.read(0x1001)?, 0x22);
        assert_eq!(rom.read(0x1002)?, 0x33);
        assert_eq!(rom.read(0x1003)?, 0x44);

        Ok(())
    }

    #[test]
    fn rom_load_out_of_bounds() -> Result<(), String> {
        let mut rom = ROM::new(vec![0x12, 0x34, 0x56, 0x78], 4, 0x1000);

        let data = vec![0x11, 0x22, 0x33, 0x44, 0x55];
        match rom.load(data) {
            Ok(_) => Err(String::from("ROM: Data size should be out of bounds")),
            Err(memory_error) => {
                match memory_error {
                    MemoryError::OutOfBounds => Ok(()),
                    _ => Err(String::from("ROM: Data size should be out of bounds"))
                }
            }
        }
    }

    #[test]
    fn rom_load_fill() -> Result<(), MemoryError> {
        let mut rom = ROM::new(vec![0x12, 0x34, 0x56, 0x78], 4, 0x1000);

        let data = vec![0x11, 0x22];
        rom.load(data)?;
        assert_eq!(rom.size, 4);

        assert_eq!(rom.read(0x1000)?, 0x11);
        assert_eq!(rom.read(0x1001)?, 0x22);
        assert_eq!(rom.read(0x1002)?, 0x00);
        assert_eq!(rom.read(0x1003)?, 0x00);

        Ok(())
    }

}