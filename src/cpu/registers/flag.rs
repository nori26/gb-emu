use crate::cpu::Register;
use std::cell::RefCell;

#[derive(Default, Debug)]
pub struct Flag(RefCell<u8>);

impl Register<u8> for Flag {
    fn load(&self) -> u8 {
        *self.0.borrow()
    }

    fn store(&self, val: u8) {
        *self.0.borrow_mut() = val & 0xF0;
    }
}
