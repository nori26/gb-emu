use crate::cpu::instruction_set::{Readable, Writable};
use crate::cpu::operands::Addressable;
use crate::cpu::operands::Register;
use std::rc::Rc;

pub struct Reg8x2<Reg8>
where
    Reg8: Register<Value = u8>,
{
    high: Rc<Reg8>,
    low: Rc<Reg8>,
}

impl<Reg8> Reg8x2<Reg8>
where
    Reg8: Register<Value = u8>,
{
    pub fn new(high: Rc<Reg8>, low: Rc<Reg8>) -> Self {
        Self {
            high: high,
            low: low,
        }
    }
}

impl<Reg8> Writable for Reg8x2<Reg8>
where
    Reg8: Register<Value = u8>,
{
    type Value = u16;

    fn write(&self, val: u16) -> Option<()> {
        Some(()).inspect(|_| {
            let low = (val & 0xFF) as u8;
            let high = (val >> u8::BITS) as u8;
            self.low.write(low);
            self.high.write(high);
        })
    }
}

impl<Reg8> Readable for Reg8x2<Reg8>
where
    Reg8: Register<Value = u8>,
{
    type Value = u16;

    fn read(&self) -> Option<u16> {
        let low = self.low.read() as u16;
        let high = self.high.read() as u16;
        Some(high << u8::BITS | low)
    }
}

impl<Reg8> Addressable for Reg8x2<Reg8> where Reg8: Register<Value = u8> {}
