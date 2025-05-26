use crate::cpu::Instruction;
use crate::cpu::instruction_set::{Readable, Writable};

pub struct Ld<D, S>
where
    D: Writable,
    S: Readable<Value = D::Value>,
{
    step: u32,
    is_done: bool,
    loaded: Option<D::Value>,
    dst: D,
    src: S,
}

impl<D, S> Ld<D, S>
where
    D: Writable,
    S: Readable<Value = D::Value>,
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

impl<D, S> Instruction for Ld<D, S>
where
    D: Writable,
    S: Readable<Value = D::Value>,
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
