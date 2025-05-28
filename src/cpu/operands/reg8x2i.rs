use crate::cpu::Register;
use crate::cpu::instruction_set::{Readable, Writable};
use crate::cpu::operands::Addressable;
use crate::cpu::operands::reg8x2::Reg8x2;
use std::rc::Rc;

pub struct Reg8x2i<Reg8>
where
    Reg8: Register<Value = u8>,
{
    reg: Reg8x2<Reg8>,
}

impl<Reg8> Reg8x2i<Reg8>
where
    Reg8: Register<Value = u8>,
{
    pub fn new(high: Rc<Reg8>, low: Rc<Reg8>) -> Self {
        let reg = Reg8x2::new(high, low);
        Self { reg }
    }
}

impl<Reg8> Writable for Reg8x2i<Reg8>
where
    Reg8: Register<Value = u8>,
{
    type Value = u16;

    fn write(&self, val: u16) -> Option<()> {
        self.reg.write(val)
    }
}

impl<Reg8> Readable for Reg8x2i<Reg8>
where
    Reg8: Register<Value = u8>,
{
    type Value = u16;

    fn read(&self) -> Option<Self::Value> {
        self.reg.read().inspect(|val| {
            self.write(val.wrapping_add(1));
        })
    }
}

impl<Reg8> Addressable for Reg8x2i<Reg8> where Reg8: Register<Value = u8> {}
