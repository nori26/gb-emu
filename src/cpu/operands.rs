pub mod deref_r;
pub mod deref_w;
pub mod fixed_hi;
pub mod imm16;
pub mod imm8;
pub mod reg;
pub mod reg8x2;
pub mod reg8x2d;
pub mod reg8x2i;

pub use imm8::Imm8;

trait Addressable {}
