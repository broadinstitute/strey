use crate::twine::Twine;
use std::cmp::Ordering;
use std::fmt::Display;
use prefix::Prefix;

pub mod twine;
mod prefix;
pub mod iter;

#[derive(Clone, Debug)]
pub struct Strey {
    prefix: Option<Prefix>,
    string: Twine,
}

impl Strey {
    pub const fn new(prefix: Option<Prefix>, string: Twine) -> Strey {
        Strey { prefix, string }
    }

    pub fn new_string(string: String) -> Strey {
        Strey::new(None, Twine::new(string))
    }

    pub const fn new_str(string: &'static str) -> Strey {
        Strey::new(None, Twine::new_str(string))
    }

    pub fn append(&self, string: String) -> Strey {
        Strey::new(Some(Prefix::Owned(Box::new(self.clone()))), string.into())
    }
    pub fn append_str(&self, string: &'static str) -> Strey {
        Strey::new(Some(Prefix::Owned(Box::new(self.clone()))), string.into())
    }
    pub fn join(&'static self, string: String) -> Strey {
        Strey::new(Some(Prefix::Borrowed(self)), string.into())
    }
    pub const fn join_str(&'static self, string: &'static str) -> Strey {
        Strey::new(Some(Prefix::Borrowed(self)), Twine::Borrowed(string))
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
    pub fn bytes(&self) -> iter::Bytes {
        match &self.prefix {
            None => { iter::Bytes::Simple(self.string.bytes()) }
            Some(prefix) => {
                iter::Bytes::Prefixed(Box::new(prefix.bytes()).chain(self.string.bytes()))
            }
        }
    }
    pub fn chars(&self) -> iter::Chars {
        match &self.prefix {
            None => { iter::Chars::Simple(self.string.chars()) }
            Some(prefix) => {
                iter::Chars::Prefixed(Box::new(prefix.chars()).chain(self.string.chars()))
            }
        }
    }
    pub fn strip_prefix(&self, prefix: &Strey) -> Option<iter::Chars> {
        let mut self_chars = self.chars();
        let mut prefix_chars = prefix.chars();
        loop {
            match prefix_chars.next() {
                None => { break Some(self_chars); }
                Some(cp) => {
                    match self_chars.next() {
                        None => { break None; }
                        Some(cs) => {
                            if cp != cs {
                                break None;
                            }
                        }
                    }
                }
            }
        }
    }
    pub fn maybe_use_as_prefix_for(&self, strey: Strey) -> Strey {
        match strey.strip_prefix(self) {
            None => { strey }
            Some(suffix) => {
                Strey::new(Some(Prefix::from(self.clone())), suffix.collect())
            }
        }
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
        for (bs, bo) in self.bytes().zip(other.bytes()) {
            if bs != bo {
                return false;
            }
        }
        true
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