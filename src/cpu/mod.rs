mod registers;

use registers::Registers;

pub struct Cpu {
    reg: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            reg: Registers::default(),
        }
    }
}
