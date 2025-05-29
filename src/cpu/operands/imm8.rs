use crate::cpu::ProgramCounter;
use crate::cpu::instruction_set::Readable;
use crate::peripheral::Peripheral;
use std::rc::Rc;

pub struct Imm8<PC>
where
    PC: ProgramCounter<Value = u16>,
{
    is_fetch_phase: bool,
    tmp: u8,
    pc: Rc<PC>,
    bus: Rc<Peripheral>,
}

impl<PC> Imm8<PC>
where
    PC: ProgramCounter<Value = u16>,
{
    pub fn new(pc: Rc<PC>, bus: Rc<Peripheral>) -> Self {
        Self {
            is_fetch_phase: true,
            tmp: 0,
            pc,
            bus,
        }
    }
}

impl<PC> Readable for Imm8<PC>
where
    PC: ProgramCounter<Value = u16>,
{
    type Value = u8;

    fn read(&mut self) -> Option<Self::Value> {
        let result = if self.is_fetch_phase {
            self.tmp = self.bus.read(self.pc.next());
            None
        } else {
            Some(self.tmp)
        };
        self.is_fetch_phase = !self.is_fetch_phase;
        result
    }
}
