use crate::cpu::register::*;

#[derive(Debug)]
pub struct CPU {
    x: ByteRegister,
    y: ByteRegister,
    a: ByteRegister,
    pc: WordRegister,
    sp: ByteRegister,
    flags: ByteRegister
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            x: ByteRegister::new(),
            y: ByteRegister::new(),
            a: ByteRegister::new(),
            pc: WordRegister::new(),
            sp: ByteRegister::new(),
            flags: ByteRegister::new()
        }
    }

    pub fn reset(&mut self) {
        self.x.set(0);
        self.y.set(0);
        self.a.set(0);
        self.pc.set(0);
        self.sp.set(0);
        self.flags.set(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu() {
        let mut cpu = CPU::new();
        assert_eq!(cpu.x.get(), 0);
        assert_eq!(cpu.y.get(), 0);
        assert_eq!(cpu.a.get(), 0);
        assert_eq!(cpu.pc.get(), 0);
        assert_eq!(cpu.sp.get(), 0);
        assert_eq!(cpu.flags.get(), 0);
        cpu.x.set(0x12);
        cpu.y.set(0x34);
        cpu.a.set(0x56);
        cpu.pc.set(0x1234);
        cpu.sp.set(0x78);
        cpu.flags.set(0x9A);
        assert_eq!(cpu.x.get(), 0x12);
        assert_eq!(cpu.y.get(), 0x34);
        assert_eq!(cpu.a.get(), 0x56);
        assert_eq!(cpu.pc.get(), 0x1234);
        assert_eq!(cpu.sp.get(), 0x78);
        assert_eq!(cpu.flags.get(), 0x9A);
    }

    #[test]
    fn cpu_reset() {
        let mut cpu = CPU::new();
        cpu.x.set(0x12);
        cpu.y.set(0x34);
        cpu.a.set(0x56);
        cpu.pc.set(0x1234);
        cpu.sp.set(0x78);
        cpu.flags.set(0x9A);
        cpu.reset();
        assert_eq!(cpu.x.get(), 0);
        assert_eq!(cpu.y.get(), 0);
        assert_eq!(cpu.a.get(), 0);
        assert_eq!(cpu.pc.get(), 0);
        assert_eq!(cpu.sp.get(), 0);
        assert_eq!(cpu.flags.get(), 0);
    }
}