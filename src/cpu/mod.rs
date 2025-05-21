mod instructions;
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

    pub fn emulate_cycle(&mut self, bus: &mut Peripheral) {
        let instr = self.fetch(bus);
    }

    fn fetch(&self, bus: &Peripheral) -> u8 {
        let addr = self.reg.pc().read();
        let instr = bus.read(addr);
        self.reg.pc().inc();
        instr
    }

    fn decode(&self, instr: u8) {
        match instr {
            _ => panic!("Not implemented: {:02x}", instr),
        }
    }
}
