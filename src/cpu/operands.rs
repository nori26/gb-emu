pub trait Register<T: Copy> {
    fn read(&self) -> T;
    fn write(&self, val: T);
}
