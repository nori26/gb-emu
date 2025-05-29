pub mod ld;
pub mod nop;

pub use nop::Nop;

pub trait Readable<T: Copy> {
    fn read(&mut self) -> Option<T>;
}

pub trait Writable<T: Copy> {
    fn write(&mut self, val: T) -> Option<()>;
}
