// Compared by ratings.

#[derive(Clone)]
pub struct Rated<T> { pub value: T, pub rating: u32 }

impl<T: Copy> Copy for Rated<T> {}

impl<T> Rated<T> {
    pub fn new(value: T, rating: u32) -> Rated<T> {
        Rated { value, rating }
    }
}

impl<T> PartialEq for Rated<T> {
    fn eq(&self, other: &Self) -> bool {
        self.rating == other.rating
    }
}
impl<T> Eq for Rated<T> {}

impl<T> PartialOrd for Rated<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.rating.partial_cmp(&other.rating)
    }
}

impl<T> Ord for Rated<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}