use crate::cpu::Register;
use crate::cpu::instruction_set::{Readable, Writable};
use crate::cpu::operands::reg8x2::Reg8x2;
use std::rc::Rc;

pub struct Reg8x2d<Reg8>
where
    Reg8: Register<Value = u8>,
{
    reg: Reg8x2<Reg8>,
}

impl<Reg8> Reg8x2d<Reg8>
where
    Reg8: Register<Value = u8>,
{
    pub fn new(high: Rc<Reg8>, low: Rc<Reg8>) -> Self {
        let reg = Reg8x2::new(high, low);
        Self { reg }
    }
}

impl<Reg8> Writable<u16> for Reg8x2d<Reg8>
where
    Reg8: Register<Value = u8>,
{
    fn write(&mut self, val: u16) -> Option<()> {
        self.reg.write(val)
    }
}

impl<Reg8> Readable<u16> for Reg8x2d<Reg8>
where
    Reg8: Register<Value = u8>,
{
    fn read(&mut self) -> Option<u16> {
        self.reg.read().inspect(|val| {
            self.write(val.wrapping_sub(1));
        })
    }
}
