use crate::cpu::ProgramCounter;
use crate::cpu::instruction_set::Readable;
use crate::peripheral::Peripheral;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Imm8<PC>
where
    PC: ProgramCounter<Value = u16>,
{
    is_fetch_phase: RefCell<bool>,
    tmp: RefCell<u8>,
    pc: Rc<PC>,
    bus: Rc<Peripheral>,
}

impl<PC> Imm8<PC>
where
    PC: ProgramCounter<Value = u16>,
{
    pub fn new(pc: Rc<PC>, bus: Rc<Peripheral>) -> Self {
        Self {
            is_fetch_phase: RefCell::new(true),
            tmp: RefCell::new(0),
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

    fn read(&self) -> Option<Self::Value> {
        let mut tmp = self.tmp.borrow_mut();
        let mut is_fetch_phase = self.is_fetch_phase.borrow_mut();

        let result = if *is_fetch_phase {
            *tmp = self.bus.read(self.pc.next());
            None
        } else {
            Some(*tmp)
        };
        *is_fetch_phase = !*is_fetch_phase;
        result
    }
}
