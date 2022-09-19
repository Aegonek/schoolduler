use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub trait HashCode: Hash {
    fn hash_code(&self) -> u64;
} 

impl<T: Hash> HashCode for T {
    fn hash_code(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}