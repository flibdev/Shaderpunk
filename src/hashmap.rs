use std::hash::{BuildHasher, Hasher};

use castaway::cast;
use fnv_rs::{FnvHasher, Fnv64};

// Impl hashmap based on FNV1a64 hash with option to add pre-hashed keys

pub struct PassThruHasher {}

impl BuildHasher for PassThruHasher {
    type Hasher = Fnv64;

    fn build_hasher(&self) -> Self::Hasher {
        Fnv64::new()
    }

    fn hash_one<T: std::hash::Hash>(&self, x: T) -> u64
        where
            Self: Sized,
            Self::Hasher: std::hash::Hasher,
            T: 
    {
        if let Ok(hash) = cast!(&x, u64) {
            hash
        }
        else {
            let mut hasher = self.build_hasher();
            x.hash(&mut hasher);
            hasher.finish()
        }
    }
}
