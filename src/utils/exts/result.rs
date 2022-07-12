pub trait ResultExt {
    type Ok;
    type Error;

    fn void(self) -> Result<(), Self::Error>;
}

impl<T, E> ResultExt for Result<T, E> {
    type Ok = T;
    type Error = E;

    fn void(self) -> Result<(), Self::Error> {
        self.map(|_| ())
    }
}