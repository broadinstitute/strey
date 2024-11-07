use std::fmt::Display;
use std::iter::Chain;

#[derive(Clone, Debug)]
pub enum Bytes<'a> {
    Simple(std::str::Bytes<'a>),
    Prefixed(Chain<Box<Bytes<'a>>, std::str::Bytes<'a>>)
}

#[derive(Clone, Debug)]
pub enum Chars<'a> {
    Simple(std::str::Chars<'a>),
    Prefixed(Chain<Box<Chars<'a>>, std::str::Chars<'a>>)
}


impl Iterator for Bytes<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Bytes::Simple(bytes) => bytes.next(),
            Bytes::Prefixed(chain) => chain.next()
        }
    }
}

impl Iterator for Chars<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Chars::Simple(chars) => chars.next(),
            Chars::Prefixed(chain) => chain.next()
        }
    }
}

impl Display for Chars<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for c in self.clone() {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}
