use crate::cpu::Register;
use std::cell::RefCell;

#[derive(Default, Debug)]
pub struct Gpr(RefCell<u8>);

impl Register for Gpr {
    type Value = u8;

    fn load(&self) -> Self::Value {
        *self.0.borrow()
    }

    fn store(&self, val: Self::Value) {
        *self.0.borrow_mut() = val;
    }
}
