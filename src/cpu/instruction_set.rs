pub mod nop;
pub use nop::Nop;

pub trait DstOperand<T> {
	fn write(&self, val: T) -> Option<()>;
}

pub trait SrcOperand<T> {
	fn read(&self) -> Option<T>;
}
