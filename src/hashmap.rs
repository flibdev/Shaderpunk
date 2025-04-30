use std::hash::{ BuildHasherDefault, Hash, Hasher};

use hashbrown::HashMap;

use crate::types::cname::CName;


pub struct PassThruHasher32(u32);

impl PassThruHasher32 {
    #[must_use]
    pub fn new() -> Self {
        Self(0)
    }
}

impl Default for PassThruHasher32 {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher for PassThruHasher32 {
    fn finish(&self) -> u64 {
        self.0 as u64
    }

    fn write(&mut self, _: &[u8]) {
        panic!("PassThruHasher32 only accepts u32 keys")
    }

    fn write_u32(&mut self, i: u32) {
        self.0 = i
    }
}


pub struct CNameKey32 {
    cname: Option<CName>,
    hash: u32
}

impl CNameKey32 {
    pub fn from_cname(cname: CName) -> Self {
        Self {
            cname: Some(cname.clone()),
            hash: cname.as_hash32().unwrap()
        }
    }
    pub fn from_hash(hash: u32) -> Self {
        Self {
            cname: None,
            hash
        }
    }
}

impl From<u32> for CNameKey32 {
    fn from(value: u32) -> Self {
        CNameKey32::from_hash(value)
    }
}

impl From<CName> for CNameKey32 {
    fn from(value: CName) -> Self {
        CNameKey32::from_cname(value)
    }
}

impl std::fmt::Display for CNameKey32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.cname {
            Some(n) => write!(f, "[{:08x}] \"{}\"", self.hash, n),
            None => write!(f, "[{:08x}]", self.hash),
        }
    }
}

impl Hash for CNameKey32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl PartialEq for CNameKey32 {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}
impl Eq for CNameKey32 {}

pub type CNameHashMap32<T> = HashMap<CNameKey32, T, BuildHasherDefault<PassThruHasher32>>;
