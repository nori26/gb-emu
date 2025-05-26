use crate::cpu::instruction_set::{Writable, Readable};
use std::cell::RefCell;

#[derive(Default, Debug)]
pub struct Reg8(RefCell<u8>);

impl Writable<u8> for Reg8 {
    fn write(&self, val: u8) -> Option<()> {
        Some(()).inspect(|_| *self.0.borrow_mut() = val)
    }
}

impl Readable<u8> for Reg8 {
    fn read(&self) -> Option<u8> {
        Some(*self.0.borrow())
    }
}
