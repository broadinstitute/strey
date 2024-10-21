use std::fmt::Display;
use crate::{iter, Strey};

#[derive(Clone, Debug)]
pub enum Prefix {
    Borrowed(&'static Strey),
    Owned(Box<Strey>)
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
    pub fn bytes(&self) -> iter::Bytes {
        match self {
            Prefix::Borrowed(strey) => { strey.bytes() }
            Prefix::Owned(strey) => { strey.bytes() }
        }
    }
    pub fn chars(&self) -> iter::Chars {
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