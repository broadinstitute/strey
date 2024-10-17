use crate::Strey;
use crate::twine::Twine;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Hashes {
    hash1: u64,
    hash2: u64,
}

impl Hashes {
    pub(crate) fn zero() -> Hashes {
        Hashes { hash1: 0, hash2: 0 }
    }
}

const HASH_MUL: u64 = 19;

pub(crate) fn calculate_hashes(prefix: &Option<Box<Strey>>, string: &Twine) -> Hashes {
    let mut hashes =
        match prefix {
            None => { Hashes::zero() }
            Some(prefix) => { prefix.get_hashes().clone() }
        };
    for byte in string.as_str().as_bytes() {
        hashes.hash1 = hashes.hash1.wrapping_mul(HASH_MUL).wrapping_add(*byte as u64);
        hashes.hash2 = hashes.hash2.wrapping_mul(HASH_MUL).wrapping_add(*byte as u64).wrapping_add(1);
    }
    hashes
}

