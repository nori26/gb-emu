use crate::cpu::instruction_set::{Writable, Readable};
use std::cell::RefCell;

#[derive(Default, Debug)]
pub struct Registers {
    pc: Reg16,
}

impl Registers {
    pub fn pc(&self) -> &Reg16 {
        &self.pc
    }
}

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

#[derive(Default, Debug)]
pub struct Reg16(RefCell<u16>);

impl Reg16 {
    pub fn inc(&self) {
        let new = self.read().unwrap().wrapping_add(1);
        self.write(new);
    }
}

impl Writable<u16> for Reg16 {
    fn write(&self, val: u16) -> Option<()> {
        Some(()).inspect(|_| *self.0.borrow_mut() = val)
    }
}

impl Readable<u16> for Reg16 {
    fn read(&self) -> Option<u16> {
        Some(*self.0.borrow())
    }
}
