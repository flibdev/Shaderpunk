
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

pub trait Linkable<T> {
    fn as_link(self: &Self) -> Link;
}

#[derive(Clone, Default, PartialEq, Eq, Ord, Serialize, Deserialize)]
pub struct Link {
    pub name: String,
    pub path: PathBuf,
}

impl Link {
    pub fn get_template() -> String {
        String::from("[{{ name }}]({{ path }})")
    }
}

impl PartialOrd for Link {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.name.cmp(&other.name)
            .then(self.path.cmp(&other.path))
        )
    }
}

pub type LinkList = Vec<Link>;
