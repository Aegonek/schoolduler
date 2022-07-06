pub trait EagerIter: 
    IntoIterator<Item = <Self as EagerIter>::Item> 
    + FromIterator<<Self as EagerIter>::Item>
    + Sized {
    type Item;

    fn eager_map<F, T, Coll>(self, func: F) -> Coll
    where
        F: FnMut(<Self as EagerIter>::Item) -> T,
        Coll: FromIterator<T> 
    {
        self.into_iter().map(func).collect()
    }

    fn eager_filter<P>(self, pred: P) -> Self
    where
        P: FnMut(&<Self as EagerIter>::Item) -> bool
    {
        self.into_iter().filter(pred).collect()
    }

    fn eager_filter_map<F, T, Coll>(self, func: F) -> Coll
    where
        F: FnMut(<Self as EagerIter>::Item) -> Option<T>,
        Coll: FromIterator<T> 
    {
        self.into_iter().filter_map(func).collect()
    }
}

impl<T> EagerIter for Vec<T> 
{
    type Item = T;
}