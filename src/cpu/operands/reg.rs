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

impl<RegN> Writable for Reg<RegN>
where
    RegN: Register,
{
    type Value = RegN::Value;

    fn write(&mut self, val: Self::Value) -> Option<()> {
        Some(()).inspect(|_| {
            self.reg.store(val);
        })
    }
}

impl<RegN> Readable for Reg<RegN>
where
    RegN: Register,
{
    type Value = RegN::Value;

    fn read(&mut self) -> Option<Self::Value> {
        Some(self.reg.load())
    }
}
