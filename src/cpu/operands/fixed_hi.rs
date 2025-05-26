use crate::cpu::instruction_set::Readable;
use crate::cpu::operands::Register;
use std::marker::PhantomData;
use std::rc::Rc;

pub struct FixedHi<const HIGH: u8, LOW>
where
    LOW: Register<Value = u8>,
{
    val: u16,
    _pd: PhantomData<LOW>,
}

impl<const HIGH: u8, LOW> FixedHi<HIGH, LOW>
where
    LOW: Register<Value = u8>,
{
    pub fn new(low: Rc<LOW>) -> Self {
        Self {
            val: (HIGH as u16) << u8::BITS | low.read() as u16,
            _pd: PhantomData,
        }
    }
}

impl<const HIGH: u8, LOW> Readable for FixedHi<HIGH, LOW>
where
    LOW: Register<Value = u8>,
{
    type Value = u16;

    fn read(&self) -> Option<Self::Value> {
        Some(self.val)
    }
}
