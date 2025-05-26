use crate::cpu::ProgramCounter as IProgramCounter;
use crate::cpu::registers::*;

#[derive(Default, Debug)]
pub struct RegisterFile {
    pc: ProgramCounter,
}

impl RegisterFile {
    pub fn pc(&self) -> &impl IProgramCounter<Value = u16> {
        &self.pc
    }
}
