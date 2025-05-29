use crate::cpu::Register;
use crate::cpu::instruction_set::{Readable, Writable};
use crate::cpu::operands::Addressable;
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

impl<Reg8> Writable<u16> for Reg8x2<Reg8>
where
    Reg8: Register<Value = u8>,
{
    fn write(&mut self, val: u16) -> Option<()> {
        Some(()).inspect(|_| {
            let low = (val & 0xFF) as u8;
            let high = (val >> u8::BITS) as u8;
            self.low.store(low);
            self.high.store(high);
        })
    }
}

impl<Reg8> Readable<u16> for Reg8x2<Reg8>
where
    Reg8: Register<Value = u8>,
{
    fn read(&mut self) -> Option<u16> {
        let low = self.low.load() as u16;
        let high = self.high.load() as u16;
        Some(high << u8::BITS | low)
    }
}

impl<Reg8> Addressable for Reg8x2<Reg8> where Reg8: Register<Value = u8> {}
