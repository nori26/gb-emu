use crate::cpu;
use crate::cpu::Register;
use std::cell::RefCell;

#[derive(Default, Debug)]
pub struct ProgramCounter(RefCell<u16>);

impl cpu::ProgramCounter<u16> for ProgramCounter {
    fn next(&self) -> u16 {
        let old = self.load();
        self.store(old.wrapping_add(1));
        old
    }
}

impl Register<u16> for ProgramCounter {
    fn load(&self) -> u16 {
        *self.0.borrow()
    }

    fn store(&self, val: u16) {
        *self.0.borrow_mut() = val;
    }
}
