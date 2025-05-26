pub mod cpu;
mod instruction_set;
mod register_file;
mod registers;

pub use cpu::Cpu;

pub trait Instruction {
    fn exec(&mut self);
    fn is_done(&self) -> bool;
}
