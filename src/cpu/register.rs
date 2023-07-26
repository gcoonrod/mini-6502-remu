#[derive(Debug)]
pub struct ByteRegister {
    value: u8
}

impl ByteRegister {
    pub fn new() -> ByteRegister {
        ByteRegister {
            value: 0
        }
    }

    pub fn get(&self) -> u8 {
        self.value
    }

    pub fn set(&mut self, value: u8) {
        self.value = value;
    }
}

#[derive(Debug)]
pub struct WordRegister {
    value: u16
}

impl WordRegister {
    pub fn new() -> WordRegister {
        WordRegister {
            value: 0
        }
    }

    pub fn get(&self) -> u16 {
        self.value
    }

    pub fn set(&mut self, value: u16) {
        self.value = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_register() {
        let mut register = ByteRegister::new();
        assert_eq!(register.get(), 0);
        register.set(0x12);
        assert_eq!(register.get(), 0x12);
    }

    #[test]
    fn word_register() {
        let mut register = WordRegister::new();
        assert_eq!(register.get(), 0);
        register.set(0x1234);
        assert_eq!(register.get(), 0x1234);
    }
}