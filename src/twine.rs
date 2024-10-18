use std::cmp::Ordering;
use std::sync::Arc;

#[derive(Clone)]
pub enum Twine {
    Owned(Arc<String>),
    Borrowed(&'static str),
}

impl Twine {
    pub fn new(s: String) -> Self {
        Twine::Owned(Arc::new(s))
    }
    pub const fn new_str(s: &'static str) -> Self {
        Twine::Borrowed(s)
    }
    pub fn as_str(&self) -> &str {
        match self {
            Twine::Owned(s) => s.as_str(),
            Twine::Borrowed(s) => s
        }
    }
    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
    pub fn len(&self) -> usize {
        match self {
            Twine::Owned(s) => s.len(),
            Twine::Borrowed(s) => s.len()
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            Twine::Owned(s) => { s.is_empty() }
            Twine::Borrowed(s) => { s.is_empty() }
        }
    }
    pub fn bytes(&self) -> std::str::Bytes {
        match self {
            Twine::Owned(s) => { s.bytes() }
            Twine::Borrowed(s) => { s.bytes() }
        }
    }
    pub fn chars(&self) -> std::str::Chars {
        match self {
            Twine::Owned(s) => { s.chars() }
            Twine::Borrowed(s) => { s.chars() }
        }
    }
}

impl From<String> for Twine {
    fn from(s: String) -> Self {
        Twine::new(s)
    }
}

impl From<&'static str> for Twine {
    fn from(s: &'static str) -> Self {
        Twine::new_str(s)
    }
}

impl From<Arc<String>> for Twine {
    fn from(s: Arc<String>) -> Self {
        Twine::Owned(s)
    }
}

impl From<&Arc<String>> for Twine {
    fn from(s: &Arc<String>) -> Self {
        Twine::Owned(s.clone())
    }
}

impl From<&String> for Twine {
    fn from(s: &String) -> Self {
        Twine::Owned(Arc::new(s.clone()))
    }
}

impl std::fmt::Display for Twine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl PartialEq for Twine {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for Twine {}

impl std::fmt::Debug for Twine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl PartialOrd<Self> for Twine {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Twine {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}