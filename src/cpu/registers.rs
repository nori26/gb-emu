pub mod flag;
pub mod gpr;
pub mod pc;

pub use pc::ProgramCounter;

use crate::cpu;
use crate::cpu::operands;

impl<REG> operands::Register for REG
where
    REG: cpu::Register,
{
    type Value = REG::Value;

    fn read(&self) -> Self::Value {
        self.load()
    }

    fn write(&self, val: Self::Value) {
        self.store(val);
    }
}
