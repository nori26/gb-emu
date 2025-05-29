use crate::cpu::ProgramCounter;
use crate::cpu::instruction_set::Readable;
use crate::cpu::operands::Addressable;
use crate::cpu::operands::Imm8;
use crate::peripheral::Peripheral;
use std::rc::Rc;

pub struct Imm16<PC>
where
    PC: ProgramCounter<Value = u16>,
{
    step: u8,
    data: u16,
    imm8: Imm8<PC>,
}

impl<PC> Imm16<PC>
where
    PC: ProgramCounter<Value = u16>,
{
    pub fn new(pc: Rc<PC>, bus: Rc<Peripheral>) -> Self {
        Self {
            step: 0,
            data: 0,
            imm8: Imm8::new(pc, bus),
        }
    }
}

impl<PC> Readable<u16> for Imm16<PC>
where
    PC: ProgramCounter<Value = u16>,
{
    fn read(&mut self) -> Option<u16> {
        const STEP_MAX: u8 = 2;

        while self.step < STEP_MAX {
            let byte = self.imm8.read()?;

            match self.step {
                0 => self.data = (byte as u16) << u8::BITS,
                1 => self.data |= byte as u16,
                _ => unreachable!(),
            };
            self.step += 1;
        }
        self.step = 0;
        Some(self.data)
    }
}

impl<PC> Addressable for Imm16<PC> where PC: ProgramCounter<Value = u16> {}
