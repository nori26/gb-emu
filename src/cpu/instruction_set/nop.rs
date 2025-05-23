use crate::cpu::Instruction;

pub struct Nop {
    step: u32,
}

impl Nop {
    pub fn new() -> Self {
        Self {
            step: 0,
        }
    }
}

impl Instruction for Nop {
    fn exec(&mut self) {
        if self.step != 0 {
            panic!("No steps remaining.");
        }
        self.step += 1;
    }

    fn is_done(&self) -> bool {
        true
    }
}
