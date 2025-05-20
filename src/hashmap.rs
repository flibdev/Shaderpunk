use std::hash::{ BuildHasherDefault, Hash, Hasher};

use hashbrown::HashMap;
use paste::paste;

use crate::types::cname::CName;

macro_rules! hash_format {
    (32) => { "[{:08X}]" };
    (64) => { "[{:016X}]" };
}

macro_rules! create_passthru {
    ($x:literal) => { paste! {
        pub struct [<PassThruHasher $x>]([<u $x>]);

        impl Default for [<PassThruHasher $x>] {
            fn default() -> Self {
                Self(0)
            }
        }

        impl Hasher for [<PassThruHasher $x>] {
            fn finish(&self) -> u64 {
                self.0 as u64
            }
        
            fn write(&mut self, _: &[u8]) {
                panic!("[<PassThruHasher $x>] only accepts [<u $x>] keys")
            }
        
            fn [<write_u $x>](&mut self, i: [<u $x>]) {
                self.0 = i
            }
        }

        #[derive(Clone)]
        pub struct [<CNameKey $x>] {
            pub name: Option<CName>,
            pub hash: [<u $x>],
        }

        impl From<CName> for [<CNameKey $x>] {
            fn from(value: CName) -> Self {
                Self {
                    name: Some(value.clone()),
                    hash: value.[<as_hash $x>]()
                }
            }
        }

        impl From<[<u $x>]> for [<CNameKey $x>] {
            fn from(value: [<u $x>]) -> Self {
                Self {
                    name: None,
                    hash: value
                }
            }
        }

        impl Hash for [<CNameKey $x>] {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.hash.hash(state);
            }
        }

        impl PartialEq for [<CNameKey $x>] {
            fn eq(&self, other: &Self) -> bool {
                self.hash == other.hash
            }
        }
        impl Eq for [<CNameKey $x>] {}

        impl std::fmt::Display for [<CNameKey $x>] {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match &self.name {
                    Some(n) => write!(f, concat!(hash_format!($x), " \"{}\""), self.hash, n),
                    None => write!(f, hash_format!($x), self.hash),
                }
            }
        }

        pub type [<CNameHashMap $x>]<T> = HashMap<[<CNameKey $x>], T, BuildHasherDefault<[<PassThruHasher $x>]>>;
    }};
}

create_passthru!(32);
create_passthru!(64);

