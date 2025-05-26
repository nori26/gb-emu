use crate::cpu::registers::Reg16;

#[derive(Default, Debug)]
pub struct RegisterFile {
    pc: Reg16,
}

impl RegisterFile {
    pub fn pc(&self) -> &Reg16 {
        &self.pc
    }
}
