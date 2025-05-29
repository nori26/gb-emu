use crate::cpu::Register;
use crate::cpu::instruction_set::Readable;
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
            val: (HIGH as u16) << u8::BITS | low.load() as u16,
            _pd: PhantomData,
        }
    }
}

impl<const HIGH: u8, LOW> Readable<u16> for FixedHi<HIGH, LOW>
where
    LOW: Register<Value = u8>,
{
    fn read(&mut self) -> Option<u16> {
        Some(self.val)
    }
}
