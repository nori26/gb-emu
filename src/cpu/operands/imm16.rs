use crate::cpu::ProgramCounter;
use crate::cpu::instruction_set::Readable;
use crate::cpu::operands::Addressable;
use crate::cpu::operands::Imm8;
use crate::peripheral::Peripheral;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Imm16<PC>
where
    PC: ProgramCounter<Value = u16>,
{
    step: RefCell<u8>,
    data: RefCell<u16>,
    imm8: Imm8<PC>,
}

impl<PC> Imm16<PC>
where
    PC: ProgramCounter<Value = u16>,
{
    pub fn new(pc: Rc<PC>, bus: Rc<Peripheral>) -> Self {
        Self {
            step: RefCell::new(0),
            data: RefCell::new(0),
            imm8: Imm8::new(pc, bus),
        }
    }
}

impl<PC> Readable for Imm16<PC>
where
    PC: ProgramCounter<Value = u16>,
{
    type Value = u16;

    fn read(&self) -> Option<Self::Value> {
        const STEP_MAX: u8 = 2;
        let mut data = self.data.borrow_mut();
        let mut step = self.step.borrow_mut();

        while *step < STEP_MAX {
            let byte = self.imm8.read()?;

            match *step {
                0 => *data = (byte as u16) << u8::BITS,
                1 => *data |= byte as u16,
                _ => unreachable!(),
            };
            *step += 1;
        }
        *step = 0;
        Some(*data)
    }
}

impl<PC> Addressable for Imm16<PC> where PC: ProgramCounter<Value = u16> {}
