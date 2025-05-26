use crate::cpu::instruction_set::{Readable, Writable};
use crate::cpu::operands::Register;
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

    fn write(&self, val: Self::Value) -> Option<()> {
        Some(()).inspect(|_| {
            self.reg.write(val);
        })
    }
}

impl<RegN> Readable for Reg<RegN>
where
    RegN: Register,
{
    type Value = RegN::Value;

    fn read(&self) -> Option<Self::Value> {
        Some(self.reg.read())
    }
}
