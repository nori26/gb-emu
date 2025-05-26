use crate::cpu::instruction_set::{Readable, Writable};
use crate::cpu::operands::Register;

pub struct Reg8x2<'a, Reg8>
where
    Reg8: Register<u8>,
{
    high: &'a Reg8,
    low: &'a Reg8,
}

impl<'a, Reg8> Reg8x2<'a, Reg8>
where
    Reg8: Register<u8>,
{
    pub fn new(high: &'a Reg8, low: &'a Reg8) -> Self {
        Self {
            high: high,
            low: low,
        }
    }
}

impl<'a, Reg8> Writable<u16> for Reg8x2<'a, Reg8>
where
    Reg8: Register<u8>,
{
    fn write(&self, val: u16) -> Option<()> {
        Some(()).inspect(|_| {
            let low = (val & 0xFF) as u8;
            let high = (val >> u8::BITS) as u8;
            self.low.write(low);
            self.high.write(high);
        })
    }
}

impl<'a, Reg8> Readable<u16> for Reg8x2<'a, Reg8>
where
    Reg8: Register<u8>,
{
    fn read(&self) -> Option<u16> {
        let low = self.low.read() as u16;
        let high = self.high.read() as u16;
        Some(high << u8::BITS | low)
    }
}
