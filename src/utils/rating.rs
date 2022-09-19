// Compared by ratings. Greater ratings are better than smaller.

use std::{ops::{Mul, Div}, fmt::Display};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rating(u32);

impl Rating {
    pub const MIN: Rating = Rating(0);
    pub const MAX: Rating = Rating(1_000_000);

    pub const fn new(value: u32) -> Self {
        assert!(value >= Rating::MIN.0 && value <= Rating::MAX.0);
        Rating(value)
    }

    pub const fn value(&self) -> u32 {
        self.0
    }
}

impl Mul<u32> for Rating
{
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        let new = self.0.saturating_mul(rhs);
        Rating(new)
    }
}

impl Div<u32> for Rating {
    type Output = Self;

    fn div(self, rhs: u32) -> Self::Output {
        let new = self.0 / rhs;
        Rating(new)
    }
}

impl Mul<f64> for Rating
{
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let new = self.0 as f64 * rhs;
        if new > Self::MAX.0 as f64 {
            Self::MAX
        } else {
            Rating(new.round() as u32)
        }
    }
}

impl Div<f64> for Rating {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let new = self.0 as f64 / rhs;
        let new = new.clamp(Rating::MIN.0 as f64, Rating::MAX.0 as f64);
        Rating(new.round() as u32)
    }
}

impl From<Rating> for f64 {
    fn from(rating: Rating) -> Self {
        rating.0 as f64
    }
}

impl Display for Rating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone)]
pub struct Rated<T> {
    pub value: T,
    pub rating: Rating,
}

impl<T: Copy> Copy for Rated<T> {}

impl<T> Rated<T> {
    pub fn new(value: T, rating: Rating) -> Rated<T> {
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

unsafe impl<T: Send> Send for Rated<T> {}
