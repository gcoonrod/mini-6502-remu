// Module: main
use crate::devices::memory_map::*;
use crate::devices::memory::*;

pub mod cpu;
pub mod devices;
pub mod emulator;

fn main() {
    
    // Create a MemoryMap and add the RAM and ROM to it
    let mut memory_map = MemoryMap::new();
    memory_map.create("RAM".to_string(), MemoryType::RAM, 0x4000, 0x0000).unwrap();
    memory_map.create(String::from("IO"), MemoryType::MMIO, 0x4000, 0x4000).unwrap();
    memory_map.create("ROM".to_string(), MemoryType::ROM, 0x8000, 0x8000).unwrap();

    // Print the MemoryMap
    memory_map.print_table();

}
