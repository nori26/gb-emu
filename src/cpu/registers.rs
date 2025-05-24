use std::cell::RefCell;
use crate::cpu::instruction_set::{DstOperand, SrcOperand};

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
pub struct Reg8 {
    value: RefCell<u8>,
}

impl DstOperand<u8> for Reg8 {
    fn write(&self, val: u8) -> Option<()> {
        Some(()).inspect(|_| *self.value.borrow_mut() = val)
    }    
}

impl SrcOperand<u8> for Reg8 {
    fn read(&self) -> Option<u8> {
        Some(*self.value.borrow())
    }
}

#[derive(Default, Debug)]
pub struct Reg16 {
    value: RefCell<u16>,
}

impl Reg16 {
    pub fn read(&self) -> u16 {
        *self.value.borrow()
    }

    pub fn write(&self, val: u16) {
        *self.value.borrow_mut() = val;
    }

    pub fn inc(&self) {
        let new = self.read().wrapping_add(1);
        self.write(new);
    }
}
