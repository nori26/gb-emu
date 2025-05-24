use crate::cpu::Instruction;
use crate::cpu::instruction_set::{Dst, Src};

pub struct Ld<T: Copy> {
    step: u32,
    is_done: bool,
    loaded: Option<T>,
    dst: Dst<T>,
    src: Src<T>,
}

impl<T: Copy> Ld<T> {
    pub fn new(dst: Dst<T>, src: Src<T>) -> Self {
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

impl<T: Copy> Instruction for Ld<T> {
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
