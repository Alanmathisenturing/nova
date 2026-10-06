use crate::JournalError;
use nova_types::{Digest, StateRoot};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommitRecord {
    pub event: Digest,
    pub state_root: StateRoot,
}

impl CommitRecord {
    pub fn encode(self) -> [u8; 64] {
        let mut out = [0u8; 64];
        out[..32].copy_from_slice(&self.event.0);
        out[32..].copy_from_slice(&self.state_root.0);
        out
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, JournalError> {
        if bytes.len() != 64 {
            return Err(JournalError::InvalidFormat("invalid commit record length"));
        }
        let mut event = [0u8; 32];
        let mut root = [0u8; 32];
        event.copy_from_slice(&bytes[..32]);
        root.copy_from_slice(&bytes[32..]);
        Ok(Self {
            event: Digest(event),
            state_root: StateRoot(root),
        })
    }
}
