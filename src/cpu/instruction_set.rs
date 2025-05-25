pub mod ld;
pub mod nop;

pub use nop::Nop;

pub trait Readable<T> {
    fn read(&self) -> Option<T>;
}

pub trait Writable<T> {
    fn write(&self, val: T) -> Option<()>;
}
