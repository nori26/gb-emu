pub mod cpu;
mod dummy_instruction;
mod instruction_set;
mod operands;
mod register_file;
mod registers;

pub use cpu::Cpu;
pub use dummy_instruction::DummyInstruction;

pub trait Instruction {
    fn exec(&mut self);
    fn is_done(&self) -> bool;
}

pub trait Register {
    type Value: Copy;
    fn load(&self) -> Self::Value;
    fn store(&self, val: Self::Value);
}

pub trait ProgramCounter<T: Copy> {
    fn next(&self) -> T;
}
