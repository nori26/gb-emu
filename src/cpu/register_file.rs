use crate::cpu::registers::*;

#[derive(Default, Debug)]
pub struct RegisterFile {
    pc: ProgramCounter,
}

impl RegisterFile {
    pub fn pc(&self) -> &ProgramCounter {
        &self.pc
    }
}
