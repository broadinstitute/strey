use std::iter::Chain;

pub enum Bytes<'a> {
    Simple(std::str::Bytes<'a>),
    Prefixed(Chain<Box<Bytes<'a>>, std::str::Bytes<'a>>)
}

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
