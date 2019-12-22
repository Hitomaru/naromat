pub trait DocumentElement {
    fn get(self) -> String;
    fn print(self);
}