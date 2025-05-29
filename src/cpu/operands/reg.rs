use crate::cpu::Register;
use crate::cpu::instruction_set::{Readable, Writable};
use std::rc::Rc;

pub struct Reg<RegN>
where
    RegN: Register,
{
    reg: Rc<RegN>,
}

impl<RegN> Reg<RegN>
where
    RegN: Register,
{
    pub fn new(v: Rc<RegN>) -> Self {
        Self { reg: v }
    }
}

impl<RegN> Writable<RegN::Value> for Reg<RegN>
where
    RegN: Register,
{
    fn write(&mut self, val: RegN::Value) -> Option<()> {
        Some(()).inspect(|_| {
            self.reg.store(val);
        })
    }
}

impl<RegN> Readable<RegN::Value> for Reg<RegN>
where
    RegN: Register,
{
    fn read(&mut self) -> Option<RegN::Value> {
        Some(self.reg.load())
    }
}
