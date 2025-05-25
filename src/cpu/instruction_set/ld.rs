use crate::cpu::Instruction;
use crate::cpu::instruction_set::{DstOperand, SrcOperand};

pub struct Ld<T, D, S>
where
    T: Copy,
    D: DstOperand<T>,
    S: SrcOperand<T>,
{
    step: u32,
    is_done: bool,
    loaded: Option<T>,
    dst: D,
    src: S,
}

impl<T, D, S> Ld<T, D, S>
where
    T: Copy,
    D: DstOperand<T>,
    S: SrcOperand<T>,
{
    pub fn new(dst: D, src: S) -> Self {
        Self {
            step: 0,
            is_done: false,
            loaded: None,
            dst: dst,
            src: src,
        }
    }

    fn next(&mut self) -> Option<()> {
        match self.step {
            0 => self.src.read().map(|val| self.loaded = Some(val)),
            1 => self.dst.write(self.loaded.unwrap()),
            2 => Some(()).inspect(|_| self.is_done = true),
            _ => None,
        }
        .map(|_| self.step += 1)
    }
}

impl<T, D, S> Instruction for Ld<T, D, S>
where
    T: Copy,
    D: DstOperand<T>,
    S: SrcOperand<T>,
{
    fn exec(&mut self) {
        if self.is_done() {
            panic!("No steps remaining.");
        }
        while let Some(_) = self.next() {}
    }

    fn is_done(&self) -> bool {
        self.is_done
    }
}
