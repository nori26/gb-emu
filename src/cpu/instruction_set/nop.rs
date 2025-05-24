use crate::cpu::Instruction;

pub struct Nop {
    is_done: bool,
}

impl Nop {
    pub fn new() -> Self {
        Self { is_done: false }
    }
}

impl Instruction for Nop {
    fn exec(&mut self) {
        if self.is_done() {
            panic!("No steps remaining.");
        }
        self.is_done = true;
    }

    fn is_done(&self) -> bool {
        self.is_done
    }
}
