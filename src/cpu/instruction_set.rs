pub mod ld;
pub mod nop;

pub use nop::Nop;

type Dst<T> = Box<dyn DstOperand<T>>;
type Src<T> = Box<dyn SrcOperand<T>>;

pub trait DstOperand<T> {
	fn write(&self, val: T) -> Option<()>;
}

pub trait SrcOperand<T> {
	fn read(&self) -> Option<T>;
}
