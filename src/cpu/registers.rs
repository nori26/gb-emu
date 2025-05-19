#[derive(Default, Debug)]
pub struct Registers {
    pc: u16,
}

impl Registers {
    pub fn pc(&self) -> u16 {
        self.pc
    }
}
