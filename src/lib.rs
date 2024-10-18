use crate::twine::Twine;
use std::cmp::Ordering;
use std::fmt::Display;

pub mod twine;

#[derive(Clone, Debug)]
pub enum Prefix {
    Borrowed(&'static Strey),
    Owned(Box<Strey>)
}

#[derive(Clone, Debug)]
pub struct Strey {
    prefix: Option<Prefix>,
    string: Twine,
}

impl Prefix {
    pub fn len(&self) -> usize {
        match self {
            Prefix::Borrowed(strey) => { strey.len() }
            Prefix::Owned(strey) => { strey.len() }
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            Prefix::Borrowed(strey) => { strey.is_empty() }
            Prefix::Owned(strey) => { strey.is_empty() }
        }
    }
    pub fn bytes(&self) -> Box<dyn Iterator<Item=u8> + '_> {
        match self {
            Prefix::Borrowed(strey) => { strey.bytes() }
            Prefix::Owned(strey) => { strey.bytes() }
        }
    }
    pub fn chars(&self) -> Box<dyn Iterator<Item=char> + '_> {
        match self {
            Prefix::Borrowed(strey) => { strey.chars() }
            Prefix::Owned(strey) => { strey.chars() }
        }
    }
}

impl Display for Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Prefix::Borrowed(strey) => { write!(f, "{}", strey)?; }
            Prefix::Owned(strey) => { write!(f, "{}", strey)?; }
        }
        Ok(())
    }
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
    pub fn bytes(&self) -> Box<dyn Iterator<Item=u8> + '_> {
        match &self.prefix {
            None => { Box::new(self.string.bytes()) }
            Some(prefix) => { Box::new(prefix.bytes().chain(self.string.bytes())) }
        }
    }
    pub fn chars(&self) -> Box<dyn Iterator<Item=char> + '_> {
        match &self.prefix {
            None => { Box::new(self.string.chars()) }
            Some(prefix) => { Box::new(prefix.chars().chain(self.string.chars())) }
        }
    }
    pub fn strip_prefix(&self, prefix: &Strey) -> Option<Box<dyn Iterator<Item=char> + '_>> {
        todo!()
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