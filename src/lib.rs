use std::cmp::Ordering;
use std::fmt::Display;
use crate::twine::Twine;

pub mod twine;

#[derive(Clone, Debug)]
pub struct Strey {
    prefix: Option<Box<Strey>>,
    hash1: u64,
    hash2: u64,
    string: Twine,
}

impl Strey {
    pub fn new(prefix: Option<Box<Strey>>, string: Twine) -> Strey {
        let (hash1, hash2) = calculate_hashes(&prefix, &string);
        Strey { prefix, hash1, hash2, string }
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

const HASH_MUL: u64 = 19;

fn calculate_hashes(prefix: &Option<Box<Strey>>, string: &Twine) -> (u64, u64) {
    let (mut hash1, mut hash2) =
        match prefix {
            None => { (0u64, 0u64) }
            Some(prefix) => { (prefix.hash1, prefix.hash2) }
        };
    for byte in string.as_str().as_bytes() {
        hash1 = hash1.wrapping_mul(HASH_MUL).wrapping_add(*byte as u64);
        hash2 = hash2.wrapping_mul(HASH_MUL).wrapping_add(*byte as u64).wrapping_add(1);
    }
    (hash1, hash2)
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
        self.hash1 == other.hash1 && self.hash2 == other.hash2
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