pub mod fixed_hi;
pub mod reg;
pub mod reg8x2;


pub trait Register {
    type Value: Copy;
    fn read(&self) -> Self::Value;
    fn write(&self, val: Self::Value);
}
