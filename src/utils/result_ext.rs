pub trait ResultExt {
    type Error;

    fn void(self) -> Result<(), Self::Error>;
}

impl<T, E> ResultExt for Result<T, E> {
    type Error = E;

    fn void(self) -> Result<(), Self::Error> {
        self.map(|_| ())
    }
}