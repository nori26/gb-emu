mod registers;

use crate::peripheral::Peripheral;
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

    fn fetch(&self, bus: &Peripheral) -> u8 {
        let addr = self.reg.pc().read();
        let instr = bus.read(addr);
        self.reg.pc().inc();
        instr
    }
}
