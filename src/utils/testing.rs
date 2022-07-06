pub type Ok = ();

pub struct Case<T, U> where U: PartialEq {
    pub payload: T,
    pub expected: U
}