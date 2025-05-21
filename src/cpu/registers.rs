use std::cell::RefCell;

#[derive(Default, Debug)]
pub struct Registers {
    pc: u16,
}

impl Registers {
    pub fn pc(&self) -> u16 {
        self.pc
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
