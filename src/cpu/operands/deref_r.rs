use crate::cpu::instruction_set::Readable;
use crate::peripheral::Peripheral;
use std::rc::Rc;

pub struct DerefR<T>
where
    T: Readable<u16>,
{
    addressable: T,
    bus: Rc<Peripheral>,
    step: u8,
    addr: u16,
    data: u8,
}

impl<T> DerefR<T>
where
    T: Readable<u16>,
{
    pub fn new(addressable: T, bus: Rc<Peripheral>) -> Self {
        Self {
            addressable,
            bus,
            step: 0,
            addr: 0,
            data: 0,
        }
    }
}

impl<T> Readable<u8> for DerefR<T>
where
    T: Readable<u16>,
{
    fn read(&mut self) -> Option<u8> {
        match self.step {
            0 => {
                self.addr = self.addressable.read()?;
                self.step += 1;
                None
            }
            1 => {
                self.data = self.bus.read(self.addr);
                self.step += 1;
                None
            }
            2 => {
                self.step = 0;
                Some(self.data)
            }
            _ => unreachable!(),
        }
    }
}
