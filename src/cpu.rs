pub mod cpu;
mod instruction_set;
mod register_file;
mod registers;

pub use cpu::Cpu;

pub trait Instruction {
    fn exec(&mut self);
    fn is_done(&self) -> bool;
}

pub trait Register<T: Copy> {
    fn load(&self) -> T;
    fn store(&self, val: T);
}

pub trait ProgramCounter<T: Copy> {
    fn next(&self) -> T;
}
