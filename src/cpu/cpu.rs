use crate::cpu::Instruction;
use crate::cpu::instruction_set::*;
use crate::cpu::registers::Registers;
use crate::peripheral::Peripheral;

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
        let mut instruction = self.decode(instr);
        instruction.exec();
    }

    fn fetch(&self, bus: &Peripheral) -> u8 {
        let addr = self.reg.pc().read().unwrap();
        let instr = bus.read(addr);
        self.reg.pc().inc();
        instr
    }

    fn decode(&self, instr: u8) -> Box<dyn Instruction> {
        match instr {
            0x00 => Box::new(Nop::new()),
            _ => panic!("Not implemented: {:02x}", instr),
        }
    }
}
