use crate::cpu;
use crate::cpu::Register;
use std::cell::RefCell;

#[derive(Default, Debug)]
pub struct ProgramCounter(RefCell<u16>);

impl cpu::ProgramCounter for ProgramCounter {
    type Value = u16;
    fn next(&self) -> Self::Value {
        let old = self.load();
        self.store(old.wrapping_add(1));
        old
    }
}

impl Register for ProgramCounter {
    type Value = u16;

    fn load(&self) -> Self::Value {
        *self.0.borrow()
    }

    fn store(&self, val: Self::Value) {
        *self.0.borrow_mut() = val;
    }
}
