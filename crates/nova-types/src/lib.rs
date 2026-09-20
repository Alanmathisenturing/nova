use sha2::{Digest as ShaDigest, Sha256};
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EventId(pub u64);
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Sequence(pub u64);
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StateRoot(pub [u8; 32]);
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Digest(pub [u8; 32]);

impl Digest {
    pub fn of(domain: &[u8], bytes: &[u8]) -> Self {
        let mut h = Sha256::new();
        h.update(domain.len().to_le_bytes());
        h.update(domain);
        h.update(bytes);
        Self(h.finalize().into())
    }
    pub fn hex(self) -> String { self.0.iter().map(|b| format!("{b:02x}")).collect() }
}
impl StateRoot { pub fn hex(self) -> String { Digest(self.0).hex() } }
impl fmt::Display for Digest { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.hex()) } }
impl fmt::Display for StateRoot { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.hex()) } }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SchemaVersion(pub u16);
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct LogicalTime(pub u64);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Provenance { pub origin: String, pub trace_id: String }

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TypeError { EmptyOrigin, InvalidTraceId }
