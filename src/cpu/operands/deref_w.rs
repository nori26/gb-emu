use crate::cpu::instruction_set::{Readable, Writable};
use crate::peripheral::Peripheral;
use std::rc::Rc;

pub struct DerefW<T>
where
    T: Readable<u16>,
{
    addressable: T,
    bus: Rc<Peripheral>,
    step: u8,
    addr: u16,
}

impl<T> DerefW<T>
where
    T: Readable<u16>,
{
    pub fn new(addressable: T, bus: Rc<Peripheral>) -> Self {
        Self {
            addressable,
            bus,
            step: 0,
            addr: 0,
        }
    }
}

impl<T> Writable<u8> for DerefW<T>
where
    T: Readable<u16>,
{
    fn write(&mut self, val: u8) -> Option<()> {
        match self.step {
            0 => {
                self.addr = self.addressable.read()?;
                self.step += 1;
                None
            }
            1 => {
                self.bus.write(self.addr, val);
                self.step += 1;
                None
            }
            2 => {
                self.step = 0;
                Some(())
            }
            _ => unreachable!(),
        }
    }
}

impl<T> Writable<u16> for DerefW<T>
where
    T: Readable<u16>,
{
    fn write(&mut self, val: u16) -> Option<()> {
        match self.step {
            0 => {
                self.addr = self.addressable.read()?;
                self.step += 1;
                None
            }
            1 => {
                self.bus.write(self.addr, val as u8);
                self.step += 1;
                None
            }
            2 => {
                let next = self.addr.wrapping_add(1);
                self.bus.write(next, (val >> u8::BITS) as u8);
                self.step += 1;
                None
            }
            3 => {
                self.step = 0;
                Some(())
            }
            _ => unreachable!(),
        }
    }
}
