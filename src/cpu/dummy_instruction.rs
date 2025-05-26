use crate::cpu::Instruction;

pub struct DummyInstruction;

impl DummyInstruction {
    pub fn new() -> Self {
        Self
    }
}

impl Instruction for DummyInstruction {
    fn exec(&mut self) {
        panic!("Cannot exec");
    }

    fn is_done(&self) -> bool {
        true
    }
}
