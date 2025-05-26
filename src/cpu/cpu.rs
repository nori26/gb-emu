use crate::cpu::DummyInstruction;
use crate::cpu::Instruction;
use crate::cpu::ProgramCounter;
use crate::cpu::instruction_set::*;
use crate::cpu::register_file::RegisterFile;
use crate::peripheral::Peripheral;

pub struct Cpu {
    reg: RegisterFile,
    instruction: Box<dyn Instruction>,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            reg: RegisterFile::default(),
            instruction: Box::new(DummyInstruction::new()),
        }
    }

    pub fn emulate_cycle(&mut self, bus: &mut Peripheral) {
        if self.instruction.is_done() {
            let instr = self.fetch(bus);
            self.instruction = self.decode(instr);
        }
        self.instruction.exec();
    }

    fn fetch(&self, bus: &Peripheral) -> u8 {
        let addr = self.reg.pc().next();
        bus.read(addr)
    }

    fn decode(&self, instr: u8) -> Box<dyn Instruction> {
        match instr {
            0x00 => Box::new(Nop::new()),
            _ => panic!("Not implemented: {:02x}", instr),
        }
    }
}
