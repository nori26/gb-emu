pub mod cpu;
mod instruction_set;
mod registers;

pub use cpu::Cpu;

pub trait Instruction {
    fn exec(&mut self);
    fn is_done(&self) -> bool;
}
