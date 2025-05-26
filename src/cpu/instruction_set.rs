pub mod ld;
pub mod nop;

pub use nop::Nop;

pub trait Readable {
	type Value: Copy;
    fn read(&self) -> Option<Self::Value>;
}

pub trait Writable {
	type Value: Copy;
    fn write(&self, val: Self::Value) -> Option<()>;
}
