/**
 * Memory Map for the 6502 Emulator
 * 
 * The MemoryMap struct is a wrapper around a collection of devices that implement the Memory trait. It provides a
 * unified interface to the CPU for reading and writing to memory. In addition, it provides some static utilities for
 * creating new instances of Memory devices and inserting them into the map. 
 */

use crate::devices::memory::*;

#[derive(Debug)]
pub enum MemoryMapError {
    Overlap,
    OutOfBounds
}

pub type MemoryMapInsertResult = Result<(), MemoryMapError>;

// MemoryMapEntry is a simple struct that holds a Memory device and the range of addresses that it occupies. It is private
// to the module.
#[derive(Debug)]
struct MemoryMapEntry {
    name: String,
    device: Box<dyn Memory>,
    size: u32,
    offset: u32
}

impl MemoryMapEntry {
    fn new(name: String, device: Box<dyn Memory>, size: u32, offset: u32) -> MemoryMapEntry {
        MemoryMapEntry {
            name,
            device,
            size,
            offset
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn device_type(&self) -> String {
        match self.device.type_of() {
            MemoryType::RAM => String::from("RAM"),
            MemoryType::ROM => String::from("ROM"),
            MemoryType::MMIO => String::from("MMIO")
        }
    }

    // Print a formatted table of the memory map in the following format:
    // Device Name | Device Type | Start Address | End Address
    pub fn print_row(&self) {
        println!("{: <12} | {: <10} | {:#06x} | {:#06x}", self.name(), self.device_type(), self.offset, self.offset + self.size - 1);
    }
}

// The MemoryMap struct is the main struct of this module. It holds a vector of MemoryMapEntry structs and provides
// methods for reading and writing to the devices in the map.
#[derive(Debug)]
pub struct MemoryMap {
    devices: Vec<MemoryMapEntry>
}

impl MemoryMap {
    pub fn new() -> MemoryMap {
        MemoryMap {
            devices: Vec::new()
        }
    }

    pub fn count(&self) -> usize {
        self.devices.len()
    }

    pub fn read(&self, address: u16) -> MemoryReadResult {
        for entry in &self.devices {
            let address = address as u32;
            if address >= entry.offset && address < entry.offset + entry.size {
                return entry.device.read(address as u16);
            }
        }

        Err(MemoryError::Unmapped)
    }

    pub fn write(&mut self, address: u16, value: u8) -> MemoryWriteResult {
        for entry in &mut self.devices {
            let address = address as u32;
            if address >= entry.offset && address < entry.offset + entry.size {
                return entry.device.write(address as u16, value);
            }
        }

        Err(MemoryError::Unmapped)
    }

    fn insert(&mut self, name: String, device: Box<dyn Memory>, size: u32, offset: u32) -> MemoryMapInsertResult {
        // Verify that the device does not overlap with any existing devices
        for entry in &self.devices {
            if offset >= entry.offset && offset < entry.offset + entry.size {
                //panic!("MemoryMap: Device overlaps with existing device: {:#06x} {}", offset, entry.name());
                return Err(MemoryMapError::Overlap);
            }

            if offset + size > entry.offset && offset + size <= entry.offset + entry.size {
                //panic!("MemoryMap: Device overlaps with existing device: {:#06x} {}", offset, entry.name());
                return Err(MemoryMapError::Overlap);
            }
        }

        self.devices.push(MemoryMapEntry::new(name, device, size, offset));
        Ok(())
    }

    pub fn create(&mut self, name: String, memory_type: MemoryType, size: u32, offset: u32) -> MemoryMapInsertResult {
        let memory = match memory_type {
            MemoryType::RAM | MemoryType::MMIO => Box::new(RAM::new(vec![0; size as usize], size, offset)) as Box<dyn Memory>,
            MemoryType::ROM => Box::new(ROM::new(vec![0; size as usize], size, offset)) as Box<dyn Memory>
        };
        
        return self.insert(name, memory, size, offset);
    }

    // Print a formatted table of the memory map in the following format:
    // Device Name | Device Type | Start Address | End Address
    pub fn print_table(&self) {
        println!("{: <12} | {: <10} | {: <12} | {: <12}", "Device Name", "Device Type", "Start Address", "End Address");
        println!("{:-<12}-+-{:-<10}-+-{:-<12}-+-{:-<12}", "", "", "", "");
        for entry in &self.devices {
            entry.print_row();
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_map() {
        // Create a new MemoryMap and assert that it is empty
        let memory_map = MemoryMap::new();
        assert_eq!(memory_map.count(), 0);
    }

    #[test]
    fn memory_map_insert(){
        // Create a new MemoryMap and insert a device
        let mut memory_map = MemoryMap::new();
        memory_map.create("RAM".to_string(), MemoryType::RAM, 0x4000, 0x0000).unwrap();
        assert_eq!(memory_map.count(), 1);

        // Insert a second device
        memory_map.create("ROM".to_string(), MemoryType::ROM, 0x8000, 0x8000).unwrap();
        assert_eq!(memory_map.count(), 2);
        
    }

    #[test]
    fn memory_map_overlap() -> Result<(), String> {
        // Create a new MemoryMap and insert a device
        let mut memory_map = MemoryMap::new();
        memory_map.create("RAM".to_string(), MemoryType::RAM, 0x4000, 0x0000).unwrap();
        assert_eq!(memory_map.count(), 1);

        // Insert a second device
        memory_map.create("ROM".to_string(), MemoryType::ROM, 0x8000, 0x8000).unwrap();
        assert_eq!(memory_map.count(), 2);

        // Insert a device that overlaps with the RAM
        match memory_map.create("More RAM".to_string(), MemoryType::RAM, 0x4000, 0x0000) {
            Ok(_) => Err(String::from("MemoryMap: Inserted device that overlaps with existing device")),
            Err(error) => {
                match error {
                    MemoryMapError::Overlap => Ok(()),
                    _ => Err(String::from("MemoryMap: Inserted device that overlaps with existing device"))
                }
            }
        }
    }

    #[test]
    fn memory_map_unmapped() -> Result<(), String> {
        // Create a new MemoryMap and insert a device
        let mut memory_map = MemoryMap::new();
        memory_map.create("RAM".to_string(), MemoryType::RAM, 0x4000, 0x0000).unwrap();
        assert_eq!(memory_map.count(), 1);

        // Write to an address that is unmapped
        match memory_map.write(0x8000, 0x12) {
            Ok(_) => Err(String::from("MemoryMap: Wrote to an unmapped address")),
            Err(error) => {
                match error {
                    MemoryError::Unmapped => Ok(()),
                    _ => Err(String::from("MemoryMap: Wrote to an unmapped address"))
                }
            }
        }
    }

    #[test]
    fn memory_map_read_write() {
        // Create a new MemoryMap and insert a RAM device
        let mut memory_map = MemoryMap::new();
        memory_map.create("RAM".to_string(), MemoryType::RAM, 0x4000, 0x0000).unwrap();
        assert_eq!(memory_map.count(), 1);

        // Insert a ROM device
        memory_map.create("ROM".to_string(), MemoryType::ROM, 0x8000, 0x8000).unwrap();
        assert_eq!(memory_map.count(), 2);

        // Write to the RAM
        memory_map.write(0x0000, 0x12).unwrap();
        assert_eq!(memory_map.read(0x0000).unwrap(), 0x12);

        // Write to the ROM and verify that it is ignored
        memory_map.write(0x8000, 0x34).unwrap();
        assert_eq!(memory_map.read(0x8000).unwrap(), 0x00);
    }

}