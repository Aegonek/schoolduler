use std::error::Error;

pub fn custom(msg: &str) -> Box<dyn Error> {
    String::from(msg).into()
}