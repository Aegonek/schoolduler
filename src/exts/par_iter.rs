use std::{
    convert::{TryFrom, TryInto},
    error::Error,
};

use rayon::prelude::ParallelIterator;

use crate::utils;

pub trait ParallelTryCollect<A>
where
    Self: ParallelIterator<Item = A>,
    A: Send,
{
    fn try_collect<Container>(self) -> Result<Container, Box<dyn Error>>
    where
        Container: TryFrom<Vec<A>>;
}

impl<T, A> ParallelTryCollect<A> for T
where
    T: ParallelIterator<Item = A>,
    A: Send,
{
    fn try_collect<Container>(self) -> Result<Container, Box<dyn Error>>
    where
        Vec<A>: TryInto<Container>,
    {
        self.collect::<Vec<_>>()
            .try_into()
            .map_err(|_| utils::error::custom("Invalid array bounds!"))
    }
}