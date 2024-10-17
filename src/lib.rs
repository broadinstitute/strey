use std::cmp::Ordering;
use std::fmt::Display;
use std::sync::OnceLock;
use crate::hashes::{calculate_hashes, Hashes};
use crate::twine::Twine;

pub mod twine;
mod hashes;

#[derive(Clone, Debug)]
pub struct Strey {
    prefix: Option<Box<Strey>>,
    string: Twine,
    hash_lock: OnceLock<Hashes>,
}

impl Strey {
    pub const fn new(prefix: Option<Box<Strey>>, string: Twine) -> Strey {
        let hash_lock = OnceLock::new();
        Strey { prefix, string, hash_lock }
    }

    pub fn new_string(string: String) -> Strey {
        Strey::new(None, Twine::new(string))
    }

    pub const fn new_str(string: &'static str) -> Strey {
        Strey::new(None, Twine::new_str(string))
    }

    fn get_hashes(&self) -> &Hashes {
        self.hash_lock.get_or_init(|| calculate_hashes(&self.prefix, &self.string))
    }
    pub fn len(&self) -> usize {
        match &self.prefix {
            None => { self.string.len() }
            Some(prefix) => { prefix.len() + self.string.len() }
        }
    }
    pub fn is_empty(&self) -> bool {
        match &self.prefix {
            None => { self.string.is_empty() }
            Some(prefix) => { prefix.is_empty() && self.string.is_empty() }
        }
    }
    pub fn bytes(&self) -> Box<dyn Iterator<Item=u8> + '_> {
        match &self.prefix {
            None => { Box::new(self.string.bytes()) }
            Some(prefix) => { Box::new(prefix.bytes().chain(self.string.bytes())) }
        }
    }
    pub fn append(&self, string: String) -> Strey {
        Strey::new(Some(Box::new(self.clone())), string.into())
    }
    pub fn append_str(&self, string: &'static str) -> Strey {
        Strey::new(Some(Box::new(self.clone())), string.into())
    }
}

impl Display for Strey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.prefix {
            None => { write!(f, "{}", self.string)?; }
            Some(prefix) => {
                write!(f, "{}{}", prefix, self.string)?;
            }
        }
        Ok(())
    }
}

impl PartialEq for Strey {
    fn eq(&self, other: &Self) -> bool {
        self.get_hashes() == other.get_hashes()
    }
}

impl Eq for Strey {}

impl PartialOrd<Self> for Strey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Strey {
    fn cmp(&self, other: &Self) -> Ordering {
        for (bs, bo) in self.bytes().zip(other.bytes()) {
            match bs.cmp(&bo) {
                Ordering::Equal => { continue; }
                ord => { return ord; }
            }
        }
        Ordering::Equal
    }
}

impl From<String> for Strey {
    fn from(string: String) -> Self {
        Strey::new(None, string.into())
    }
}

impl From<&'static str> for Strey {
    fn from(string: &'static str) -> Self {
        Strey::new(None, string.into())
    }
}